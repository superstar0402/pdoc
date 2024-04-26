# provenance-test-tube

---

`provwasm-test-tube` is a generic library for building testing environments
for [ProvWasm](https://github.com/provenance-io/provwasm) smart contracts. It allows
you to test your smart contract logic against the actual Cosmos SDK chain's logic, which is written in Go, using Rust.
This eliminates the need to write Go code or learn Go in order to test your smart contracts against Provenance
Blockchain.

This `provwasm-test-tube` repository is a fork
of [`osmosis-test-tube`](https://github.com/osmosis-labs/test-tube/tree/main/packages/osmosis-test-tube), customized
for [Provenance](https://github.com/provenance-io/provenance).

## Features

- Test your ProvWasm smart contracts using Rust without the need to write Go code or learn Go
- Test against the actual Provenance chain's logic

## Usage

See [the documentation](packages/provwasm-test-tube/README.md)

## What about `cw-multi-test`?

You might want to just use `cw-multi-test` if your contract does not interact with Provenance's custom modules.
`cw-multi-test` is faster since it does not need to run the chain code or build and upload `.wasm` file, but it does not
test your contract against the actual chain's logic and rely on simulation which only some basic modules are
implemented.

So if your contract just interact with common modules like Bank, Staking, and Distribution, `cw-multi-test` is enough.
But if it's interacting with custom modules, you should use `provwasm-test-tube`.

## Known Issues

### linking with `cc` failed on Mac M1/M2 (`arm64`) https://github.com/osmosis-labs/test-tube/issues/28

Check your `go version` if it's `darwin/arm64`, it might be `darwin/amd64` and your local build has been working
but it confuses the linker.

If you don't want to reinstall, go you opt for cross-compile.

```sh
cargo clean
export GOOS="darwin"
export GOARCH="arm64"
export CGO_ENABLED=1
```

But we would installing go version that match your architecture makes more sense.

## Contributing

We welcome contributions to `provwasm-test-tube`! If you find a bug or have an idea for a new feature, please open an
issue or
submit a pull request.
