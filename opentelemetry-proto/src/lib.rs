#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! This crate contains generated files from [opentelemetry-proto](https://github.com/open-telemetry/opentelemetry-proto)
//! repository and transformation between types from generated files and types defined in [opentelemetry](https://github.com/open-telemetry/opentelemetry-rust/tree/main/opentelemetry)
//!
//! Based on the build tool needed, users can choose to generate files using [tonic](https://github.com/hyperium/tonic)
//! or [grpcio](https://github.com/tikv/grpc-rs).
//!
//!
//! # Feature flags
//! `Opentelemetry-proto` includes a set of feature flags to avoid pull in unnecessary dependencies.
//! The following is the full list of currently supported features:
//!
//! ## Signals
//! - `trace`: generate types that used in traces. Currently supports `gen-tonic` or `gen-grpcio`.
//! - `metrics`: generate types that used in metrics. Currently supports `gen-tonic`.
//! - `logs`: generate types that used in logs. Currently supports `gen-tonic`.
//! - `zpages`: generate types that used in zPages. Currently only tracez related types will be generated. Currently supports `gen-tonic` or `gen-grpcio`.
//!
//! ## Creates used to generate files
//! - `gen-tonic`: generate rs files using [tonic](https://github.com/hyperium/tonic) and [prost](https://github.com/tokio-rs/prost).
//! - `gen-grpcio`: generate rs files using [grpcio](https://github.com/tikv/grpc-rs).
//!
//! ## Misc
//! - `full`: enabled all features above.
//!
//! By default, no feature is enabled.

// proto mod contains file generated by protobuf or other build tools.
// we shouldn't manually change it. Thus skip format and lint check.
#[rustfmt::skip]
#[allow(warnings)]
#[doc(hidden)]
mod proto;

#[cfg(feature = "gen-grpcio-messages")]
pub use proto::grpcio;

#[cfg(feature = "gen-tonic-messages")]
pub use proto::tonic;

pub mod transform;

#[cfg(feature = "gen-grpcio-messages")]
pub use prost;
