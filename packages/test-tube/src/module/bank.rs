#![cfg(feature = "bank")]

use crate::{fn_execute, fn_query};
use provwasm_std::types::cosmos::bank::v1beta1::{
    MsgSend, MsgSendResponse, QueryBalanceRequest, QueryBalanceResponse, QuerySupplyOfRequest,
    QuerySupplyOfResponse,
};

use crate::module::Module;
use crate::runner::Runner;

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

    fn_query! {
        pub query_balance ["/cosmos.bank.v1beta1.Query/Balance"]: QueryBalanceRequest => QueryBalanceResponse
    }

    fn_query! {
        pub query_supply_of ["/cosmos.bank.v1beta1.Query/SupplyOf"]: QuerySupplyOfRequest => QuerySupplyOfResponse
    }
}
