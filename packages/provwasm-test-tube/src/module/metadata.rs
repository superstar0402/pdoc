use provwasm_std::types::provenance::metadata::v1::{
    ContractSpecificationRequest, ContractSpecificationResponse,
    MsgAddContractSpecToScopeSpecRequest, MsgAddContractSpecToScopeSpecResponse,
    MsgAddScopeDataAccessRequest, MsgAddScopeDataAccessResponse, MsgAddScopeOwnerRequest,
    MsgAddScopeOwnerResponse, MsgBindOsLocatorRequest, MsgBindOsLocatorResponse,
    MsgDeleteContractSpecFromScopeSpecRequest, MsgDeleteContractSpecFromScopeSpecResponse,
    MsgDeleteContractSpecificationRequest, MsgDeleteContractSpecificationResponse,
    MsgDeleteOsLocatorRequest, MsgDeleteOsLocatorResponse, MsgDeleteRecordRequest,
    MsgDeleteRecordResponse, MsgDeleteRecordSpecificationRequest,
    MsgDeleteRecordSpecificationResponse, MsgDeleteScopeDataAccessRequest,
    MsgDeleteScopeDataAccessResponse, MsgDeleteScopeOwnerRequest, MsgDeleteScopeOwnerResponse,
    MsgDeleteScopeRequest, MsgDeleteScopeResponse, MsgDeleteScopeSpecificationRequest,
    MsgDeleteScopeSpecificationResponse, MsgMigrateValueOwnerRequest, MsgMigrateValueOwnerResponse,
    MsgModifyOsLocatorRequest, MsgModifyOsLocatorResponse, MsgUpdateValueOwnersRequest,
    MsgUpdateValueOwnersResponse, MsgWriteContractSpecificationRequest,
    MsgWriteContractSpecificationResponse, MsgWriteRecordRequest, MsgWriteRecordResponse,
    MsgWriteRecordSpecificationRequest, MsgWriteRecordSpecificationResponse, MsgWriteScopeRequest,
    MsgWriteScopeResponse, MsgWriteScopeSpecificationRequest, MsgWriteScopeSpecificationResponse,
    MsgWriteSessionRequest, MsgWriteSessionResponse, OsLocatorParamsRequest,
    OsLocatorParamsResponse, OsLocatorRequest, OsLocatorResponse, OsLocatorsByScopeRequest,
    OsLocatorsByScopeResponse, OsLocatorsByUriRequest, OsLocatorsByUriResponse, OwnershipRequest,
    OwnershipResponse, QueryParamsRequest, QueryParamsResponse, RecordSpecificationRequest,
    RecordSpecificationResponse, RecordSpecificationsForContractSpecificationRequest,
    RecordSpecificationsForContractSpecificationResponse, RecordsRequest, RecordsResponse,
    ScopeRequest, ScopeResponse, ScopeSpecificationRequest, ScopeSpecificationResponse,
    SessionsRequest, SessionsResponse, ValueOwnershipRequest, ValueOwnershipResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Metadata<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Metadata<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Metadata<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub write_scope: MsgWriteScopeRequest["/provenance.metadata.v1.MsgWriteScopeRequest"] => MsgWriteScopeResponse
    }

    fn_execute! {
        pub delete_scope: MsgDeleteScopeRequest["/provenance.metadata.v1.MsgDeleteScopeRequest"] => MsgDeleteScopeResponse
    }

    fn_execute! {
        pub add_scope_data_access: MsgAddScopeDataAccessRequest["/provenance.metadata.v1.MsgAddScopeDataAccessRequest"] => MsgAddScopeDataAccessResponse
    }

    fn_execute! {
        pub delete_scope_data_access: MsgDeleteScopeDataAccessRequest["/provenance.metadata.v1.MsgDeleteScopeDataAccessRequest"] => MsgDeleteScopeDataAccessResponse
    }

    fn_execute! {
        pub add_scope_owner: MsgAddScopeOwnerRequest["/provenance.metadata.v1.MsgAddScopeOwnerRequest"] => MsgAddScopeOwnerResponse
    }

    fn_execute! {
        pub delete_scope_owner: MsgDeleteScopeOwnerRequest["/provenance.metadata.v1.MsgDeleteScopeOwnerRequest"] => MsgDeleteScopeOwnerResponse
    }

    fn_execute! {
        pub update_value_owner: MsgUpdateValueOwnersRequest["/provenance.metadata.v1.MsgUpdateValueOwnersRequest"] => MsgUpdateValueOwnersResponse
    }

    fn_execute! {
        pub migrate_value_owner: MsgMigrateValueOwnerRequest["/provenance.metadata.v1.MsgMigrateValueOwnerRequest"] => MsgMigrateValueOwnerResponse
    }

    fn_execute! {
        pub write_session: MsgWriteSessionRequest["/provenance.metadata.v1.MsgWriteSessionRequest"] => MsgWriteSessionResponse
    }

    fn_execute! {
        pub write_record: MsgWriteRecordRequest["/provenance.metadata.v1.MsgWriteRecordRequest"] => MsgWriteRecordResponse
    }

    fn_execute! {
        pub delete_record: MsgDeleteRecordRequest["/provenance.metadata.v1.MsgDeleteRecordRequest"] => MsgDeleteRecordResponse
    }

    fn_execute! {
        pub write_scope_specification: MsgWriteScopeSpecificationRequest["/provenance.metadata.v1.MsgWriteScopeSpecificationRequest"] => MsgWriteScopeSpecificationResponse
    }

    fn_execute! {
        pub delete_scope_specification: MsgDeleteScopeSpecificationRequest["/provenance.metadata.v1.MsgDeleteScopeSpecificationRequest"] => MsgDeleteScopeSpecificationResponse
    }

    fn_execute! {
        pub write_contract_specification: MsgWriteContractSpecificationRequest["/provenance.metadata.v1.MsgWriteContractSpecificationRequest"] => MsgWriteContractSpecificationResponse
    }

    fn_execute! {
        pub add_contract_spec_to_scope_spec: MsgAddContractSpecToScopeSpecRequest["/provenance.metadata.v1.MsgAddContractSpecToScopeSpecRequest"] => MsgAddContractSpecToScopeSpecResponse
    }

    fn_execute! {
        pub delete_contract_spec_from_scope_spec: MsgDeleteContractSpecFromScopeSpecRequest["/provenance.metadata.v1.MsgDeleteContractSpecFromScopeSpecRequest"] => MsgDeleteContractSpecFromScopeSpecResponse
    }

    fn_execute! {
        pub delete_contract_specification: MsgDeleteContractSpecificationRequest["/provenance.metadata.v1.MsgDeleteContractSpecificationRequest"] => MsgDeleteContractSpecificationResponse
    }

    fn_execute! {
        pub write_record_specification: MsgWriteRecordSpecificationRequest["/provenance.metadata.v1.MsgWriteRecordSpecificationRequest"] => MsgWriteRecordSpecificationResponse
    }

    fn_execute! {
        pub delete_record_specification: MsgDeleteRecordSpecificationRequest["/provenance.metadata.v1.MsgDeleteRecordSpecificationRequest"] => MsgDeleteRecordSpecificationResponse
    }

    fn_execute! {
        pub bind_os_locator: MsgBindOsLocatorRequest["/provenance.metadata.v1.MsgBindOSLocatorRequest"] => MsgBindOsLocatorResponse
    }

    fn_execute! {
        pub delete_os_locator: MsgDeleteOsLocatorRequest["/provenance.metadata.v1.MsgDeleteOSLocatorRequest"] => MsgDeleteOsLocatorResponse
    }

    fn_execute! {
        pub modify_os_locator: MsgModifyOsLocatorRequest["/provenance.metadata.v1.MsgModifyOSLocatorRequest"] => MsgModifyOsLocatorResponse
    }

    fn_query! {
        pub query_params ["/provenance.metadata.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_scope ["/provenance.metadata.v1.Query/Scope"]: ScopeRequest => ScopeResponse
    }

    fn_query! {
        pub query_sessions ["/provenance.metadata.v1.Query/Sessions"]: SessionsRequest => SessionsResponse
    }

    fn_query! {
        pub query_records ["/provenance.metadata.v1.Query/Records"]: RecordsRequest => RecordsResponse
    }

    fn_query! {
        pub query_ownership ["/provenance.metadata.v1.Query/Ownership"]: OwnershipRequest => OwnershipResponse
    }

    fn_query! {
        pub query_value_ownership ["/provenance.metadata.v1.Query/ValueOwnership"]: ValueOwnershipRequest => ValueOwnershipResponse
    }

    fn_query! {
        pub query_scope_specification["/provenance.metadata.v1.Query/ScopeSpecification"]: ScopeSpecificationRequest => ScopeSpecificationResponse
    }

    fn_query! {
        pub query_contract_specification ["/provenance.metadata.v1.Query/ContractSpecification"]: ContractSpecificationRequest => ContractSpecificationResponse
    }

    fn_query! {
        pub query_record_specs_for_contract_spec ["/provenance.metadata.v1.Query/RecordSpecificationsForContractSpecification"]: RecordSpecificationsForContractSpecificationRequest => RecordSpecificationsForContractSpecificationResponse
    }

    fn_query! {
        pub query_record_specification ["/provenance.metadata.v1.Query/RecordSpecification"]: RecordSpecificationRequest => RecordSpecificationResponse
    }

    fn_query! {
        pub query_os_locator_params ["/provenance.metadata.v1.Query/OSLocatorParams"]: OsLocatorParamsRequest => OsLocatorParamsResponse
    }

    fn_query! {
        pub query_os_locator ["/provenance.metadata.v1.Query/OSLocator"]: OsLocatorRequest => OsLocatorResponse
    }

    fn_query! {
        pub query_os_locators_by_uri ["/provenance.metadata.v1.Query/OSLocatorsByURI"]: OsLocatorsByUriRequest => OsLocatorsByUriResponse
    }

    fn_query! {
        pub query_os_locators_by_scope ["/provenance.metadata.v1.Query/OSLocatorsByScope"]: OsLocatorsByScopeRequest => OsLocatorsByScopeResponse
    }
}
