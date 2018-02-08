![banner](assets/banner.png)

# Libyobicash
[![Travis branch](https://img.shields.io/travis/yobicash/libyobicash/master.svg)](https://travis-ci.org/yobicash/libyobicash)
[![Coveralls github branch](https://img.shields.io/coveralls/github/yobicash/libyobicash/master.svg)](https://coveralls.io/github/yobicash/libyobicash?branch=master)
[![Crates.io](https://img.shields.io/crates/v/libyobicash.svg)](https://crates.io/crates/libyobicash)
[![Docs.rs](https://docs.rs/libyobicash/badge.svg)](https://docs.rs/libyobicash)
[![Crates.io](https://img.shields.io/crates/l/libyobicash.svg)]()

The [Yobicash](https://yobicash.org) cryptocurrency library.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [License](#license)
- [Contributing](#contributing)

## Install

To install libyobicash add in your Cargo.toml:

```toml
# Cargo.toml

[dependencies]
libyobicash = "^0.1"
```

and in the root of your crate:

```rust
//main.rs

extern crate libyobicash;
```

## Usage

Look at the [documentation](https://docs.rs/libyobicash) or at the tests for guidance.

```rust
// main.rs

// Bob wants to send some data to Alice through Yobicash and has found
// a node offering a good price for broadcasting the data.

// let plain = some super important secret.
// let alice_pk = Alice public key;
let bob_sk = SecretKey::random();
let bob_data = Data::new(bob_sk, alice_pk, &plain)?;

// let bob_coins = some spendable coins Bob owns that pay the fee
// required by the node;
// let bob = the output to the node that will broadcast Bob writes operation;

// One secret instance from Bob. Bob will use it to permit the network to
// delete his data.
let bob_instance = Scalar::random();

// The write operation containing the encrypted data that only
// Bob and Alice can read. After confirmation, it will be broadcasted
// to the network.
let bob_write = WriteOp::new(&bob_coins, &bob_data, bob_instance, &bob_fee)?;

println!(write_op.to_json()?);

// Now that the secret is not needed anymore, Bob and Alice agree that it
// can be erased from the dagchain. Bob finds a node with making a good offer
// to get rid of that data and broadcast the operation to the network.

// let node_coins = some spendable coins the node owns that pay Bob's fee;
// let node_fee = the fee promised to Bob by the node;

// Bob generates the proof of the bob_write witness instance to send to the node
// using the fee that the node will use and bob_write itself.
let bob_proof = DeleteOp::proof(&bob_write, &bob_instance, &node_fee)?;

// The node creates a delete operation with Bob's witness.
// The node will delete bob_writes' Data after node_delete get confirmed.
// The other nodes of the network will do the same.
let node_delete = DeleteOp::new(&node_coins, &bob_write, &bob_proof, &node_fee)?;

println!(node_delete.to_json()?);
```

## Maintainers

[@chritchens](https://github.com/chritchens)

## License

This project is license under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in libyobicash by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
