use anyhow::Result;
use tracing::{error, info, trace, warn};
use trust_dns_server::{
    authority::{
        AuthorityObject, LookupOptions, MessageRequest, MessageResponseBuilder, UpdateResult,
    },
    client::rr::LowerName,
    proto::{
        op::{Header, MessageType, ResponseCode},
        rr::RecordType,
    },
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub struct DnsHandler();
/*
#[async_trait::async_trait]
impl AuthorityObject for DnsHandler {
    fn box_clone(&self) -> Box<dyn AuthorityObject>  {
        todo!()
    }

    fn zone_type(&self) -> trust_dns_server::authority::ZoneType {
        todo!()
    }

    fn is_axfr_allowed(&self) -> bool {
        todo!()
    }

    async fn update(&self, _update: &MessageRequest) -> UpdateResult<bool> {
        Err(ResponseCode::NotImp)
    }

    fn origin(&self) ->  &trust_dns_server::client::rr::LowerName {
        todo!()
    }

    async fn lookup(
        &self,
        name: &LowerName,
        query_type: RecordType,
        lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError>{    todo!()
    }

    fn search<'life0,'life1,'async_trait>(&'life0 self,request_info:trust_dns_server::server::RequestInfo<'life1> ,lookup_options:trust_dns_server::authority::LookupOptions,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = std::result::Result<Box<dyn trust_dns_server::authority::LookupObject> ,trust_dns_server::authority::LookupError> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }

    fn get_nsec_records<'life0,'life1,'async_trait>(&'life0 self,name: &'life1 trust_dns_server::client::rr::LowerName,lookup_options:trust_dns_server::authority::LookupOptions,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = std::result::Result<Box<dyn trust_dns_server::authority::LookupObject> ,trust_dns_server::authority::LookupError> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    } /*
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> ResponseInfo {
        trace!("DNS req: {:?}", request);

        let result = match request.message_type() {
            MessageType::Query => match request.op_code() {
                /*OpCode::Query => {
                    let info = self.lookup(request, response_edns, response_handle).await;
                    Ok(info)
                }*/
                c => {
                    trace!("unimplemented op_code: {:?}", c);

                    response_handle
                        .send_response(
                            MessageResponseBuilder::from_message_request(request)
                                .error_msg(request.header(), ResponseCode::NotImp),
                        )
                        .await
                }
            },
            MessageType::Response => {
                warn!("got a response as a request from id: {}", request.id());

                response_handle
                    .send_response(
                        MessageResponseBuilder::from_message_request(request)
                            .error_msg(request.header(), ResponseCode::FormErr),
                    )
                    .await
            }
        };

        match result {
            Err(e) => {
                error!(err = e.to_string(), "request failed");

                let mut header = Header::new();
                header.set_response_code(ResponseCode::ServFail);
                header.into()
            }
            Ok(info) => info,
        }
    }*/
}

impl DnsHandler {
    async fn resolve()
}
*/
