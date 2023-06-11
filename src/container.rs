use std::{cmp, collections::HashMap, env};

use anyhow::{Context, Result};
use podman_api::{
    api::Containers,
    models::InspectContainerData,
    opts::{
        ContainerCreateOpts, ContainerCreateOptsBuilder, ContainerDeleteOpts, ContainerListFilter,
        ContainerListOpts,
    },
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    constant::{ENV_PURGE_RUNNING_CTRS, LABEL_NO_PURGE, LABEL_NO_PURGE_VAL},
    direct_into_build,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct ContainerResources {
    #[serde(default)]
    pub created: Vec<ContainerCreated>,
    #[serde(default)]
    pub removed: Vec<ContainerRemoved>,
}

impl ContainerResources {
    pub async fn apply(&self, api: &Containers) -> Result<()> {
        for created in &self.created {
            let remote = api.get(&created.name);
            if remote.exists().await? {
                if created == &remote.inspect().await? {
                    continue;
                } else {
                    remote.delete(&Default::default()).await?;
                    info!(
                        name = created.name,
                        "deleted exists container for not matching"
                    )
                }
            }
            let resp = api
                .create(&created.clone().into())
                .await
                .with_context(|| format!("target container: {}", &created.name))?;
            info!(
                name = created.name,
                response = serde_json::to_string(&resp)?,
                "created container"
            );
        }
        for removed in &self.removed {
            let remote = api.get(&removed.name);
            if remote.exists().await? {
                let force = removed.force.unwrap_or(false);
                if force {
                    remote.remove().await?;
                } else {
                    remote
                        .delete(&ContainerDeleteOpts::builder().force(true).build())
                        .await?;
                }
                info!(name = removed.name, force, "deleted exists container");
            }
        }
        Ok(())
    }

    pub async fn purge(&self, api: &Containers) -> Result<()> {
        // todo: skip podrc container
        let container = api
            .list(
                &ContainerListOpts::builder()
                    .filter([ContainerListFilter::NoLabelKeyVal(
                        LABEL_NO_PURGE.to_string(),
                        LABEL_NO_PURGE_VAL.to_string(),
                    )])
                    .build(),
            )
            .await?;
        let managed = self
            .created
            .iter()
            .map(|f| f.name.to_owned())
            .collect::<Vec<_>>();
        for container in container {
            let id = container.id.unwrap();
            if !managed.contains(&id)
                && !container
                    .names
                    .unwrap_or_default()
                    .iter()
                    .any(|f| managed.contains(f))
            {
                let remote = api.get(&id);
                if remote
                    .inspect()
                    .await?
                    .state
                    .map(|f| f.running)
                    .flatten()
                    .unwrap_or_default()
                    && env::var(ENV_PURGE_RUNNING_CTRS).unwrap() != "true"
                {
                    continue;
                }
                remote.delete(&Default::default()).await?;
                info!(id, "purged container");
            }
        }
        Ok(())
    }

    pub fn merge(self, new: &mut Self) {
        new.created.extend(self.created);
        new.removed.extend(self.removed);
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct ContainerCreated {
    #[serde(default)]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(default)]
    pub apparmor_profile: String,
    #[serde(default)]
    pub add_capabilities: Vec<String>,
    #[serde(default)]
    pub drop_capabilities: Vec<String>,
    #[serde(default)]
    pub cgroup_parent: Option<String>,
    #[serde(default)]
    pub cgroup_mode: Option<String>,
    #[serde(default)]
    pub chroot_directories: Vec<String>,
    #[serde(default)]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    pub common_pid_file: Option<String>,
    #[serde(default)]
    pub cpu_period: u64,
    #[serde(default)]
    pub cpu_quota: i64,
    #[serde(default = "value_true")]
    pub create_working_dir: bool,
    #[serde(default)]
    pub dependency_containers: Vec<String>,
    #[serde(default)]
    pub dns_option: Vec<String>,
    #[serde(default)]
    pub dns_search: Vec<String>,
    #[serde(default)]
    pub dns_server: Vec<String>,
    #[serde(default)]
    pub entrypoint: Option<Vec<String>>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub env_host: bool,
    #[serde(default)]
    pub env_merge: Vec<String>,
    #[serde(default)]
    pub groups: Vec<String>,
    #[serde(default)]
    pub hosts_add: Vec<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub hostusers: Vec<String>,
    #[serde(default = "value_true")]
    pub http_proxy: bool,
    pub image: String,
    #[serde(default)]
    pub image_arch: Option<String>,
    #[serde(default)]
    pub image_os: Option<String>,
    #[serde(default)]
    pub image_variant: Option<String>,
    #[serde(default)]
    pub init: bool,
    #[serde(default)]
    pub init_container_type: Option<String>,
    #[serde(default)]
    pub init_path: Option<String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub mask: Option<Vec<String>>,
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub network_options: Option<HashMap<String, String>>,
    #[serde(default)]
    pub networks: HashMap<String, String>,
    #[serde(default)]
    pub no_new_privilages: bool,
    #[serde(default)]
    pub oci_runtime: Option<String>,
    #[serde(default)]
    pub oom_score_adj: Option<i64>,
    #[serde(default)]
    pub passwd_entry: Option<String>,
    #[serde(default)]
    pub pod: Option<String>,
    #[serde(default)]
    pub privileged: bool,
    #[serde(default)]
    pub procfs_opts: Option<Vec<String>>,
    #[serde(default)]
    pub publish_image_ports: bool,
    #[serde(default)]
    pub raw_image_name: Option<String>,
    #[serde(default)]
    pub read_only_fs: bool,
    #[serde(default)]
    pub remove: bool,
    #[serde(default)]
    pub restart_tries: u64,
    #[serde(default)]
    pub rootfs: String,
    #[serde(default)]
    pub rootfs_overlay: bool,
    #[serde(default)]
    pub rootfs_propagation: Option<String>,
    #[serde(default)]
    pub secret_env: HashMap<String, String>,
    #[serde(default)]
    pub selinux_opts: Vec<String>,
    #[serde(default)]
    pub shm_size: Option<i64>,
    #[serde(default)]
    pub stdin: bool,
    #[serde(default)]
    pub stop_signal: Option<i64>,
    #[serde(default)]
    pub stop_timeout: Option<u64>,
    #[serde(default)]
    pub storage_opts: HashMap<String, String>,
    #[serde(default)]
    pub sysctl: HashMap<String, String>,
    #[serde(default)]
    pub terminal: bool,
    #[serde(default)]
    pub throttle_read_bps_device: HashMap<String, String>,
    #[serde(default)]
    pub throttle_read_iops_device: HashMap<String, String>,
    #[serde(default)]
    pub throttle_write_bps_device: HashMap<String, String>,
    #[serde(default)]
    pub throttle_write_iops_device: HashMap<String, String>,
    #[serde(default)]
    pub timeout: u64,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub umask: Option<String>,
    #[serde(default)]
    pub unified: HashMap<String, String>,
    #[serde(default)]
    pub unmask: Vec<String>,
    #[serde(default)]
    pub unset_env: Vec<String>,
    #[serde(default)]
    pub unset_env_all: bool,
    #[serde(default)]
    pub use_image_hosts: bool,
    #[serde(default)]
    pub use_image_resolv_conf: bool,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub volatile: bool,
    #[serde(default)]
    pub work_dir: Option<String>,
}

fn value_true() -> bool {
    true
}

impl Into<ContainerCreateOptsBuilder> for ContainerCreated {
    fn into(self) -> ContainerCreateOptsBuilder {
        let mut builder = ContainerCreateOpts::builder()
            .apparmor_profile(self.apparmor_profile)
            .add_capabilities(self.add_capabilities)
            .drop_capabilities(self.drop_capabilities)
            .chroot_directories(self.chroot_directories)
            .cpu_period(self.cpu_period)
            .cpu_quota(self.cpu_quota)
            .create_working_dir(self.create_working_dir)
            .dependency_containers(self.dependency_containers)
            .dns_option(self.dns_option)
            .dns_search(self.dns_search)
            .dns_server(self.dns_server)
            .env(self.env)
            .env_host(self.env_host)
            .envmerge(self.env_merge)
            .groups(self.groups)
            .hosts_add(self.hosts_add)
            .hostusers(self.hostusers)
            .http_proxy(self.http_proxy)
            .image(self.image)
            .init(self.init)
            .labels(self.labels)
            .name(self.name)
            .namespace(self.namespace)
            .networks(self.networks)
            .no_new_privilages(self.no_new_privilages)
            .privileged(self.privileged)
            .publish_image_ports(self.publish_image_ports)
            .read_only_fs(self.read_only_fs)
            .remove(self.remove)
            .restart_tries(self.restart_tries)
            .rootfs(self.rootfs)
            .rootfs_overlay(self.rootfs_overlay)
            .secret_env(self.secret_env)
            .selinux_opts(self.selinux_opts)
            .stdin(self.stdin)
            .storage_opts(self.storage_opts)
            .sysctl(self.sysctl)
            .terminal(self.terminal)
            .throttle_read_bps_device(self.throttle_read_bps_device)
            .throttle_read_iops_device(self.throttle_read_iops_device)
            .throttle_write_bps_device(self.throttle_write_bps_device)
            .throttle_write_iops_device(self.throttle_write_iops_device)
            .timeout(self.timeout)
            .unified(self.unified)
            // todo: https://github.com/containers/podman/pull/18849
            //.unmask(self.unmask)
            .unset_env(self.unset_env)
            .unset_env_all(self.unset_env_all)
            .use_image_hosts(self.use_image_hosts)
            .use_image_resolv_conf(self.use_image_resolv_conf)
            .volatile(self.volatile);
        // todo: https://github.com/containers/podman/pull/18849
        if !self.unmask.is_empty() {
            builder = builder.unmask(self.unmask);
        }
        if let Some(value) = self.annotations {
            builder = builder.annotations(value);
        }
        if let Some(value) = self.cgroup_parent {
            builder = builder.cgroup_parent(value);
        }
        if let Some(value) = self.cgroup_mode {
            builder = builder.cgroup_mode(value);
        }
        if let Some(value) = self.command {
            builder = builder.command(value);
        }
        if let Some(value) = self.common_pid_file {
            builder = builder.common_pid_file(value);
        }
        if let Some(value) = self.entrypoint {
            builder = builder.entrypoint(value);
        }
        if let Some(value) = self.hostname {
            builder = builder.hostname(value);
        }
        if let Some(value) = self.image_arch {
            builder = builder.image_arch(value);
        }
        if let Some(value) = self.image_os {
            builder = builder.image_os(value);
        }
        if let Some(value) = self.image_variant {
            builder = builder.image_variant(value);
        }
        if let Some(value) = self.init_container_type {
            builder = builder.init_container_type(value);
        }
        if let Some(value) = self.init_path {
            builder = builder.init_path(value);
        }
        if let Some(value) = self.mask {
            builder = builder.mask(value);
        }
        if let Some(value) = self.network_options {
            builder = builder.network_options(value);
        }
        if let Some(value) = self.oci_runtime {
            builder = builder.oci_runtime(value);
        }
        if let Some(value) = self.oom_score_adj {
            builder = builder.oom_score_adj(value);
        }
        if let Some(value) = self.passwd_entry {
            builder = builder.passwd_entry(value);
        }
        if let Some(value) = self.pod {
            builder = builder.pod(value);
        }
        if let Some(value) = self.procfs_opts {
            builder = builder.procfs_opts(value);
        }
        if let Some(value) = self.raw_image_name {
            builder = builder.raw_image_name(value);
        }
        if let Some(value) = self.rootfs_propagation {
            builder = builder.rootfs_propagation(value);
        }
        if let Some(value) = self.shm_size {
            builder = builder.shm_size(value);
        }
        if let Some(value) = self.stop_signal {
            builder = builder.stop_signal(value);
        }
        if let Some(value) = self.stop_timeout {
            builder = builder.stop_timeout(value);
        }
        if let Some(value) = self.timezone {
            builder = builder.timezone(value);
        }
        if let Some(value) = self.umask {
            builder = builder.umask(value);
        }
        if let Some(value) = self.user {
            builder = builder.user(value);
        }
        if let Some(value) = self.work_dir {
            builder = builder.work_dir(value);
        }
        builder
    }
}

direct_into_build!(ContainerCreated, ContainerCreateOptsBuilder => ContainerCreateOpts);

impl cmp::PartialEq<InspectContainerData> for ContainerCreated {
    fn eq(&self, other: &InspectContainerData) -> bool {
        let other = other.to_owned();
        let config = other.config.unwrap();
        self.name == other.name.unwrap() && self.labels == config.labels.unwrap_or_default()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct ContainerRemoved {
    pub name: String,
    #[serde(default)]
    pub force: Option<bool>,
}
