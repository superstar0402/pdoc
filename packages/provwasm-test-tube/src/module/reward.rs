use provwasm_std::types::provenance::reward::v1::{
    MsgClaimAllRewardsRequest, MsgClaimAllRewardsResponse, MsgClaimRewardsRequest,
    MsgClaimRewardsResponse, MsgCreateRewardProgramRequest, MsgCreateRewardProgramResponse,
    MsgEndRewardProgramRequest, MsgEndRewardProgramResponse,
    QueryClaimPeriodRewardDistributionsByIdRequest,
    QueryClaimPeriodRewardDistributionsByIdResponse, QueryClaimPeriodRewardDistributionsRequest,
    QueryClaimPeriodRewardDistributionsResponse, QueryRewardDistributionsByAddressRequest,
    QueryRewardDistributionsByAddressResponse, QueryRewardProgramByIdRequest,
    QueryRewardProgramByIdResponse, QueryRewardProgramsRequest, QueryRewardProgramsResponse,
};
use test_tube::module::Module;
use test_tube::runner::Runner;
use test_tube::{fn_execute, fn_query};

pub struct Reward<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Reward<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Reward<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_reward_program: MsgCreateRewardProgramRequest["/provenance.reward.v1.MsgCreateRewardProgramRequest"] => MsgCreateRewardProgramResponse
    }

    fn_execute! {
        pub end_reward_program: MsgEndRewardProgramRequest["/provenance.reward.v1.MsgEndRewardProgramRequest"] => MsgEndRewardProgramResponse
    }

    fn_execute! {
        pub claim_rewards: MsgClaimRewardsRequest["/provenance.reward.v1.MsgClaimRewardsRequest"] => MsgClaimRewardsResponse
    }

    fn_execute! {
        pub claim_all_rewards: MsgClaimAllRewardsRequest["/provenance.reward.v1.MsgClaimAllRewardsRequest"] => MsgClaimAllRewardsResponse
    }

    fn_query! {
        pub query_ ["/provenance.reward.v1.Query/QueryRewardProgramByIDRequest"]: QueryRewardProgramByIdRequest => QueryRewardProgramByIdResponse
    }

    fn_query! {
        pub query_reward_programs ["/provenance.reward.v1.Query/QueryRewardProgramsRequest"]: QueryRewardProgramsRequest => QueryRewardProgramsResponse
    }

    fn_query! {
        pub query_claim_period_reward_distributions ["/provenance.reward.v1.Query/QueryClaimPeriodRewardDistributionsRequest"]: QueryClaimPeriodRewardDistributionsRequest => QueryClaimPeriodRewardDistributionsResponse
    }

    fn_query! {
        pub query_claim_period_reward_distributions_by_id ["/provenance.reward.v1.Query/QueryClaimPeriodRewardDistributionsByIDRequest"]: QueryClaimPeriodRewardDistributionsByIdRequest => QueryClaimPeriodRewardDistributionsByIdResponse
    }

    fn_query! {
        pub query_reward_distributions_by_address ["/provenance.reward.v1.Query/QueryRewardDistributionsByAddressRequest"]: QueryRewardDistributionsByAddressRequest => QueryRewardDistributionsByAddressResponse
    }
}
