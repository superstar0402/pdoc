use provwasm_std::types::provenance::exchange::v1::{
    MsgAcceptPaymentRequest, MsgAcceptPaymentResponse, MsgCancelOrderRequest,
    MsgCancelOrderResponse, MsgCancelPaymentsRequest, MsgCancelPaymentsResponse,
    MsgChangePaymentTargetRequest, MsgChangePaymentTargetResponse, MsgCommitFundsRequest,
    MsgCommitFundsResponse, MsgCreateAskRequest, MsgCreateAskResponse, MsgCreateBidRequest,
    MsgCreateBidResponse, MsgCreatePaymentRequest, MsgCreatePaymentResponse, MsgFillAsksRequest,
    MsgFillAsksResponse, MsgFillBidsRequest, MsgFillBidsResponse, MsgGovCloseMarketRequest,
    MsgGovCloseMarketResponse, MsgGovCreateMarketRequest, MsgGovCreateMarketResponse,
    MsgGovManageFeesRequest, MsgGovManageFeesResponse, MsgGovUpdateParamsRequest,
    MsgGovUpdateParamsResponse, MsgMarketCommitmentSettleRequest,
    MsgMarketCommitmentSettleResponse, MsgMarketManagePermissionsRequest,
    MsgMarketManagePermissionsResponse, MsgMarketManageReqAttrsRequest,
    MsgMarketManageReqAttrsResponse, MsgMarketReleaseCommitmentsRequest,
    MsgMarketSetOrderExternalIdRequest, MsgMarketSetOrderExternalIdResponse,
    MsgMarketSettleRequest, MsgMarketSettleResponse, MsgMarketUpdateAcceptingCommitmentsRequest,
    MsgMarketUpdateAcceptingCommitmentsResponse, MsgMarketUpdateAcceptingOrdersRequest,
    MsgMarketUpdateAcceptingOrdersResponse, MsgMarketUpdateDetailsRequest,
    MsgMarketUpdateDetailsResponse, MsgMarketUpdateEnabledRequest, MsgMarketUpdateEnabledResponse,
    MsgMarketUpdateIntermediaryDenomRequest, MsgMarketUpdateIntermediaryDenomResponse,
    MsgMarketUpdateUserSettleRequest, MsgMarketUpdateUserSettleResponse, MsgMarketWithdrawRequest,
    MsgMarketWithdrawResponse, MsgRejectPaymentRequest, MsgRejectPaymentResponse,
    MsgRejectPaymentsRequest, MsgRejectPaymentsResponse, QueryGetAllMarketsRequest,
    QueryGetAllMarketsResponse, QueryGetAllOrdersRequest, QueryGetAllOrdersResponse,
    QueryGetAssetOrdersRequest, QueryGetAssetOrdersResponse, QueryGetMarketOrdersRequest,
    QueryGetMarketOrdersResponse, QueryGetMarketRequest, QueryGetMarketResponse,
    QueryGetOrderByExternalIdRequest, QueryGetOrderByExternalIdResponse, QueryGetOrderRequest,
    QueryGetOrderResponse, QueryGetOwnerOrdersRequest, QueryGetOwnerOrdersResponse,
    QueryOrderFeeCalcRequest, QueryOrderFeeCalcResponse, QueryParamsRequest, QueryParamsResponse,
    QueryValidateCreateMarketRequest, QueryValidateCreateMarketResponse,
    QueryValidateManageFeesRequest, QueryValidateManageFeesResponse, QueryValidateMarketRequest,
    QueryValidateMarketResponse,
};

use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Exchange<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Exchange<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Exchange<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_ask: MsgCreateAskRequest["/provenance.exchange.v1.MsgCreateAskRequest"] => MsgCreateAskResponse
    }

    fn_execute! {
        pub create_bid: MsgCreateBidRequest["/provenance.exchange.v1.MsgCreateBidRequest"] => MsgCreateBidResponse
    }

    fn_execute! {
        pub commit_funds: MsgCommitFundsRequest["/provenance.exchange.v1.MsgCommitFundsRequest"] => MsgCommitFundsResponse
    }

    fn_execute! {
        pub cancel_order: MsgCancelOrderRequest["/provenance.exchange.v1.MsgCancelOrderRequest"] => MsgCancelOrderResponse
    }

    fn_execute! {
        pub fill_bids: MsgFillBidsRequest["/provenance.exchange.v1.MsgFillBidsRequest"] => MsgFillBidsResponse
    }

    fn_execute! {
        pub fill_asks: MsgFillAsksRequest["/provenance.exchange.v1.MsgFillAsksRequest"] => MsgFillAsksResponse
    }

    fn_execute! {
        pub market_settle: MsgMarketSettleRequest["/provenance.exchange.v1.MsgMarketSettleRequest"] => MsgMarketSettleResponse
    }

    fn_execute! {
        pub market_commitment_settle: MsgMarketCommitmentSettleRequest["/provenance.exchange.v1.MsgMarketCommitmentSettleRequest"] => MsgMarketCommitmentSettleResponse
    }

    fn_execute! {
        pub market_release_commitments: MsgMarketReleaseCommitmentsRequest["/provenance.exchange.v1.MsgMarketCommitmentSettleRequest"] => MsgMarketCommitmentSettleResponse
    }

    fn_execute! {
        pub set_order_external_id: MsgMarketSetOrderExternalIdRequest["/provenance.exchange.v1.MsgMarketSetOrderExternalIDRequest"] => MsgMarketSetOrderExternalIdResponse
    }

    fn_execute! {
        pub withdraw: MsgMarketWithdrawRequest["/provenance.exchange.v1.MsgMarketWithdrawRequest"] => MsgMarketWithdrawResponse
    }

    fn_execute! {
        pub market_update_details: MsgMarketUpdateDetailsRequest["/provenance.exchange.v1.MsgMarketUpdateDetailsRequest"] => MsgMarketUpdateDetailsResponse
    }

    fn_execute! {
        pub market_update_enabled: MsgMarketUpdateEnabledRequest["/provenance.exchange.v1.MsgMarketUpdateEnabledRequest"] => MsgMarketUpdateEnabledResponse
    }

    fn_execute! {
        pub market_update_accepting_orders: MsgMarketUpdateAcceptingOrdersRequest["/provenance.exchange.v1.MsgMarketUpdateAcceptingOrdersRequest"] => MsgMarketUpdateAcceptingOrdersResponse
    }

    fn_execute! {
        pub market_update_user_settle: MsgMarketUpdateUserSettleRequest["/provenance.exchange.v1.MsgMarketUpdateUserSettleRequest"] => MsgMarketUpdateUserSettleResponse
    }

    fn_execute! {
        pub market_update_accepting_commitments: MsgMarketUpdateAcceptingCommitmentsRequest["/provenance.exchange.v1.MsgMarketUpdateAcceptingCommitmentsRequest"] => MsgMarketUpdateAcceptingCommitmentsResponse
    }

    fn_execute! {
        pub market_update_intermediary_denom: MsgMarketUpdateIntermediaryDenomRequest["/provenance.exchange.v1.MsgMarketUpdateIntermediaryDenomRequest"] => MsgMarketUpdateIntermediaryDenomResponse
    }

    fn_execute! {
        pub market_manage_permissions: MsgMarketManagePermissionsRequest["/provenance.exchange.v1.MsgMarketManagePermissionsRequest"] => MsgMarketManagePermissionsResponse
    }

    fn_execute! {
        pub market_manage_req_attrs: MsgMarketManageReqAttrsRequest["/provenance.exchange.v1.MsgMarketManageReqAttrsRequest"] => MsgMarketManageReqAttrsResponse
    }

    fn_execute! {
        pub create_payment: MsgCreatePaymentRequest["/provenance.exchange.v1.MsgCreatePaymentRequest"] => MsgCreatePaymentResponse
    }

    fn_execute! {
        pub accept_payment: MsgAcceptPaymentRequest["/provenance.exchange.v1.MsgAcceptPaymentRequest"] => MsgAcceptPaymentResponse
    }

    fn_execute! {
        pub reject_payment: MsgRejectPaymentRequest["/provenance.exchange.v1.MsgRejectPaymentRequest"] => MsgRejectPaymentResponse
    }

    fn_execute! {
        pub reject_payments: MsgRejectPaymentsRequest["/provenance.exchange.v1.MsgRejectPaymentsRequest"] => MsgRejectPaymentsResponse
    }

    fn_execute! {
        pub cancel_payments: MsgCancelPaymentsRequest["/provenance.exchange.v1.MsgCancelPaymentsResponse"] => MsgCancelPaymentsResponse
    }

    fn_execute! {
        pub change_payment_target: MsgChangePaymentTargetRequest["/provenance.exchange.v1.MsgChangePaymentTargetRequest"] => MsgChangePaymentTargetResponse
    }

    fn_execute! {
        pub gov_create_market: MsgGovCreateMarketRequest["/provenance.exchange.v1.MsgGovCreateMarketRequest"] => MsgGovCreateMarketResponse
    }

    fn_execute! {
        pub gov_manage_fees: MsgGovManageFeesRequest["/provenance.exchange.v1.MsgGovManageFeesRequest"] => MsgGovManageFeesResponse
    }

    fn_execute! {
        pub gov_close_market: MsgGovCloseMarketRequest["/provenance.exchange.v1.MsgGovCloseMarketRequest"] => MsgGovCloseMarketResponse
    }

    fn_execute! {
        pub gov_update_params: MsgGovUpdateParamsRequest["/provenance.exchange.v1.MsgGovUpdateParamsRequest"] => MsgGovUpdateParamsResponse
    }

    fn_query! {
        pub query_order_fee_calc ["/provenance.exchange.v1.Query/OrderFeeCalc"]: QueryOrderFeeCalcRequest => QueryOrderFeeCalcResponse
    }

    fn_query! {
        pub query_get_order ["/provenance.exchange.v1.Query/GetOrder"]: QueryGetOrderRequest => QueryGetOrderResponse
    }

    fn_query! {
        pub query_get_order_by_external_id ["/provenance.exchange.v1.Query/GetOrderByExternalID"]: QueryGetOrderByExternalIdRequest => QueryGetOrderByExternalIdResponse
    }

    fn_query! {
        pub query_get_market_orders ["/provenance.exchange.v1.Query/GetMarketOrders"]: QueryGetMarketOrdersRequest => QueryGetMarketOrdersResponse
    }

    fn_query! {
        pub query_get_owner_orders ["/provenance.exchange.v1.Query/GetOwnerOrders"]: QueryGetOwnerOrdersRequest => QueryGetOwnerOrdersResponse
    }

    fn_query! {
        pub query_get_asset_orders ["/provenance.exchange.v1.Query/GetAssetOrders"]: QueryGetAssetOrdersRequest => QueryGetAssetOrdersResponse
    }

    fn_query! {
        pub query_get_all_orders ["/provenance.exchange.v1.Query/GetAllOrders"]: QueryGetAllOrdersRequest => QueryGetAllOrdersResponse
    }

    fn_query! {
        pub query_get_market ["/provenance.exchange.v1.Query/GetMarket"]: QueryGetMarketRequest => QueryGetMarketResponse
    }

    fn_query! {
        pub query_get_all_markets ["/provenance.exchange.v1.Query/GetAllMarkets"]: QueryGetAllMarketsRequest => QueryGetAllMarketsResponse
    }

    fn_query! {
        pub query_params ["/provenance.exchange.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_validate_create_market ["/provenance.exchange.v1.Query/ValidateCreateMarket"]: QueryValidateCreateMarketRequest => QueryValidateCreateMarketResponse
    }

    fn_query! {
        pub query_validate_market ["/provenance.exchange.v1.Query/ValidateMarket"]: QueryValidateMarketRequest => QueryValidateMarketResponse
    }

    fn_query! {
        pub query_validate_manage_fees ["/provenance.exchange.v1.Query/ValidateManageFees"]: QueryValidateManageFeesRequest => QueryValidateManageFeesResponse
    }
}
