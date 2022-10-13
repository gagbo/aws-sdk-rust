# aws-sdk-migrationhuborchestrator

**Please Note: The SDK is currently in Developer Preview and is intended strictly for
feedback purposes only. Do not use this SDK for production workloads.**

This API reference provides descriptions, syntax, and other details about each of the actions and data types for AWS Migration Hub Orchestrator. he topic for each action shows the API request parameters and the response. Alternatively, you can use one of the AWS SDKs to access an API that is tailored to the programming language or platform that you're using.

## Getting Started

> Examples are available for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-migrationhuborchestrator` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = "0.49.0"
aws-sdk-migrationhuborchestrator = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

Then in code, a client can be created with the following:

```rust
use aws_sdk_migrationhuborchestrator as migrationhuborchestrator;

#[tokio::main]
async fn main() -> Result<(), migrationhuborchestrator::Error> {
    let config = aws_config::load_from_env().await;
    let client = migrationhuborchestrator::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}
```

See the [client documentation](https://docs.rs/aws-sdk-migrationhuborchestrator/latest/aws_sdk_migrationhuborchestrator/client/struct.Client.html)
for information on what calls can be made, and the inputs and outputs for each of those calls.

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the
[Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
additional sections for the guide by opening an issue and describing what you are trying to do.

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) – For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)

## License

This project is licensed under the Apache-2.0 License.
