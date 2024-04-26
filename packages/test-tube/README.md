# test-tube

[![test-tube on crates.io](https://img.shields.io/crates/v/test-tube.svg)](https://crates.io/crates/test-tube) [![Docs](https://docs.rs/test-tube/badge.svg)](https://docs.rs/test-tube)

`provwasm-test-tube` is a generic library for building testing environments
for [ProvWasm](https://github.com/provenance-io/provwasm) smart contracts. It allows you to test your smart contract
logic against the actual Provenance/Cosmos SDK chain's logic, which is written in Go, using Rust. This eliminates the
need to write Go code or learn Go in order to test your smart contracts against the network.
