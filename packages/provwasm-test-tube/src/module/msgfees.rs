use provwasm_std::types::provenance::msgfees::v1::{
    MsgAssessCustomMsgFeeRequest, MsgAssessCustomMsgFeeResponse, QueryParamsRequest,
    QueryParamsResponse,
};
use test_tube::module::Module;
use test_tube::runner::Runner;
use test_tube::{fn_execute, fn_query};

pub struct MsgFees<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for MsgFees<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> MsgFees<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub assess_custom_msg_fee: MsgAssessCustomMsgFeeRequest["/provenance.msgfees.v1.MsgAssessCustomMsgFeeRequest"] => MsgAssessCustomMsgFeeResponse
    }

    fn_query! {
        pub query_params ["/provenance.marker.v1.Query/QueryParamsRequest"]: QueryParamsRequest => QueryParamsResponse
    }
}
