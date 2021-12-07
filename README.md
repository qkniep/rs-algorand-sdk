<div align="center">

# rs-algorand-sdk

A Rust library for interacting with Algorand nodes.
This library includes:
* HTTP clients for the algod (agreement) and kmd (key management) APIs
* Standalone functionality for interacting with the Algorand protocol, including transaction signing, message encoding, etc.

[![Build Status](https://img.shields.io/circleci/build/github/qkniep/rs-algorand-sdk/main?token=86739a8e33bf4ab2812b9771d04a7585fa90f80c&style=for-the-badge&logo=circleci)](https://app.circleci.com/pipelines/github/qkniep/rs-algorand-sdk)
[![Test Coverage](https://img.shields.io/codecov/c/github/qkniep/rs-algorand-sdk?label=test%20coverage&logo=codecov&style=for-the-badge)](https://codecov.io/gh/qkniep/rs-algorand-sdk)
[![crates.io](https://img.shields.io/crates/v/rs-algorand-sdk?label=crates.io&style=for-the-badge)](https://crates.io/rs-algorand-sdk)
[![docs.rs](https://img.shields.io/docsrs/rs-algorand-sdk?style=for-the-badge)](https://docs.rs/rs-algorand-sdk)

</div>
<br>

## Example

```rust
use algosdk::{AlgodClient, KmdClient};

fn main() {
    let algod_address = "http://localhost:8080";
    let algod_token = "contents-of-algod.token";
    let kmd_address = "http://localhost:7833";
    let kmd_token = "contents-of-kmd.token";

    let algod_client = AlgodClient::new(algod_address, algod_token);
    let kmd_client = KmdClient::new(kmd_address, kmd_token);

    println!("Algod versions: {:?}", algod_client.versions().unwrap().versions);
    println!("Kmd versions: {:?}", kmd_client.versions().unwrap().versions);
}
```

## Other Resources

* [Algorand Developer Portal](https://developer.algorand.org)
* Algorand SDKs in other languages:
	- [Python](https://developer.algorand.org/docs/sdks/python)
	- [JavaScript](https://developer.algorand.org/docs/sdks/javascript)
	- [Go](https://developer.algorand.org/docs/sdks/go)
	- [Java](https://developer.algorand.org/docs/sdks/java)

## License

Released under the [MIT License](LICENSE).
