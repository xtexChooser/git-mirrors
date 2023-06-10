use std::{cmp, collections::HashMap};

use anyhow::{bail, Context, Result};
use podman_api::{
    api::Networks,
    models::{LeaseRange, Network, Subnet},
    opts::{NetworkCreateOpts, NetworkCreateOptsBuilder},
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct NetworkResources {
    #[serde(default)]
    pub created: Vec<NetworkCreated>,
    #[serde(default)]
    pub removed: Vec<NetworkRemoved>,
}

impl NetworkResources {
    pub async fn apply(&self, api: &Networks) -> Result<()> {
        for created in &self.created {
            let remote_net = api.get(&created.name);
            if remote_net.exists().await? {
                if created == &remote_net.inspect().await? {
                    continue;
                } else {
                    remote_net.delete().await?;
                    info!(
                        name = created.name,
                        "deleted exists network for not matching"
                    )
                }
            }
            let resp = api
                .create(&TryInto::<NetworkCreateOptsBuilder>::try_into(created.clone())?.build())
                .await
                .with_context(|| format!("target network: {}", &created.name))?;
            info!(
                name = created.name,
                response = serde_json::to_string(&resp)?,
                "created network"
            );
        }
        for removed in &self.removed {
            let net = api.get(&removed.name);
            if net.exists().await? {
                let force = removed.force.unwrap_or(false);
                if force {
                    net.remove().await?;
                } else {
                    net.delete().await?;
                }
                info!(name = removed.name, force, "deleted exists network");
            }
        }
        Ok(())
    }

    pub async fn purge(&self, api: &Networks) -> Result<()> {
        // todo
        Ok(())
    }

    pub fn merge(self, new: &mut Self) {
        new.created.extend(self.created);
        new.removed.extend(self.removed);
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct NetworkCreated {
    pub name: String,
    pub driver: String,
    #[serde(default)]
    pub options: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default = "value_true")]
    pub dns: bool,
    #[serde(default)]
    pub internal: bool,
    #[serde(default)]
    pub ipam_options: Option<HashMap<String, String>>,
    #[serde(default)]
    pub ipv6: bool,
    #[serde(default)]
    pub iface: Option<String>,
    #[serde(default)]
    pub subnets: Option<Vec<SubnetConfig>>,
}

fn value_true() -> bool {
    true
}

impl TryInto<NetworkCreateOptsBuilder> for NetworkCreated {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<NetworkCreateOptsBuilder, Self::Error> {
        let mut builder = NetworkCreateOpts::builder()
            .name(self.name)
            .driver(self.driver)
            .options(self.options)
            .labels(self.labels)
            .dns_enabled(self.dns)
            .internal(self.internal)
            .ipv6_enabled(self.ipv6);
        if let Some(value) = self.ipam_options {
            builder = builder.ipam_options(value);
        }
        if let Some(value) = self.iface {
            builder = builder.network_interface(value);
        }
        if let Some(value) = self.subnets {
            // todo: https://rust-lang.github.io/rfcs/3058-try-trait-v2.html
            builder = builder.subnets(
                value
                    .into_iter()
                    .map(Subnet::try_from)
                    .collect::<Result<Vec<Subnet>>>()?,
            );
        }
        Ok(builder)
    }
}

impl cmp::PartialEq<Network> for NetworkCreated {
    fn eq(&self, other: &Network) -> bool {
        let other = other.to_owned();
        if !(self.name == other.name.unwrap()
            && self.driver == other.driver.unwrap()
            && self.options == other.options.unwrap_or_default()
            && self.labels == other.labels.unwrap_or_default()
            && self.dns == other.dns_enabled.unwrap_or(false)
            && self.internal == other.internal.unwrap_or(false)
            && self.ipv6 == other.ipv_6_enabled.unwrap_or(false))
        {
            return false;
        }
        if let Some(value) = &self.ipam_options {
            if value != &other.ipam_options.unwrap_or_default() {
                return false;
            }
        }
        if let Some(value) = &self.iface {
            if value != &other.network_interface.unwrap_or_default() {
                return false;
            }
        }
        if let Some(value) = &self.subnets {
            // todo: https://rust-lang.github.io/rfcs/3058-try-trait-v2.html
            let mut value = value.to_owned();
            value.sort();
            let mut other = other
                .subnets
                .unwrap_or_default()
                .into_iter()
                .map(SubnetConfig::from)
                .collect::<Vec<_>>();
            other.sort();
            if value != other {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Default)]
pub struct SubnetConfig {
    #[serde(default)]
    pub subnet: Option<String>,
    #[serde(default)]
    pub gateway: Option<String>,
    #[serde(default)]
    pub lease_from: Option<String>,
    #[serde(default)]
    pub lease_to: Option<String>,
}

impl TryFrom<SubnetConfig> for Subnet {
    type Error = anyhow::Error;

    fn try_from(value: SubnetConfig) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            subnet: value.subnet,
            gateway: value.gateway,
            lease_range: if let Some(from) = value.lease_from {
                if let Some(to) = value.lease_to {
                    Some(LeaseRange {
                        start_ip: Some(from),
                        end_ip: Some(to),
                    })
                } else {
                    bail!("lease_from is defined but no lease_to")
                }
            } else if value.lease_to.is_some() {
                bail!("lease_to is defined but no lease_from")
            } else {
                None
            },
        })
    }
}

impl From<Subnet> for SubnetConfig {
    fn from(value: Subnet) -> Self {
        Self {
            subnet: value.subnet,
            gateway: value.gateway,
            lease_from: value.lease_range.clone().map(|f| f.start_ip).flatten(),
            lease_to: value.lease_range.map(|f| f.end_ip).flatten(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct NetworkRemoved {
    pub name: String,
    #[serde(default)]
    pub force: Option<bool>,
}
