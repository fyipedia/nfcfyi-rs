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

Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

## License

MIT
