# nfcfyi

[![crates.io](https://img.shields.io/crates/v/nfcfyi)](https://crates.io/crates/nfcfyi)
[![docs.rs](https://docs.rs/nfcfyi/badge.svg)](https://docs.rs/nfcfyi)

Async Rust client for the [NFCFYI](https://nfcfyi.com) API. Look up NFC chips, NDEF record types, operating modes, ISO 14443/15693 standards, and contactless protocols.

## Install

```toml
[dependencies]
nfcfyi = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use nfcfyi::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let results = client.search("ntag").await?;
    println!("Found {} results", results.total);
    Ok(())
}
```

## API Methods

| Method | Description |
|--------|-------------|
| `search(query)` | Search chips, standards, and glossary |
| `chip(slug)` | Get NFC chip details |
| `chip_family(slug)` | Get chip family details |
| `standard(slug)` | Get standard details |
| `operating_mode(slug)` | Get operating mode details |
| `ndef_type(slug)` | Get NDEF type details |
| `use_case(slug)` | Get use case details |
| `glossary_term(slug)` | Get glossary term definition |
| `compare(slug_a, slug_b)` | Compare two NFC chips |
| `random()` | Get a random NFC chip |

All methods are async and return `Result<T, NfcFyiError>`.

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| Python | [nfcfyi](https://pypi.org/project/nfcfyi/) | `pip install nfcfyi` |
| TypeScript | [nfcfyi](https://www.npmjs.com/package/nfcfyi) | `npm install nfcfyi` |
| Go | [nfcfyi-go](https://pkg.go.dev/github.com/fyipedia/nfcfyi-go) | `go get github.com/fyipedia/nfcfyi-go` |
| Rust | [nfcfyi](https://crates.io/crates/nfcfyi) | `cargo add nfcfyi` |
| Ruby | [nfcfyi](https://rubygems.org/gems/nfcfyi) | `gem install nfcfyi` |

## Code FYI Family

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode symbologies & standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types & encoding |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips & protocols |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags & readers |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart card platforms |

## License

MIT
