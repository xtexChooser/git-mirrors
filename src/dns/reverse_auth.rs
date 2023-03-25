use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ipnet::IpNet;
use lazy_static::lazy_static;
use tracing::info;
use trust_dns_server::authority::LookupRecords;
use trust_dns_server::proto::rr::{Name, RData, Record, RecordSet};
use trust_dns_server::{
    authority::{
        AuthLookup, Authority, LookupError, LookupOptions, MessageRequest, UpdateResult, ZoneType,
    },
    client::rr::LowerName,
    proto::{op::ResponseCode, rr::RecordType},
    server::RequestInfo,
};

use crate::{resolver, subnet};

lazy_static! {
    pub static ref ARPA: Name = Name::from_str("arpa.").unwrap();
    pub static ref ARPA_LC: LowerName = ARPA.clone().into();
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ReverseAuth();

#[async_trait::async_trait]
impl Authority for ReverseAuth {
    type Lookup = AuthLookup;

    fn zone_type(&self) -> ZoneType {
        ZoneType::Primary
    }

    fn is_axfr_allowed(&self) -> bool {
        false
    }

    async fn update(&self, _update: &MessageRequest) -> UpdateResult<bool> {
        Err(ResponseCode::NotImp)
    }

    fn origin(&self) -> &'static LowerName {
        &ARPA_LC
    }

    async fn lookup(
        &self,
        name: &LowerName,
        query_type: RecordType,
        _lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        if query_type != RecordType::PTR && query_type != RecordType::ANY {
            return Ok(AuthLookup::Empty);
        }
        let net = Name::from(name)
            .parse_arpa_name()
            .map_err(|_| LookupError::ResponseCode(ResponseCode::BADNAME))?;

        match net {
            IpNet::V4(_) => Err(LookupError::ResponseCode(ResponseCode::BADNAME)),
            IpNet::V6(net) => {
                if let Some((_, index, hop)) = subnet::try_parse(net.addr()) {
                    info!(
                        name = name.to_string(),
                        subnet = net.to_string(),
                        index,
                        hop,
                        "resolving IPv6 rDNS: {net:?}"
                    );
                    if let Some(chain) = resolver::resolve(index).await {
                        if let Ok(rname) = chain.name(hop.into()) {
                            Ok(AuthLookup::Records {
                                answers: LookupRecords::Records {
                                    lookup_options: LookupOptions::default(),
                                    records: Arc::new(RecordSet::from(Record::from_rdata(
                                        name.into(),
                                        30,
                                        RData::PTR(rname),
                                    ))),
                                },
                                additionals: None,
                            })
                        } else {
                            Err(LookupError::ResponseCode(ResponseCode::BADNAME))
                        }
                    } else {
                        Err(LookupError::ResponseCode(ResponseCode::BADNAME))
                    }
                } else {
                    Err(LookupError::ResponseCode(ResponseCode::BADNAME))
                }
            }
        }
    }

    async fn search(
        &self,
        request_info: RequestInfo<'_>,
        lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        self.lookup(
            request_info.query.name(),
            request_info.query.query_type(),
            lookup_options,
        )
        .await
    }

    async fn get_nsec_records(
        &self,
        _name: &LowerName,
        _lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        Ok(AuthLookup::default())
    }
}
