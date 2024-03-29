# Change Log

All notable changes to this project will be documented in this file
automatically by Versionist. DO NOT EDIT THIS FILE MANUALLY!
This project adheres to [Semantic Versioning](http://semver.org/).

# v0.2.28
## (2023-06-13)

* Formatting adjustments in How It Works README section [Zahari Petkov]

# v0.2.27
## (2023-06-12)

* How It Works README section [Zahari Petkov]

# v0.2.26
## (2023-05-24)

* Remove duplicated log lines and add a missing one [Zahari Petkov]

# v0.2.25
## (2023-05-24)

* Improve log messages ordering [Zahari Petkov]

# v0.2.24
## (2023-05-23)

* Disable printing log level in debug mode [Zahari Petkov]

# v0.2.23
## (2023-05-22)

* Bump clap to v4 [Zahari Petkov]

# v0.2.22
## (2023-05-22)

* Bump env_logger to v0.10 [Zahari Petkov]

# v0.2.21
## (2023-04-26)

* Make a `safe` counterpart of each fs function [Zahari Petkov]
* Move commit_md5sum_file and verify_checksum to checksum.rs [Zahari Petkov]

# v0.2.20
## (2023-04-21)

* Add --unsafe/-u CLI argument [Zahari Petkov]
* Handle low disk space scenarios by resorting to unsafe write/rename [Zahari Petkov]

# v0.2.19
## (2023-02-16)

* Switch repo type to `rust-module` to enable auto cargo versioning [Zahari Petkov]

# v0.2.18
## (2023-02-09)

* Fix repo.yml to be proper yaml [Kyle Harding]

# v0.2.17
## (2023-02-03)

* Add repo yml to fix versionist errors [Kyle Harding]

# v0.2.16
## (2023-02-02)

* Fix flowzone Rust toolchain to 1.67 [Felipe Lalanne]

# v0.2.15
## (2023-02-01)

* Fix clippy warnings introduced in Rust 1.67 [Zahari Petkov]

# v0.2.14
## (2022-11-18)

* Use flowzone instead of custom CI configuration [Felipe Lalanne]

# v0.2.13
## (2022-11-16)

* Switch to rust-toolchain@master [Zahari Petkov]

# v0.2.12
## (2022-11-16)

* Replace actions-rs/cargo build with direct cross usage [Zahari Petkov]

# v0.2.11
## (2022-11-16)

* Switch CI to dtolnay/rust-toolchain [Zahari Petkov]

# v0.2.10
## (2022-11-16)

* Fix clippy lints on Rust v1.65 [Zahari Petkov]

# v0.2.9
## (2022-08-23)

* Enable verbose clippy warnings [Zahari Petkov]

# v0.2.8
## (2022-08-23)

* Use Rust 2021 Edition [Zahari Petkov]

# v0.2.7
## (2022-08-18)

* Use name and email in package.authors in Cargo.toml [Zahari Petkov]

# v0.2.6
## (2022-08-18)

* Add package.repository in Cargo.toml [Zahari Petkov]

# v0.2.5
## (2022-08-18)

* Restructure write entry logging [Zahari Petkov]

# v0.2.4
## (2022-08-18)

* Add debug CLI argument [Zahari Petkov]

# v0.2.3
## (2022-08-13)

* Transition to clap v3 [Zahari Petkov]

# v0.2.2
## (2022-08-10)

* Simpler log lines [Zahari Petkov]

# v0.2.1
## (2022-08-10)

* Simpler log formatting [Zahari Petkov]

# v0.2.0
## (2022-08-09)

* Use AsRef<Path> on the crate's public API only [Zahari Petkov]
* Update dependencies [Zahari Petkov]

# v0.1.10
## (2022-06-29)

* Do not use custom release profile for CI builds [Zahari Petkov]

# v0.1.9
## (2022-06-22)

* Use a separate release profile for CI static builds [Zahari Petkov]

# v0.1.8
## (2022-06-22)

* Cleanup files in the root folder [Zahari Petkov]

# v0.1.7
## (2022-06-22)

* Change balena project type to rust-public-crate [Zahari Petkov]

# v0.1.6
## (2022-06-20)

* Remove panic="abort" as it breaks meta-rust [Zahari Petkov]

# v0.1.5
## (2022-06-10)

* Refine when to trigger builds [Zahari Petkov]

# v0.1.4
## (2022-06-10)

* Create a GitHub workflow [Zahari Petkov]

# v0.1.3
## (2022-05-13)

* README update [Zahari Petkov]

# v0.1.2
## (2022-05-13)

* Copy subcommand [Zahari Petkov]

# v0.1.1
## (2022-05-12)

* Write operation receives content from stdin [Zahari Petkov]

# v0.1.0
## (2022-05-12)

* Read operation working on top of bytes instead of string [Zahari Petkov]

# v0.0.10
## (2022-05-11)

* Compatibility with Rust 1.33 [Zahari Petkov]

# v0.0.9
## (2022-05-11)

* Compatibility with Rust 1.36 [Zahari Petkov]

# v0.0.8
## (2022-05-11)

* Downgrade clap to 2.34 [Zahari Petkov]

# v0.0.7
## (2022-05-10)

* Regenerate Cargo.lock with older cargo version [Zahari Petkov]

# v0.0.6
## (2022-05-10)

* Switch to edition 2018 to support Rust v1.54.0 [Zahari Petkov]

# v0.0.5
## (2022-04-19)

* Update dependencies and Cargo.toml description [Zahari Petkov]

# v0.0.4
## (2022-04-19)

* Update README with multiline examples [Zahari Petkov]

# v0.0.3
## (2022-04-19)

* Include a README with usage and short introduction [Zahari Petkov]

# v0.0.2
## (2022-04-19)

* Add balena.yml [Zahari Petkov]
