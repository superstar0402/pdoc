use provwasm_std::types::provenance::attribute::v1::{
    MsgAddAttributeRequest, MsgAddAttributeResponse, MsgDeleteDistinctAttributeRequest,
    MsgUpdateAttributeRequest, QueryAttributeRequest, QueryAttributeResponse,
    QueryAttributesRequest, QueryAttributesResponse, QueryParamsRequest, QueryParamsResponse,
    QueryScanRequest, QueryScanResponse,
};

use test_tube::module::Module;
use test_tube::runner::Runner;
use test_tube::{fn_execute, fn_query};

pub struct Attribute<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Attribute<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Attribute<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub add_attribute: MsgAddAttributeRequest["/provenance.attribute.v1.MsgAddAttributeRequest"] => MsgAddAttributeResponse
    }

    fn_execute! {
        pub update_attribute: MsgUpdateAttributeRequest["/provenance.attribute.v1.MsgUpdateAttributeRequest"] => MsgAddAttributeResponse
    }

    fn_execute! {
        pub delete_distinct_attribute: MsgDeleteDistinctAttributeRequest["/provenance.attribute.v1.MsgDeleteDistinctAttributeRequest"] => MsgAddAttributeResponse
    }

    fn_query! {
        pub query_attribute ["/provenance.attribute.v1.Query/QueryAttributeRequest"]: QueryAttributeRequest => QueryAttributeResponse
    }

    fn_query! {
        pub query_attributes ["/provenance.attribute.v1.Query/QueryAttributesRequest"]: QueryAttributesRequest => QueryAttributesResponse
    }

    fn_query! {
        pub query_params ["/provenance.attribute.v1.Query/QueryParamsRequest"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_scan ["/provenance.attribute.v1.Query/QueryScanRequest"]: QueryScanRequest => QueryScanResponse
    }
}
