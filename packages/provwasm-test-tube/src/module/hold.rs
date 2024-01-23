use provwasm_std::types::provenance::hold::v1::{
    AccountHold, GetAllHoldsRequest, GetAllHoldsResponse, GetHoldsRequest, GetHoldsResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Hold<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Hold<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Hold<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub account_hold: AccountHold["/provenance.hold.v1.AccountHold"] => ()
    }

    fn_query! {
        pub query_get_holds ["/provenance.hold.v1.Query/GetHolds"]: GetHoldsRequest => GetHoldsResponse
    }

    fn_query! {
        pub query_get_all_holds ["/provenance.hold.v1.Query/GetAllHolds"]: GetAllHoldsRequest => GetAllHoldsResponse
    }
}
