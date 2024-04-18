use provwasm_std::types::provenance::msgfees::v1::{
    CalculateTxFeesRequest, CalculateTxFeesResponse, MsgAssessCustomMsgFeeRequest,
    MsgAssessCustomMsgFeeResponse, QueryAllMsgFeesRequest, QueryAllMsgFeesResponse,
    QueryParamsRequest, QueryParamsResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

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
        pub query_params ["/provenance.msgfees.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_all_msg_fees ["/provenance.msgfees.v1.Query/QueryAllMsgFees"]: QueryAllMsgFeesRequest => QueryAllMsgFeesResponse
    }

    fn_query! {
        pub query_calculate_tx_fees ["/provenance.msgfees.v1.Query/CalculateTxFees"]: CalculateTxFeesRequest => CalculateTxFeesResponse
    }
}
