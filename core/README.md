# SW4RM-RS Core

## About This Crate

This crate is the central core of OpenAPI parsing. It is compatible with OpenAPI v2, v3, and v3.1, in both JSON and 
YAML formats. It provides all the types required to manipulate and use any of these OpenAPI specifications within a 
Rust context, as well as helper functions to read in and parse OpenAPI specifications.

## Why This Crate?

This crate is inspired from [OAS3](https://github.com/x52dev/oas3-rs) and
[openapi](https://github.com/softprops/openapi), but attempts to do certain tasks better. Specifically, it tries to:

- Maintain compatibility with all in-use versions of OpenAPI, utilizing a single structure.
- Provide convenience functions to resolve and search for items within the specification tree.
- Provide accessibility and insight to developers in Rust for using OpenAPI.

## Compatibility Table

Compare this library with popular alternatives across the ecosystem

|                                 Library                                 |     OpenAPI 2      |     OpenAPI 3      |    OpenAPI 3.1     | Helper Functionality | Reference Resolution |
|:-----------------------------------------------------------------------:|:------------------:|:------------------:|:------------------:|:--------------------:|:--------------------:|
|                                SW4RM-RS                                 | :white_check_mark: | :white_check_mark: | :white_check_mark: |  :white_check_mark:  |  :white_check_mark:  |
|                [OAS3](https://github.com/x52dev/oas3-rs)                |        :x:         | :white_check_mark: |        :x:         |         :x:          |         :x:          |
|        [softprops/openapi](https://github.com/softprops/openapi)        | :white_check_mark: | :white_check_mark: |        :x:         |      :warning:       |  :white_check_mark:  |
| [oxidecomputer/progenitor](https://github.com/oxidecomputer/progenitor) |        :x:         | :white_check_mark: |        :x:         |      :warning:       |      :warning:       |

## Installation

> [!TIP]
> It is generally _not_ recommended to use this crate unless you are directly manipulating OpenAPI specifications.

Install this crate from crates.io:
```shell
cargo install sw4rm-rs
```

or add it to your `Cargo.toml`
```toml
[dependencies]
sw4rm-rs = "0.2.0"
```

## Usage

To read in a specification `Spec` type, two methods are available:
```rust
sw4rm_rs::from_path("path/to/swagger");
// OR
sw4rm_rs::from_reader(Reader);
```