use provwasm_std::types::provenance::exchange::v1::{
    MsgCancelOrderRequest, MsgCancelOrderResponse, MsgCreateAskRequest, MsgCreateAskResponse,
    MsgCreateBidRequest, MsgCreateBidResponse, MsgFillAsksRequest, MsgFillAsksResponse,
    MsgFillBidsRequest, MsgFillBidsResponse, MsgGovCreateMarketRequest, MsgGovCreateMarketResponse,
    MsgGovManageFeesRequest, MsgGovManageFeesResponse, MsgGovUpdateParamsRequest,
    MsgGovUpdateParamsResponse, MsgMarketManagePermissionsRequest,
    MsgMarketManagePermissionsResponse, MsgMarketManageReqAttrsRequest,
    MsgMarketManageReqAttrsResponse, MsgMarketSetOrderExternalIdRequest,
    MsgMarketSetOrderExternalIdResponse, MsgMarketSettleRequest, MsgMarketSettleResponse,
    MsgMarketUpdateDetailsRequest, MsgMarketUpdateDetailsResponse, MsgMarketUpdateEnabledRequest,
    MsgMarketUpdateEnabledResponse, MsgMarketUpdateUserSettleRequest,
    MsgMarketUpdateUserSettleResponse, MsgMarketWithdrawRequest, MsgMarketWithdrawResponse,
    QueryGetAllMarketsRequest, QueryGetAllMarketsResponse, QueryGetAllOrdersRequest,
    QueryGetAllOrdersResponse, QueryGetAssetOrdersRequest, QueryGetAssetOrdersResponse,
    QueryGetMarketOrdersRequest, QueryGetMarketOrdersResponse, QueryGetMarketRequest,
    QueryGetMarketResponse, QueryGetOrderByExternalIdRequest, QueryGetOrderByExternalIdResponse,
    QueryGetOrderRequest, QueryGetOrderResponse, QueryGetOwnerOrdersRequest,
    QueryGetOwnerOrdersResponse, QueryOrderFeeCalcRequest, QueryOrderFeeCalcResponse,
    QueryParamsRequest, QueryParamsResponse, QueryValidateCreateMarketRequest,
    QueryValidateCreateMarketResponse, QueryValidateManageFeesRequest,
    QueryValidateManageFeesResponse, QueryValidateMarketRequest, QueryValidateMarketResponse,
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
        pub market_update_user_settle: MsgMarketUpdateUserSettleRequest["/provenance.exchange.v1.MsgMarketUpdateUserSettleRequest"] => MsgMarketUpdateUserSettleResponse
    }

    fn_execute! {
        pub market_manage_permissions: MsgMarketManagePermissionsRequest["/provenance.exchange.v1.MsgMarketManagePermissionsRequest"] => MsgMarketManagePermissionsResponse
    }

    fn_execute! {
        pub market_manage_req_attrs: MsgMarketManageReqAttrsRequest["/provenance.exchange.v1.MsgMarketManageReqAttrsRequest"] => MsgMarketManageReqAttrsResponse
    }

    fn_execute! {
        pub gov_create_market: MsgGovCreateMarketRequest["/provenance.exchange.v1.MsgGovCreateMarketRequest"] => MsgGovCreateMarketResponse
    }

    fn_execute! {
        pub gov_manage_fees: MsgGovManageFeesRequest["/provenance.exchange.v1.MsgGovManageFeesRequest"] => MsgGovManageFeesResponse
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
        pub query_get_asset_orders_request ["/provenance.exchange.v1.Query/GetAssetOrders"]: QueryGetAssetOrdersRequest => QueryGetAssetOrdersResponse
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
