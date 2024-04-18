use test_tube_prov::{fn_execute, fn_query, Module, Runner};

use provwasm_std::types::cosmos::gov::v1beta1::{
    MsgSubmitProposal, MsgSubmitProposalResponse, MsgVote, MsgVoteResponse, QueryDepositRequest,
    QueryDepositResponse, QueryParamsRequest, QueryParamsResponse, QueryVoteRequest,
    QueryVoteResponse,
};

pub struct Gov<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Gov<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Gov<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub submit_proposal: MsgSubmitProposal["/cosmos.gov.v1beta1.MsgSubmitProposal"] => MsgSubmitProposalResponse
    }

    fn_execute! {
        pub vote: MsgVote["/cosmos.gov.v1beta1.MsgVote"] => MsgVoteResponse
    }

    fn_query! {
        pub query_vote ["/cosmos.gov.v1beta1.Query/Vote"]: QueryVoteRequest => QueryVoteResponse
    }

    fn_query! {
        pub query_params ["/cosmos.gov.v1beta1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_deposit ["/cosmos.gov.v1beta1.Query/Deposit"]: QueryDepositRequest => QueryDepositResponse
    }
}
