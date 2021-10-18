// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`DescribeGroupInput`](crate::input::DescribeGroupInput)
pub mod describe_group_input {
    /// A builder for [`DescribeGroupInput`](crate::input::DescribeGroupInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) identity_store_id: std::option::Option<std::string::String>,
        pub(crate) group_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The globally unique identifier for the identity store, such as
        /// <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and
        /// <code>1234567890</code> is a randomly generated string that contains number and lower
        /// case letters. This value is generated at the time that a new identity store is
        /// created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.identity_store_id = Some(input.into());
            self
        }
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.identity_store_id = input;
            self
        }
        /// <p>The identifier for a group in the identity store.</p>
        pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.group_id = Some(input.into());
            self
        }
        pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.group_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeGroupInput`](crate::input::DescribeGroupInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::DescribeGroupInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::DescribeGroupInput {
                identity_store_id: self.identity_store_id,
                group_id: self.group_id,
            })
        }
    }
}
#[doc(hidden)]
pub type DescribeGroupInputOperationOutputAlias = crate::operation::DescribeGroup;
#[doc(hidden)]
pub type DescribeGroupInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DescribeGroupInput {
    /// Consumes the builder and constructs an Operation<[`DescribeGroup`](crate::operation::DescribeGroup)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DescribeGroup,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::DescribeGroupInput,
            output: &mut String,
        ) -> Result<(), smithy_http::operation::BuildError> {
            write!(output, "/").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::DescribeGroupInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::DescribeGroupInput,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/x-amz-json-1.1",
            );
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSIdentityStore.DescribeGroup",
            );
            Ok(builder)
        }
        let properties = smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_describe_group(&self)
            .map_err(|err| smithy_http::operation::BuildError::SerializationError(err.into()))?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = smithy_http::operation::Request::from_parts(
            request.map(smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            smithy_http::operation::Operation::new(request, crate::operation::DescribeGroup::new())
                .with_metadata(smithy_http::operation::Metadata::new(
                    "DescribeGroup",
                    "identitystore",
                ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`DescribeGroupInput`](crate::input::DescribeGroupInput)
    pub fn builder() -> crate::input::describe_group_input::Builder {
        crate::input::describe_group_input::Builder::default()
    }
}

/// See [`DescribeUserInput`](crate::input::DescribeUserInput)
pub mod describe_user_input {
    /// A builder for [`DescribeUserInput`](crate::input::DescribeUserInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) identity_store_id: std::option::Option<std::string::String>,
        pub(crate) user_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The globally unique identifier for the identity store, such as
        /// <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and
        /// <code>1234567890</code> is a randomly generated string that contains number and lower
        /// case letters. This value is generated at the time that a new identity store is
        /// created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.identity_store_id = Some(input.into());
            self
        }
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.identity_store_id = input;
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_id = Some(input.into());
            self
        }
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeUserInput`](crate::input::DescribeUserInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::DescribeUserInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::DescribeUserInput {
                identity_store_id: self.identity_store_id,
                user_id: self.user_id,
            })
        }
    }
}
#[doc(hidden)]
pub type DescribeUserInputOperationOutputAlias = crate::operation::DescribeUser;
#[doc(hidden)]
pub type DescribeUserInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DescribeUserInput {
    /// Consumes the builder and constructs an Operation<[`DescribeUser`](crate::operation::DescribeUser)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DescribeUser,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::DescribeUserInput,
            output: &mut String,
        ) -> Result<(), smithy_http::operation::BuildError> {
            write!(output, "/").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::DescribeUserInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::DescribeUserInput,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/x-amz-json-1.1",
            );
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSIdentityStore.DescribeUser",
            );
            Ok(builder)
        }
        let properties = smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_describe_user(&self)
            .map_err(|err| {
            smithy_http::operation::BuildError::SerializationError(err.into())
        })?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = smithy_http::operation::Request::from_parts(
            request.map(smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            smithy_http::operation::Operation::new(request, crate::operation::DescribeUser::new())
                .with_metadata(smithy_http::operation::Metadata::new(
                    "DescribeUser",
                    "identitystore",
                ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`DescribeUserInput`](crate::input::DescribeUserInput)
    pub fn builder() -> crate::input::describe_user_input::Builder {
        crate::input::describe_user_input::Builder::default()
    }
}

/// See [`ListGroupsInput`](crate::input::ListGroupsInput)
pub mod list_groups_input {
    /// A builder for [`ListGroupsInput`](crate::input::ListGroupsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) identity_store_id: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) filters: std::option::Option<std::vec::Vec<crate::model::Filter>>,
    }
    impl Builder {
        /// <p>The globally unique identifier for the identity store, such as
        /// <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and
        /// <code>1234567890</code> is a randomly generated string that contains number and lower
        /// case letters. This value is generated at the time that a new identity store is
        /// created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.identity_store_id = Some(input.into());
            self
        }
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.identity_store_id = input;
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the
        /// <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results
        /// to return in one page. The length limit is 50 characters.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API
        /// operations. This value is generated by the identity store service. It is returned in the
        /// API response if the total results are more than the size of one page. This token is also
        /// returned when it is used in the API request to search for the next page.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        pub fn filters(mut self, input: impl Into<crate::model::Filter>) -> Self {
            let mut v = self.filters.unwrap_or_default();
            v.push(input.into());
            self.filters = Some(v);
            self
        }
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.filters = input;
            self
        }
        /// Consumes the builder and constructs a [`ListGroupsInput`](crate::input::ListGroupsInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::ListGroupsInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::ListGroupsInput {
                identity_store_id: self.identity_store_id,
                max_results: self.max_results,
                next_token: self.next_token,
                filters: self.filters,
            })
        }
    }
}
#[doc(hidden)]
pub type ListGroupsInputOperationOutputAlias = crate::operation::ListGroups;
#[doc(hidden)]
pub type ListGroupsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl ListGroupsInput {
    /// Consumes the builder and constructs an Operation<[`ListGroups`](crate::operation::ListGroups)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ListGroups,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::ListGroupsInput,
            output: &mut String,
        ) -> Result<(), smithy_http::operation::BuildError> {
            write!(output, "/").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::ListGroupsInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::ListGroupsInput,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/x-amz-json-1.1",
            );
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSIdentityStore.ListGroups",
            );
            Ok(builder)
        }
        let properties = smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_list_groups(&self)
            .map_err(|err| smithy_http::operation::BuildError::SerializationError(err.into()))?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = smithy_http::operation::Request::from_parts(
            request.map(smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            smithy_http::operation::Operation::new(request, crate::operation::ListGroups::new())
                .with_metadata(smithy_http::operation::Metadata::new(
                    "ListGroups",
                    "identitystore",
                ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`ListGroupsInput`](crate::input::ListGroupsInput)
    pub fn builder() -> crate::input::list_groups_input::Builder {
        crate::input::list_groups_input::Builder::default()
    }
}

/// See [`ListUsersInput`](crate::input::ListUsersInput)
pub mod list_users_input {
    /// A builder for [`ListUsersInput`](crate::input::ListUsersInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) identity_store_id: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) filters: std::option::Option<std::vec::Vec<crate::model::Filter>>,
    }
    impl Builder {
        /// <p>The globally unique identifier for the identity store, such as
        /// <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and
        /// <code>1234567890</code> is a randomly generated string that contains number and lower
        /// case letters. This value is generated at the time that a new identity store is
        /// created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.identity_store_id = Some(input.into());
            self
        }
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.identity_store_id = input;
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the
        /// <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results
        /// to return in one page. The length limit is 50 characters.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API
        /// operations. This value is generated by the identity store service. It is returned in the
        /// API response if the total results are more than the size of one page. This token is also
        /// returned when it is used in the API request to search for the next page.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        pub fn filters(mut self, input: impl Into<crate::model::Filter>) -> Self {
            let mut v = self.filters.unwrap_or_default();
            v.push(input.into());
            self.filters = Some(v);
            self
        }
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.filters = input;
            self
        }
        /// Consumes the builder and constructs a [`ListUsersInput`](crate::input::ListUsersInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::ListUsersInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::ListUsersInput {
                identity_store_id: self.identity_store_id,
                max_results: self.max_results,
                next_token: self.next_token,
                filters: self.filters,
            })
        }
    }
}
#[doc(hidden)]
pub type ListUsersInputOperationOutputAlias = crate::operation::ListUsers;
#[doc(hidden)]
pub type ListUsersInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl ListUsersInput {
    /// Consumes the builder and constructs an Operation<[`ListUsers`](crate::operation::ListUsers)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ListUsers,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::ListUsersInput,
            output: &mut String,
        ) -> Result<(), smithy_http::operation::BuildError> {
            write!(output, "/").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::ListUsersInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::ListUsersInput,
        ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/x-amz-json-1.1",
            );
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSIdentityStore.ListUsers",
            );
            Ok(builder)
        }
        let properties = smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_list_users(&self)
            .map_err(|err| smithy_http::operation::BuildError::SerializationError(err.into()))?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = smithy_http::operation::Request::from_parts(
            request.map(smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            smithy_http::operation::Operation::new(request, crate::operation::ListUsers::new())
                .with_metadata(smithy_http::operation::Metadata::new(
                    "ListUsers",
                    "identitystore",
                ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`ListUsersInput`](crate::input::ListUsersInput)
    pub fn builder() -> crate::input::list_users_input::Builder {
        crate::input::list_users_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListUsersInput {
    /// <p>The globally unique identifier for the identity store, such as
    /// <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and
    /// <code>1234567890</code> is a randomly generated string that contains number and lower
    /// case letters. This value is generated at the time that a new identity store is
    /// created.</p>
    pub identity_store_id: std::option::Option<std::string::String>,
    /// <p>The maximum number of results to be returned per request. This parameter is used in the
    /// <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results
    /// to return in one page. The length limit is 50 characters.</p>
    pub max_results: std::option::Option<i32>,
    /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API
    /// operations. This value is generated by the identity store service. It is returned in the
    /// API response if the total results are more than the size of one page. This token is also
    /// returned when it is used in the API request to search for the next page.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
    pub filters: std::option::Option<std::vec::Vec<crate::model::Filter>>,
}
impl std::fmt::Debug for ListUsersInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListUsersInput");
        formatter.field("identity_store_id", &self.identity_store_id);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &self.next_token);
        formatter.field("filters", &self.filters);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListGroupsInput {
    /// <p>The globally unique identifier for the identity store, such as
    /// <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and
    /// <code>1234567890</code> is a randomly generated string that contains number and lower
    /// case letters. This value is generated at the time that a new identity store is
    /// created.</p>
    pub identity_store_id: std::option::Option<std::string::String>,
    /// <p>The maximum number of results to be returned per request. This parameter is used in the
    /// <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results
    /// to return in one page. The length limit is 50 characters.</p>
    pub max_results: std::option::Option<i32>,
    /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API
    /// operations. This value is generated by the identity store service. It is returned in the
    /// API response if the total results are more than the size of one page. This token is also
    /// returned when it is used in the API request to search for the next page.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
    pub filters: std::option::Option<std::vec::Vec<crate::model::Filter>>,
}
impl std::fmt::Debug for ListGroupsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListGroupsInput");
        formatter.field("identity_store_id", &self.identity_store_id);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &self.next_token);
        formatter.field("filters", &self.filters);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeUserInput {
    /// <p>The globally unique identifier for the identity store, such as
    /// <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and
    /// <code>1234567890</code> is a randomly generated string that contains number and lower
    /// case letters. This value is generated at the time that a new identity store is
    /// created.</p>
    pub identity_store_id: std::option::Option<std::string::String>,
    /// <p>The identifier for a user in the identity store.</p>
    pub user_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeUserInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeUserInput");
        formatter.field("identity_store_id", &self.identity_store_id);
        formatter.field("user_id", &self.user_id);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeGroupInput {
    /// <p>The globally unique identifier for the identity store, such as
    /// <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and
    /// <code>1234567890</code> is a randomly generated string that contains number and lower
    /// case letters. This value is generated at the time that a new identity store is
    /// created.</p>
    pub identity_store_id: std::option::Option<std::string::String>,
    /// <p>The identifier for a group in the identity store.</p>
    pub group_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeGroupInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeGroupInput");
        formatter.field("identity_store_id", &self.identity_store_id);
        formatter.field("group_id", &self.group_id);
        formatter.finish()
    }
}