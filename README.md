# nfcfyi

[![crates.io](https://img.shields.io/crates/v/nfcfyi.svg)](https://crates.io/crates/nfcfyi)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust client for the [NFCFYI](https://nfcfyi.com) REST API. NFC chips, NDEF. Uses `reqwest` for HTTP.

> **Explore at [nfcfyi.com](https://nfcfyi.com)** — interactive tools and comprehensive reference.

## Install

```toml
[dependencies]
nfcfyi = "0.1"
```

## Quick Start

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = nfcfyi::Client::new();
    let result = client.search("query")?;
    println!("{:?}", result);
    Ok(())
}
```

## Also Available

| Platform | Install | Link |
|----------|---------|------|
| **Python** | `pip install nfcfyi` | [PyPI](https://pypi.org/project/nfcfyi/) |
| **npm** | `npm install nfcfyi` | [npm](https://www.npmjs.com/package/nfcfyi) |
| **Go** | `go get github.com/fyipedia/nfcfyi-go` | [pkg.go.dev](https://pkg.go.dev/github.com/fyipedia/nfcfyi-go) |
| **Rust** | `cargo add nfcfyi` | [crates.io](https://crates.io/crates/nfcfyi) |
| **Ruby** | `gem install nfcfyi` | [rubygems](https://rubygems.org/gems/nfcfyi) |


## Links

- **Site**: [nfcfyi.com](https://nfcfyi.com)
- **API**: [nfcfyi.com/api/v1/](https://nfcfyi.com/api/v1/)
- **OpenAPI**: [nfcfyi.com/api/v1/schema/](https://nfcfyi.com/api/v1/schema/)
- **Glossary**: [nfcfyi.com/glossary/](https://nfcfyi.com/glossary/)
- **Guides**: [nfcfyi.com/guide/](https://nfcfyi.com/guide/)
- **Tools**: [nfcfyi.com/tools/](https://nfcfyi.com/tools/)
Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

## Tag FYI Family

Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode formats, EAN, UPC, ISBN standards |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy, GATT, beacons |
| **NFCFYI** | [nfcfyi.com](https://nfcfyi.com) | NFC chips, tag types, NDEF, standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types, versions, encoding modes |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags, frequency bands, standards |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart cards, EMV, APDU, Java Card |

## Embed Widget

Embed [NFCFYI](https://nfcfyi.com) widgets on any website with [nfcfyi-embed](https://widget.nfcfyi.com):

```html
<script src="https://cdn.jsdelivr.net/npm/nfcfyi-embed@1/dist/embed.min.js"></script>
<div data-nfcfyi="entity" data-slug="example"></div>
```

Zero dependencies · Shadow DOM · 4 themes (light/dark/sepia/auto) · [Widget docs](https://widget.nfcfyi.com)

## License

MIT
