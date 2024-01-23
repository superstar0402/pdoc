use provwasm_std::types::provenance::trigger::v1::{
    MsgCreateTriggerRequest, MsgCreateTriggerResponse, MsgDestroyTriggerRequest,
    MsgDestroyTriggerResponse, QueryTriggerByIdRequest, QueryTriggerByIdResponse,
    QueryTriggersRequest, QueryTriggersResponse,
};

use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Trigger<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Trigger<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Trigger<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_trigger: MsgCreateTriggerRequest["/provenance.trigger.v1.MsgCreateTriggerRequest"] => MsgCreateTriggerResponse
    }

    fn_execute! {
        pub destroy_trigger: MsgDestroyTriggerRequest["/provenance.trigger.v1.MsgDestroyTriggerRequest"] => MsgDestroyTriggerResponse
    }

    fn_query! {
        pub query_trigger_by_id ["/provenance.trigger.v1.Query/TriggerByID"]: QueryTriggerByIdRequest => QueryTriggerByIdResponse
    }

    fn_query! {
        pub query_triggers ["/provenance.trigger.v1.Query/Triggers"]: QueryTriggersRequest => QueryTriggersResponse
    }
}
