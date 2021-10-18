#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <fullname>Amazon ElastiCache</fullname>
//! <p>Amazon ElastiCache is a web service that makes it easier to set up, operate,
//! and scale a distributed cache in the cloud.</p>
//! <p>With ElastiCache, customers get all of the benefits of a high-performance,
//! in-memory cache with less of the administrative burden involved in launching and managing a distributed cache.
//! The service makes setup, scaling,
//! and cluster failure handling much simpler than in a self-managed cache deployment.</p>
//! <p>In addition, through integration with Amazon CloudWatch,
//! customers get enhanced visibility into the key performance statistics
//! associated with their cache and can receive alarms if a part of their cache runs hot.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
mod query_ser;
mod rest_xml_wrapped_errors;
mod xml_deser;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("elasticache", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;