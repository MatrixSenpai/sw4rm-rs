# SW4RM-RS

A crate to parse OpenAPI v2, v3, and v3.1 specs and generate Rust APIs from specs.

### Why this crate?
[OAS3](https://github.com/x52dev/oas3-rs) and [openapi](https://github.com/softprops/openapi) have odd features, don't 
support certain versions of OpenAPI, and can be hard to work with. They also are not updated frequently, and, in
openapi's case, haven't seen an update in 3 years.

[openapi-generator](https://github.com/OpenAPITools/openapi-generator) isn't written in Rust, doesn't generate Rust
models in a Rusty way, and _only_ supports v2.

This crate aims to solve both these issues, by providing the user with a model to parse OpenAPI specs of all versions
in one simple model, and a way to generate models through a cargo command or in a build script at compile time.

### Roadmap
- [x] Cross-Spec support for v2 and v3
- [ ] Parse and create Rust models using `syn`
  - [x] Unoptimized parsing
  - [ ] Create mod file
  - [ ] Write files
  - [ ] Types imports
- [ ] Parse and create Rust apis using `syn`
  - [ ] Allow the user to specify what framework to utilize underneath
  - [ ] Allow the user to specify other options, including additional traits to derive on models
  - [ ] Allow the user to provide a base struct to extend rather than generating one internally
- [ ] More complete documentation and readme with examples for crates.io