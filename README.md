# cargo-xtask-workspace

A template for Rust projects using Cargo Workspaces  and the `xtask` pattern with an `./xtask` + `./crates/*` layout.

It also includes some opinionated boilerplate for what some might view as a typical Cargo workspace:

- It includes a handful of dependencies in the root Cargo.toml `[workspace.dependencies]` at `*` version (these should get pinned after applying the template):
  - `anyhow` and `thiserror` for managing error types and error reporting
  - `tracing` as the log facade; even if there isn't any async code, `tracing` is a great structured logging implementation.
  - `env_logger` as the default log implementation
  - `clap` for building CLI's
- Rust Edition is 2021
- Rust Version is not pinned, for flexibility, but you should probably pin it
