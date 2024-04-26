# provwasm-test-tube

[![provwasm-test-tube on crates.io](https://img.shields.io/crates/v/provwasm-test-tube.svg)](https://crates.io/crates/provwasm-test-tube) [![Docs](https://docs.rs/provwasm-test-tube/badge.svg)](https://docs.rs/provwasm-test-tube)

CosmWasm x ProvWasm integration testing library that, unlike `cw-multi-test`, it allows you to test your ProvWasm
contract against real chain's logic instead of mocks.

## Table of Contents

- [Getting Started](#getting-started)
- [Debugging](#debugging)
- [Using Module Wrapper](#using-module-wrapper)
- [Custom Module Wrapper](#custom-module-wrapper)
- [Versioning](#versioning)

## Getting Started

To demonstrate how `provwasm-test-tube` works, let use simple example
contract: [marker](https://github.com/provenance-io/provwasm/contracts/marker).

Here is how to setup the test:

```rust
use cosmwasm_std::Coin;
use provwasm_test_tube::ProvwasmTestApp;

// create new provenance appchain instance.
let app = ProvwasmTestApp::new();

// create new account with initial funds
let accs = app
    .init_accounts(
        &[
            Coin::new(1_000_000_000_000, "nhash"),
        ],
        2,
    )?;

let admin = &accs[0];
let new_admin = &accs[1];
```

Now we have the appchain instance and accounts that have some initial balances and can interact with the appchain.
This does not run Docker instance or spawning external process, it just loads the appchain's code as a library and
creates an in-memory instance.

Note that `init_accounts` is a convenience function that creates multiple accounts with the same initial balance.
If you want to create just one account, you can use `init_account` instead.

```rust
use cosmwasm_std::Coin;
use provwasm_test_tube::ProvwasmTestApp;

let app = ProvwasmTestApp::new();

let account = app.init_account(&[
    Coin::new(1_000_000_000_000, "nhash"),
])?;
```

Now if we want to test a provwasm contract, we need to

- build the wasm file
- store code
- instantiate

Then we can start interacting with our contract

```rust
use cosmwasm_std::{Coin, Uint128};  
use provwasm_test_tube::wasm::Wasm;  
use provwasm_test_tube::{Account, ProvwasmTestApp};  
  
use marker::msg::{ExecuteMsg, InitMsg, QueryMsg};  
use marker::types::Marker;

let app = ProvwasmTestApp::default();
let accs = app.init_accounts(&[Coin::new(100_000_000_000_000, "nhash")], 1)?;
let admin = &accs[0];
  
let wasm = Wasm::new(&app);
let wasm_byte_code = std::fs::read("../contracts/marker/artifacts/marker.wasm").unwrap();
let store_res = wasm.store_code(&wasm_byte_code, None, admin);
let code_id = store_res?.data.code_id;
```

Now let's execute the contract and verify that the contract's state is updated properly.

```rust
use cosmwasm_std::{Coin, Uint128};
use provwasm_test_tube::wasm::Wasm;
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, RunnerError};

use marker::msg::{ExecuteMsg, InitMsg, QueryMsg};
use marker::types::Marker;

#[test]
fn create_and_withdraw() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::default();
    let accs = app.init_accounts(&[Coin::new(100_000_000_000_000, "nhash")], 1)?;
    let admin = &accs[0];

    let wasm = Wasm::new(&app);
    let wasm_byte_code = std::fs::read("../contracts/marker/artifacts/marker.wasm").unwrap();
    let store_res = wasm.store_code(&wasm_byte_code, None, admin);
    let code_id = store_res?.data.code_id;
    assert_eq!(code_id, 1);

    // let init_admins = vec![admin.address()];
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InitMsg {
                name: "marker-test.sc.pb".to_string(),
            },
            Some(&admin.address()),
            None,
            &[],
            admin,
        )?
        .data
        .address;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Create {
            supply: Uint128::new(100),
            denom: "spy".into(),
            allow_forced_transfer: false,
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::GrantAccess {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Finalize {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Activate {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Withdraw {
            amount: Uint128::new(20),
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    let marker = wasm.query::<QueryMsg, Marker>(
        &contract_addr,
        &QueryMsg::GetByDenom {
            denom: "spy".into(),
        },
    )?;

    assert_eq!(marker.marker_account.denom, "spy");

    Ok(())
}

```

## Debugging

In your contract code, if you want to debug, you can
use [`deps.api.debug(..)`](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/trait.Api.html#tymethod.debug) which will
prints the debug message to stdout. `wasmd` disabled this by default but `ProvwasmTestApp` allows stdout emission so
that you can debug your smart contract while running tests.

## Using Module Wrapper

In some cases, you might want interact directly with appchain logic to setup the environment or query appchain's state.
Module wrappers provides convenient functions to interact with the appchain's module.

Let's try interact with the `Marker` module:

```rust
use std::convert::TryFrom;

use cosmwasm_std::{Coin, Uint128};
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, RunnerError};

use provwasm_std::types::provenance::marker::v1::{
    Access, AccessGrant, MarkerAccount, MarkerStatus, MarkerType, MsgAddMarkerRequest,
    QueryMarkerRequest,
};

#[test]
fn create_and_withdraw() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::default();
    let accs = app
        .init_accounts(&[Coin::new(100_000_000_000_000, "nhash")], 1)
        .unwrap();
    let admin = &accs[0];

    let marker_module = provwasm_test_tube::module::marker::Marker::new(&app);
    let add_response = marker_module.add_marker(
        MsgAddMarkerRequest {
            amount: Some(
                Coin {
                    amount: Uint128::new(100),
                    denom: "spy".to_string(),
                }
                .into(),
            ),
            manager: admin.address(),
            from_address: admin.address(),
            status: MarkerStatus::Proposed.into(),
            marker_type: MarkerType::Coin.into(),
            access_list: vec![AccessGrant {
                address: admin.address(),
                permissions: vec![
                    Access::Admin.into(),
                    Access::Burn.into(),
                    Access::Deposit.into(),
                    Access::Delete.into(),
                    Access::Mint.into(),
                    Access::Withdraw.into(),
                ],
            }],
            supply_fixed: false,
            allow_governance_control: false,
            allow_forced_transfer: false,
            required_attributes: vec![],
            usd_cents: 0,
            volume: 0,
            usd_mills: 0,
        },
        admin,
    )?;

    let marker_response = marker_module.query_marker(&QueryMarkerRequest {
        id: "spy".to_string(),
    })?;

    assert_eq!(
        MarkerAccount::try_from(marker_response.marker.unwrap())
            .unwrap()
            .denom,
        "spy"
    );
    
    Ok(())
}

```

## Custom Module Wrapper

You might not find wrapper you want to use or the provided wrapper is too verbose. Good new is, it's trivial to create
your own wrapper easily.

Here is how you can redefine `Hold` module wrapper as a library user:

```rust
use provwasm_test_tube::{fn_execute, fn_query, Module, Runner};

use provwasm_std::types::provenance::hold::v1::{
    AccountHold, GetAllHoldsRequest, GetAllHoldsResponse, GetHoldsRequest, GetHoldsResponse,
};

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
```

If the macro generated function is not good enough for you, you can write your own function manually.
See [module directory](src/module) for more inspiration.
