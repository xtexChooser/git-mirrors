use std::{collections::BTreeMap, str::FromStr, sync::Arc, time::Duration};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, UdpSocket};
use tracing::info;
use trust_dns_server::{
    authority::{AuthorityObject, Catalog, ZoneType},
    client::rr::RrKey,
    proto::rr::{rdata::SOA, Name, RData, Record, RecordSet, RecordType},
    store::in_memory::InMemoryAuthority,
    ServerFuture,
};

use crate::config::get_config;

use self::{forward_auth::ForwardAuth, reverse_auth::ReverseAuth};

pub mod forward_auth;
pub mod reverse_auth;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct DnsConfig {
    pub listen: Vec<DnsListenAddr>,
    pub mname: String,
    pub rname: String,
    pub serial: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DnsListenAddr {
    Udp(String),
    Tcp(String),
}

pub async fn start_dns() -> Result<()> {
    let config = &get_config().dns;
    let mname = Name::from_str(&config.mname)?;
    if !mname.is_fqdn() {
        bail!("mname is not FQDN")
    }
    let rname = Name::from_str(&config.rname)?;
    if !rname.is_fqdn() {
        bail!("rname is not FQDN")
    }

    let mut catalog = Catalog::new();
    catalog.upsert(Name::root().into(), build_root_authority()?);

    catalog.upsert(
        mname.clone().into(),
        Box::new(Arc::new(ForwardAuth {
            origin: mname.into(),
        })),
    );

    catalog.upsert(
        reverse_auth::ARPA_LC.clone(),
        Box::new(Arc::new(ReverseAuth())),
    );

    let mut server = ServerFuture::new(catalog);

    for addr in &config.listen {
        match addr {
            DnsListenAddr::Udp(addr) => {
                info!(addr, "listening UDP");
                server.register_socket(UdpSocket::bind(addr).await?);
            }
            DnsListenAddr::Tcp(addr) => {
                info!(addr, "listening TCP");
                server.register_listener(TcpListener::bind(addr).await?, Duration::from_secs(10));
            }
        }
    }
    server.block_until_done().await?;
    Ok(())
}

fn build_root_authority() -> Result<Box<dyn AuthorityObject>> {
    let config = &get_config().dns;
    let records = BTreeMap::from([(
        RrKey::new(Name::root().into(), RecordType::SOA),
        RecordSet::from(Record::from_rdata(
            Name::root(),
            600,
            RData::SOA(SOA::new(
                Name::from_str(&config.mname)?,
                Name::from_str(&config.rname)?,
                config.serial,
                30,
                30,
                30,
                10,
            )),
        )),
    )]);
    Ok(Box::new(Arc::new(
        InMemoryAuthority::new(Name::root(), records, ZoneType::Primary, false)
            .map_err(anyhow::Error::msg)?,
    )))
}
