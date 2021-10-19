// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Assigns a Grafana Enterprise license to a workspace. Upgrading to Grafana Enterprise
/// incurs additional fees. For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/upgrade-to-Grafana-Enterprise.html">Upgrade a workspace to
/// Grafana Enterprise</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateLicense {
    _private: (),
}
impl AssociateLicense {
    /// Creates a new builder-style object to manufacture [`AssociateLicenseInput`](crate::input::AssociateLicenseInput)
    pub fn builder() -> crate::input::associate_license_input::Builder {
        crate::input::associate_license_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AssociateLicense {
    type Output = std::result::Result<
        crate::output::AssociateLicenseOutput,
        crate::error::AssociateLicenseError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_associate_license_error(response)
        } else {
            crate::operation_deser::parse_associate_license_response(response)
        }
    }
}

/// <p>Creates a <i>workspace</i>. In a workspace, you can create Grafana
/// dashboards and visualizations to analyze your metrics, logs, and traces. You don't have to
/// build, package, or deploy any hardware to run the Grafana server.</p>
/// <p>Don't use <code>CreateWorkspace</code> to modify an existing workspace. Instead,
/// use <a href="https://docs.aws.amazon.com/grafana/latest/APIReference/API_UpdateWorkspace.html">UpdateWorkspace</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateWorkspace {
    _private: (),
}
impl CreateWorkspace {
    /// Creates a new builder-style object to manufacture [`CreateWorkspaceInput`](crate::input::CreateWorkspaceInput)
    pub fn builder() -> crate::input::create_workspace_input::Builder {
        crate::input::create_workspace_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateWorkspace {
    type Output = std::result::Result<
        crate::output::CreateWorkspaceOutput,
        crate::error::CreateWorkspaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_create_workspace_error(response)
        } else {
            crate::operation_deser::parse_create_workspace_response(response)
        }
    }
}

/// <p>Deletes an Amazon Managed Grafana workspace.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteWorkspace {
    _private: (),
}
impl DeleteWorkspace {
    /// Creates a new builder-style object to manufacture [`DeleteWorkspaceInput`](crate::input::DeleteWorkspaceInput)
    pub fn builder() -> crate::input::delete_workspace_input::Builder {
        crate::input::delete_workspace_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteWorkspace {
    type Output = std::result::Result<
        crate::output::DeleteWorkspaceOutput,
        crate::error::DeleteWorkspaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_workspace_error(response)
        } else {
            crate::operation_deser::parse_delete_workspace_response(response)
        }
    }
}

/// <p>Displays information about one Amazon Managed Grafana workspace.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeWorkspace {
    _private: (),
}
impl DescribeWorkspace {
    /// Creates a new builder-style object to manufacture [`DescribeWorkspaceInput`](crate::input::DescribeWorkspaceInput)
    pub fn builder() -> crate::input::describe_workspace_input::Builder {
        crate::input::describe_workspace_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeWorkspace {
    type Output = std::result::Result<
        crate::output::DescribeWorkspaceOutput,
        crate::error::DescribeWorkspaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_workspace_error(response)
        } else {
            crate::operation_deser::parse_describe_workspace_response(response)
        }
    }
}

/// <p>Displays information about the authentication methods used in one Amazon Managed Grafana workspace.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeWorkspaceAuthentication {
    _private: (),
}
impl DescribeWorkspaceAuthentication {
    /// Creates a new builder-style object to manufacture [`DescribeWorkspaceAuthenticationInput`](crate::input::DescribeWorkspaceAuthenticationInput)
    pub fn builder() -> crate::input::describe_workspace_authentication_input::Builder {
        crate::input::describe_workspace_authentication_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeWorkspaceAuthentication {
    type Output = std::result::Result<
        crate::output::DescribeWorkspaceAuthenticationOutput,
        crate::error::DescribeWorkspaceAuthenticationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_workspace_authentication_error(response)
        } else {
            crate::operation_deser::parse_describe_workspace_authentication_response(response)
        }
    }
}

/// <p>Removes the Grafana Enterprise license from a workspace.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateLicense {
    _private: (),
}
impl DisassociateLicense {
    /// Creates a new builder-style object to manufacture [`DisassociateLicenseInput`](crate::input::DisassociateLicenseInput)
    pub fn builder() -> crate::input::disassociate_license_input::Builder {
        crate::input::disassociate_license_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DisassociateLicense {
    type Output = std::result::Result<
        crate::output::DisassociateLicenseOutput,
        crate::error::DisassociateLicenseError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_disassociate_license_error(response)
        } else {
            crate::operation_deser::parse_disassociate_license_response(response)
        }
    }
}

/// <p>Lists the users and groups who have the Grafana <code>Admin</code> and
/// <code>Editor</code> roles in this workspace. If you use this
/// operation without specifying <code>userId</code> or <code>groupId</code>, the operation returns
/// the roles of all users
/// and groups. If you specify a <code>userId</code> or a <code>groupId</code>, only the roles
/// for that user or group are returned. If you do this, you can specify only one <code>userId</code> or
/// one <code>groupId</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPermissions {
    _private: (),
}
impl ListPermissions {
    /// Creates a new builder-style object to manufacture [`ListPermissionsInput`](crate::input::ListPermissionsInput)
    pub fn builder() -> crate::input::list_permissions_input::Builder {
        crate::input::list_permissions_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPermissions {
    type Output = std::result::Result<
        crate::output::ListPermissionsOutput,
        crate::error::ListPermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_permissions_error(response)
        } else {
            crate::operation_deser::parse_list_permissions_response(response)
        }
    }
}

/// <p>Returns a list of Amazon Managed Grafana workspaces in the account, with some information
/// about each workspace. For more complete information about one workspace, use <a href="https://docs.aws.amazon.com/AAMG/latest/APIReference/API_DescribeWorkspace.html">DescribeWorkspace</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListWorkspaces {
    _private: (),
}
impl ListWorkspaces {
    /// Creates a new builder-style object to manufacture [`ListWorkspacesInput`](crate::input::ListWorkspacesInput)
    pub fn builder() -> crate::input::list_workspaces_input::Builder {
        crate::input::list_workspaces_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListWorkspaces {
    type Output =
        std::result::Result<crate::output::ListWorkspacesOutput, crate::error::ListWorkspacesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_workspaces_error(response)
        } else {
            crate::operation_deser::parse_list_workspaces_response(response)
        }
    }
}

/// <p>Updates which users in a workspace have the Grafana <code>Admin</code> or <code>Editor</code> roles.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdatePermissions {
    _private: (),
}
impl UpdatePermissions {
    /// Creates a new builder-style object to manufacture [`UpdatePermissionsInput`](crate::input::UpdatePermissionsInput)
    pub fn builder() -> crate::input::update_permissions_input::Builder {
        crate::input::update_permissions_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdatePermissions {
    type Output = std::result::Result<
        crate::output::UpdatePermissionsOutput,
        crate::error::UpdatePermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_permissions_error(response)
        } else {
            crate::operation_deser::parse_update_permissions_response(response)
        }
    }
}

/// <p>Modifies an existing Amazon Managed Grafana workspace. If you use this operation and omit any
/// optional parameters, the existing values of those parameters are not changed.</p>
/// <p>To modify the user authentication methods that the workspace uses, such as SAML or Amazon Web Services SSO,
/// use <a href="https://docs.aws.amazon.com/grafana/latest/APIReference/API_UpdateWorkspaceAuthentication.html">UpdateWorkspaceAuthentication</a>.</p>
/// <p>To modify which users in the workspace have the <code>Admin</code> and <code>Editor</code> Grafana roles,
/// use <a href="https://docs.aws.amazon.com/grafana/latest/APIReference/API_UpdatePermissions.html">UpdatePermissions</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateWorkspace {
    _private: (),
}
impl UpdateWorkspace {
    /// Creates a new builder-style object to manufacture [`UpdateWorkspaceInput`](crate::input::UpdateWorkspaceInput)
    pub fn builder() -> crate::input::update_workspace_input::Builder {
        crate::input::update_workspace_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateWorkspace {
    type Output = std::result::Result<
        crate::output::UpdateWorkspaceOutput,
        crate::error::UpdateWorkspaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_update_workspace_error(response)
        } else {
            crate::operation_deser::parse_update_workspace_response(response)
        }
    }
}

/// <p>Use this operation to define the identity provider (IdP) that this workspace
/// authenticates users from, using SAML. You can also map SAML assertion attributes to
/// workspace user information and define which groups in the assertion attribute are to have
/// the <code>Admin</code> and <code>Editor</code> roles in the workspace.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateWorkspaceAuthentication {
    _private: (),
}
impl UpdateWorkspaceAuthentication {
    /// Creates a new builder-style object to manufacture [`UpdateWorkspaceAuthenticationInput`](crate::input::UpdateWorkspaceAuthenticationInput)
    pub fn builder() -> crate::input::update_workspace_authentication_input::Builder {
        crate::input::update_workspace_authentication_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateWorkspaceAuthentication {
    type Output = std::result::Result<
        crate::output::UpdateWorkspaceAuthenticationOutput,
        crate::error::UpdateWorkspaceAuthenticationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_workspace_authentication_error(response)
        } else {
            crate::operation_deser::parse_update_workspace_authentication_response(response)
        }
    }
}