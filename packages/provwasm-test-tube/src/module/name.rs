use provwasm_std::types::provenance::name::v1::{
    MsgBindNameRequest, MsgBindNameResponse, MsgCreateRootNameRequest, MsgCreateRootNameResponse,
    MsgDeleteNameRequest, MsgDeleteNameResponse, MsgModifyNameRequest, MsgModifyNameResponse,
    QueryParamsRequest, QueryParamsResponse, QueryResolveRequest, QueryResolveResponse,
    QueryReverseLookupRequest, QueryReverseLookupResponse,
};

use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Name<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Name<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Name<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub bind_name: MsgBindNameRequest["/provenance.name.v1.MsgBindNameRequest"] => MsgBindNameResponse
    }

    fn_execute! {
        pub delete_name: MsgDeleteNameRequest["/provenance.name.v1.MsgDeleteNameRequest"] => MsgDeleteNameResponse
    }

    fn_execute! {
        pub create_root_name: MsgCreateRootNameRequest["/provenance.name.v1.MsgCreateRootNameRequest"] => MsgCreateRootNameResponse
    }

    fn_execute! {
        pub modify_name: MsgModifyNameRequest["/provenance.name.v1.MsgModifyNameRequest"] => MsgModifyNameResponse
    }

    fn_query! {
        pub query_params ["/provenance.name.v1.Query/QueryParamsRequest"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_resolve ["/provenance.name.v1.Query/QueryResolveRequest"]: QueryResolveRequest => QueryResolveResponse
    }

    fn_query! {
        pub query_reverse_lookup ["/provenance.name.v1.Query/QueryReverseLookupRequest"]: QueryReverseLookupRequest => QueryReverseLookupResponse
    }
}
