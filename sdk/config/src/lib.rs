#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <fullname>Config</fullname>
//! <p>Config provides a way to keep track of the configurations
//! of all the Amazon Web Services resources associated with your Amazon Web Services account. You can
//! use Config to get the current and historical configurations of
//! each Amazon Web Services resource and also to get information about the relationship
//! between the resources. An Amazon Web Services resource can be an Amazon Compute
//! Cloud (Amazon EC2) instance, an Elastic Block Store (EBS) volume, an
//! elastic network Interface (ENI), or a security group. For a complete
//! list of resources currently supported by Config, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources">Supported Amazon Web Services resources</a>.</p>
//! <p>You can access and manage Config through the Amazon Web Services Management
//! Console, the Amazon Web Services Command Line Interface (Amazon Web Services CLI), the Config
//! API, or the Amazon Web Services SDKs for Config. This reference guide contains
//! documentation for the Config API and the Amazon Web Services CLI commands that
//! you can use to manage Config. The Config API uses the
//! Signature Version 4 protocol for signing requests. For more
//! information about how to sign a request with this protocol, see
//! <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature
//! Version 4 Signing Process</a>. For detailed information
//! about Config features and their associated actions or commands,
//! as well as how to work with Amazon Web Services Management Console, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/WhatIsConfig.html">What Is Config</a> in the <i>Config Developer
//! Guide</i>.</p>

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
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("configservice", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;