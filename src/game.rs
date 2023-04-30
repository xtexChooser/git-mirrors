use anyhow::{bail, Result};
use thirtyfour::{By, Key, WebDriver};
use tracing::{info, trace};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSide {
    Left,
    Right,
}

impl ToString for PlayerSide {
    fn to_string(&self) -> String {
        (match self {
            Self::Left => "L",
            Self::Right => "R",
        })
        .to_string()
    }
}

pub trait GameInterface {
    async fn is_ready(&self) -> Result<bool>;
    async fn is_finished(&self) -> Result<bool>;
    async fn start(&self) -> Result<()>;
    async fn click_left(&self) -> Result<()>;
    async fn click_right(&self) -> Result<()>;
    async fn pull_incoming(&self) -> Result<Vec<PlayerSide>>;
}

impl GameInterface for WebDriver {
    async fn is_ready(&self) -> Result<bool> {
        Ok(self
            .find(By::Id("page_wrap"))
            .await?
            .class_name()
            .await?
            .unwrap_or_default()
            .contains("ready"))
    }

    async fn is_finished(&self) -> Result<bool> {
        Ok(!self
            .find(By::Id("page_wrap"))
            .await?
            .class_name()
            .await?
            .unwrap_or_default()
            .contains("in_game"))
    }

    async fn start(&self) -> Result<()> {
        if !self.is_finished().await? {
            bail!("game not started yet")
        }
        info!("start game");
        self.find(By::ClassName("button")).await?.click().await?;
        Ok(())
    }

    async fn click_left(&self) -> Result<()> {
        trace!("click left");
        self.action_chain()
            .send_keys(Key::Left.to_string())
            .perform()
            .await?;
        Ok(())
    }

    async fn click_right(&self) -> Result<()> {
        trace!("click right");
        self.action_chain()
            .send_keys(Key::Right.to_string())
            .perform()
            .await?;
        Ok(())
    }

    /// `da` rules:
    /// <0: right
    /// >=0: left
    async fn pull_incoming(&self) -> Result<Vec<PlayerSide>> {
        let da: Vec<i32> = self
            .execute("return window.ljs_da;", vec![])
            .await?
            .convert()?;
        let mut side = vec![];
        for val in da.into_iter() {
            side.push(if val >= 0 {
                PlayerSide::Left
            } else {
                PlayerSide::Right
            });
        }
        info!(side = format!("{:?}", side), "pulled incoming safe sides");
        Ok(side)
    }
}
