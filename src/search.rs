use std::{fs::DirEntry, path::PathBuf, sync::Arc};

use anyhow::Result;
use async_recursion::async_recursion;
use parking_lot::Mutex;
use tokio::{sync::oneshot, task::JoinSet};

use crate::db::{self, info::CacheTypeRef};

pub async fn search(
    base: PathBuf,
    worker_count: usize,
    queue_depth: usize,
) -> Result<Vec<(PathBuf, CacheTypeRef)>> {
    let ctx = Arc::new(Context {
        busy_workers: Mutex::new(worker_count),
        balancer: Mutex::default(),
        result: tokio::sync::Mutex::default(),
    });

    let mut handles = JoinSet::new();

    for i in 0..worker_count {
        let ctx = ctx.clone();
        let base = base.clone();
        handles.spawn(async move {
            let mut worker = WorkerContext::new(queue_depth).unwrap();
            if i == 0 {
                worker.queue[0].push(base);
            }
            loop {
                worker.run_search(&ctx).await.unwrap();
                if !worker.pull_job(&ctx).await.unwrap() {
                    break;
                }
            }
        });
    }

    while let Some(_) = handles.join_next().await {}

    let result = ctx.result.lock().await;
    Ok(result.clone())
}

#[derive(Debug)]
struct Context {
    busy_workers: Mutex<usize>,
    balancer: Mutex<Vec<oneshot::Sender<Option<(usize, PathBuf)>>>>,
    result: tokio::sync::Mutex<Vec<(PathBuf, CacheTypeRef)>>,
}

unsafe impl Send for Context {}

impl Context {
    async fn check_entry(&self, entry: &DirEntry) -> Result<()> {
        let path = entry.path();
        if let Some((cache, path)) = db::check_path(&path).await? {
            self.result.lock().await.push((path, cache));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct WorkerContext {
    queue: Box<[Vec<PathBuf>]>,
}

impl WorkerContext {
    fn new(queue_depth: usize) -> Result<WorkerContext> {
        let queue = Box::try_new_zeroed_slice(queue_depth)?;
        let mut queue = unsafe { queue.assume_init() };
        queue.fill_with(Vec::new);

        Ok(WorkerContext { queue: queue })
    }

    async fn pull_job(&mut self, ctx: &Arc<Context>) -> Result<bool> {
        {
            let mut workers = ctx.busy_workers.lock();
            *workers -= 1;
            if *workers == 0 {
                while let Some(tx) = ctx.balancer.lock().pop() {
                    tx.send(None).unwrap();
                }
                return Ok(false);
            }
        }

        let (tx, rx) = oneshot::channel::<Option<(usize, PathBuf)>>();
        ctx.balancer.lock().push(tx);

        if let Some((depth, path)) = rx.await? {
            self.queue[depth].push(path);
        } else {
            return Ok(false);
        }

        *ctx.busy_workers.lock() += 1;
        Ok(true)
    }

    async fn share_job(&mut self, ctx: &Arc<Context>) -> Result<()> {
        if let Some(mut balancer) = ctx.balancer.try_lock() {
            while let Some(sender) = balancer.pop() {
                let mut data = None;
                for depth in 0..self.queue.len() - 1 {
                    if self.queue[depth].len() > 2 {
                        let path = self.queue[depth].pop().unwrap();
                        //println!("sharing {} {}", depth, path.display());
                        data = Some((depth, path));
                        break;
                    }
                }
                match data {
                    Some(data) => sender.send(Some(data)).unwrap(),
                    None => {
                        balancer.push(sender);
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    async fn run_search(&mut self, ctx: &Arc<Context>) -> Result<()> {
        let mut depth = self.queue.len() - 1;
        let mut recursive_timer = 0u32;
        loop {
            if let Some(path) = self.queue[depth].pop() {
                // current depth has queued path
                let children = self.do_search(&ctx, path).await?;
                if depth + 1 < self.queue.len() {
                    if !children.is_empty() {
                        depth += 1;
                        self.queue[depth] = children;
                        self.share_job(ctx).await?;
                    }
                } else {
                    for child in children {
                        self.do_recursive_search(&ctx, &child, &mut recursive_timer)
                            .await?;
                    }
                }
            } else {
                // no queued path, pop to upper layer
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
        }
        Ok(())
    }

    async fn do_search(&self, ctx: &Arc<Context>, path: PathBuf) -> Result<Vec<PathBuf>> {
        let mut queue = vec![];

        for entry in path.read_dir()? {
            let entry = entry?;
            ctx.check_entry(&entry).await?;
            if entry.metadata()?.is_dir() {
                queue.push(entry.path());
            }
        }

        Ok(queue)
    }

    #[async_recursion]
    async fn do_recursive_search(
        &mut self,
        ctx: &Arc<Context>,
        path: &PathBuf,
        timer: &mut u32,
    ) -> Result<()> {
        for entry in path.read_dir()? {
            let entry = entry?;
            ctx.check_entry(&entry).await?;
            if entry.metadata()?.is_dir() {
                *timer = timer.overflowing_add(1).0;
                if *timer as u8 == 0 {
                    self.share_job(ctx).await?;
                }
                self.do_recursive_search(ctx, &entry.path(), timer).await?;
            }
        }

        Ok(())
    }
}
