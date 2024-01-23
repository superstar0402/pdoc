use provwasm_std::types::provenance::marker::v1::{
    MsgActivateRequest, MsgActivateResponse, MsgAddAccessRequest, MsgAddAccessResponse,
    MsgAddFinalizeActivateMarkerRequest, MsgAddFinalizeActivateMarkerResponse, MsgAddMarkerRequest,
    MsgAddMarkerResponse, MsgBurnRequest, MsgBurnResponse, MsgCancelRequest, MsgCancelResponse,
    MsgDeleteAccessRequest, MsgDeleteAccessResponse, MsgDeleteRequest, MsgDeleteResponse,
    MsgFinalizeRequest, MsgFinalizeResponse, MsgGrantAllowanceRequest, MsgGrantAllowanceResponse,
    MsgMintRequest, MsgMintResponse, MsgSetDenomMetadataRequest, MsgSetDenomMetadataResponse,
    MsgSupplyIncreaseProposalRequest, MsgSupplyIncreaseProposalResponse, MsgTransferRequest,
    MsgTransferResponse, MsgWithdrawRequest, MsgWithdrawResponse, QueryAccessRequest,
    QueryAccessResponse, QueryDenomMetadataRequest, QueryDenomMetadataResponse, QueryEscrowRequest,
    QueryEscrowResponse, QueryHoldingRequest, QueryHoldingResponse, QueryMarkerRequest,
    QueryMarkerResponse, QueryParamsRequest, QueryParamsResponse, QuerySupplyRequest,
    QuerySupplyResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Marker<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Marker<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Marker<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub grant_allowance: MsgGrantAllowanceRequest["/provenance.marker.v1.MsgGrantAllowanceRequest"] => MsgGrantAllowanceResponse
    }

    fn_execute! {
        pub add_marker: MsgAddMarkerRequest["/provenance.marker.v1.MsgAddMarkerRequest"] => MsgAddMarkerResponse
    }

    fn_execute! {
        pub add_access: MsgAddAccessRequest["/provenance.marker.v1.MsgAddAccessRequest"] => MsgAddAccessResponse
    }

    fn_execute! {
        pub delete_access: MsgDeleteAccessRequest["/provenance.marker.v1.MsgDeleteAccessRequest"] => MsgDeleteAccessResponse
    }

    fn_execute! {
        pub finalize: MsgFinalizeRequest["/provenance.marker.v1.MsgFinalizeRequest"] => MsgFinalizeResponse
    }

    fn_execute! {
        pub activate: MsgActivateRequest["/provenance.marker.v1.MsgActivateRequest"] => MsgActivateResponse
    }

    fn_execute! {
        pub cancel: MsgCancelRequest["/provenance.marker.v1.MsgCancelRequest"] => MsgCancelResponse
    }

    fn_execute! {
        pub delete: MsgDeleteRequest["/provenance.marker.v1.MsgDeleteRequest"] => MsgDeleteResponse
    }

    fn_execute! {
        pub mint: MsgMintRequest["/provenance.marker.v1.MsgMintRequest"] => MsgMintResponse
    }

    fn_execute! {
        pub burn: MsgBurnRequest["/provenance.marker.v1.MsgBurnRequest"] => MsgBurnResponse
    }

    fn_execute! {
        pub withdraw: MsgWithdrawRequest["/provenance.marker.v1.MsgWithdrawRequest"] => MsgWithdrawResponse
    }

    fn_execute! {
        pub transfer: MsgTransferRequest["/provenance.marker.v1.MsgTransferRequest"] => MsgTransferResponse
    }

    fn_execute! {
        pub set_denom_metadata: MsgSetDenomMetadataRequest["/provenance.marker.v1.MsgSetDenomMetadataRequest"] => MsgSetDenomMetadataResponse
    }

    fn_execute! {
        pub add_finalize_activate: MsgAddFinalizeActivateMarkerRequest["/provenance.marker.v1.MsgAddFinalizeActivateMarkerRequest"] => MsgAddFinalizeActivateMarkerResponse
    }

    fn_execute! {
        pub supply_increase_proposal: MsgSupplyIncreaseProposalRequest["/provenance.marker.v1.MsgSupplyIncreaseProposalRequest"] => MsgSupplyIncreaseProposalResponse
    }

    fn_query! {
        pub query_params ["/provenance.marker.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_marker ["/provenance.marker.v1.Query/Marker"]: QueryMarkerRequest => QueryMarkerResponse
    }

    fn_query! {
        pub query_holding ["/provenance.marker.v1.Query/Holding"]: QueryHoldingRequest => QueryHoldingResponse
    }

    fn_query! {
        pub query_supply ["/provenance.marker.v1.Query/Supply"]: QuerySupplyRequest => QuerySupplyResponse
    }

    fn_query! {
        pub query_escrow ["/provenance.marker.v1.Query/Escrow"]: QueryEscrowRequest => QueryEscrowResponse
    }

    fn_query! {
        pub query_access ["/provenance.marker.v1.Query/Access"]: QueryAccessRequest => QueryAccessResponse
    }

    fn_query! {
        pub query_denom_metadata ["/provenance.marker.v1.Query/DenomMetadata"]: QueryDenomMetadataRequest => QueryDenomMetadataResponse
    }
}
