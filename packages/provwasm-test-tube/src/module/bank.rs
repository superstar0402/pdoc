use provwasm_std::types::cosmos::bank::v1beta1::{
    MsgMultiSend, MsgMultiSendResponse, MsgSend, MsgSendResponse, QueryBalanceRequest,
    QueryBalanceResponse, QueryDenomMetadataRequest, QueryDenomMetadataResponse,
    QueryParamsRequest, QueryParamsResponse, QuerySupplyOfRequest, QuerySupplyOfResponse,
};

use test_tube_prov::module::Module;
use test_tube_prov::{fn_execute, fn_query, Runner};

pub struct Bank<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Bank<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Bank<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub send: MsgSend["/cosmos.bank.v1beta1.MsgSend"] => MsgSendResponse
    }

    fn_execute! {
        pub multi_send: MsgMultiSend["/cosmos.bank.v1beta1.MsgMultiSend"] => MsgMultiSendResponse
    }

    fn_query! {
        pub query_balance ["/cosmos.bank.v1beta1.Query/Balance"]: QueryBalanceRequest => QueryBalanceResponse
    }

    fn_query! {
        pub query_denom_metadata ["/cosmos.bank.v1beta1.Query/DenomMetadata"]: QueryDenomMetadataRequest => QueryDenomMetadataResponse
    }

    fn_query! {
        pub query_params ["/cosmos.bank.v1beta1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_supply_of ["/cosmos.bank.v1beta1.Query/SupplyOf"]: QuerySupplyOfRequest => QuerySupplyOfResponse
    }
}
