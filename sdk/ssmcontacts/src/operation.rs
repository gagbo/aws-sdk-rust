// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Used to acknowledge an engagement to a contact channel during an incident.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AcceptPage {
    _private: (),
}
impl AcceptPage {
    /// Creates a new builder-style object to manufacture [`AcceptPageInput`](crate::input::AcceptPageInput)
    pub fn builder() -> crate::input::accept_page_input::Builder {
        crate::input::accept_page_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AcceptPage {
    type Output =
        std::result::Result<crate::output::AcceptPageOutput, crate::error::AcceptPageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_accept_page_error(response)
        } else {
            crate::operation_deser::parse_accept_page_response(response)
        }
    }
}

/// <p>Activates a contact's contact channel. Incident Manager can't engage a contact until the
/// contact channel has been activated.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ActivateContactChannel {
    _private: (),
}
impl ActivateContactChannel {
    /// Creates a new builder-style object to manufacture [`ActivateContactChannelInput`](crate::input::ActivateContactChannelInput)
    pub fn builder() -> crate::input::activate_contact_channel_input::Builder {
        crate::input::activate_contact_channel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ActivateContactChannel {
    type Output = std::result::Result<
        crate::output::ActivateContactChannelOutput,
        crate::error::ActivateContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_activate_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_activate_contact_channel_response(response)
        }
    }
}

/// <p>Contacts are either the contacts that Incident Manager engages during an incident or the
/// escalation plans that Incident Manager uses to engage contacts in phases during an
/// incident. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateContact {
    _private: (),
}
impl CreateContact {
    /// Creates a new builder-style object to manufacture [`CreateContactInput`](crate::input::CreateContactInput)
    pub fn builder() -> crate::input::create_contact_input::Builder {
        crate::input::create_contact_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateContact {
    type Output =
        std::result::Result<crate::output::CreateContactOutput, crate::error::CreateContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_contact_error(response)
        } else {
            crate::operation_deser::parse_create_contact_response(response)
        }
    }
}

/// <p>A contact channel is the method that Incident Manager uses to engage your contact.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateContactChannel {
    _private: (),
}
impl CreateContactChannel {
    /// Creates a new builder-style object to manufacture [`CreateContactChannelInput`](crate::input::CreateContactChannelInput)
    pub fn builder() -> crate::input::create_contact_channel_input::Builder {
        crate::input::create_contact_channel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateContactChannel {
    type Output = std::result::Result<
        crate::output::CreateContactChannelOutput,
        crate::error::CreateContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_create_contact_channel_response(response)
        }
    }
}

/// <p>To no longer receive Incident Manager engagements to a contact channel, you can deactivate
/// the channel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeactivateContactChannel {
    _private: (),
}
impl DeactivateContactChannel {
    /// Creates a new builder-style object to manufacture [`DeactivateContactChannelInput`](crate::input::DeactivateContactChannelInput)
    pub fn builder() -> crate::input::deactivate_contact_channel_input::Builder {
        crate::input::deactivate_contact_channel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeactivateContactChannel {
    type Output = std::result::Result<
        crate::output::DeactivateContactChannelOutput,
        crate::error::DeactivateContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_deactivate_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_deactivate_contact_channel_response(response)
        }
    }
}

/// <p>To remove a contact from Incident Manager, you can delete the contact. Deleting a contact
/// removes them from all escalation plans and related response plans. Deleting an escalation
/// plan removes it from all related response plans. You will have to recreate the contact and
/// its contact channels before you can use it again.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteContact {
    _private: (),
}
impl DeleteContact {
    /// Creates a new builder-style object to manufacture [`DeleteContactInput`](crate::input::DeleteContactInput)
    pub fn builder() -> crate::input::delete_contact_input::Builder {
        crate::input::delete_contact_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteContact {
    type Output =
        std::result::Result<crate::output::DeleteContactOutput, crate::error::DeleteContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_contact_error(response)
        } else {
            crate::operation_deser::parse_delete_contact_response(response)
        }
    }
}

/// <p>To no longer receive engagements on a contact channel, you can delete the channel from a
/// contact. Deleting the contact channel removes it from the contact's engagement plan. If you
/// delete the only contact channel for a contact, you won't be able to engage that contact
/// during an incident.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteContactChannel {
    _private: (),
}
impl DeleteContactChannel {
    /// Creates a new builder-style object to manufacture [`DeleteContactChannelInput`](crate::input::DeleteContactChannelInput)
    pub fn builder() -> crate::input::delete_contact_channel_input::Builder {
        crate::input::delete_contact_channel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteContactChannel {
    type Output = std::result::Result<
        crate::output::DeleteContactChannelOutput,
        crate::error::DeleteContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_delete_contact_channel_response(response)
        }
    }
}

/// <p>Incident Manager uses engagements to engage contacts and escalation plans during an incident.
/// Use this command to describe the engagement that occurred during an incident.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEngagement {
    _private: (),
}
impl DescribeEngagement {
    /// Creates a new builder-style object to manufacture [`DescribeEngagementInput`](crate::input::DescribeEngagementInput)
    pub fn builder() -> crate::input::describe_engagement_input::Builder {
        crate::input::describe_engagement_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEngagement {
    type Output = std::result::Result<
        crate::output::DescribeEngagementOutput,
        crate::error::DescribeEngagementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_engagement_error(response)
        } else {
            crate::operation_deser::parse_describe_engagement_response(response)
        }
    }
}

/// <p>Lists details of the engagement to a contact channel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribePage {
    _private: (),
}
impl DescribePage {
    /// Creates a new builder-style object to manufacture [`DescribePageInput`](crate::input::DescribePageInput)
    pub fn builder() -> crate::input::describe_page_input::Builder {
        crate::input::describe_page_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribePage {
    type Output =
        std::result::Result<crate::output::DescribePageOutput, crate::error::DescribePageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_page_error(response)
        } else {
            crate::operation_deser::parse_describe_page_response(response)
        }
    }
}

/// <p>Retrieves information about the specified contact or escalation plan.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetContact {
    _private: (),
}
impl GetContact {
    /// Creates a new builder-style object to manufacture [`GetContactInput`](crate::input::GetContactInput)
    pub fn builder() -> crate::input::get_contact_input::Builder {
        crate::input::get_contact_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetContact {
    type Output =
        std::result::Result<crate::output::GetContactOutput, crate::error::GetContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_contact_error(response)
        } else {
            crate::operation_deser::parse_get_contact_response(response)
        }
    }
}

/// <p>List details about a specific contact channel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetContactChannel {
    _private: (),
}
impl GetContactChannel {
    /// Creates a new builder-style object to manufacture [`GetContactChannelInput`](crate::input::GetContactChannelInput)
    pub fn builder() -> crate::input::get_contact_channel_input::Builder {
        crate::input::get_contact_channel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetContactChannel {
    type Output = std::result::Result<
        crate::output::GetContactChannelOutput,
        crate::error::GetContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_get_contact_channel_response(response)
        }
    }
}

/// <p>Retrieves the resource policies attached to the specified contact or escalation
/// plan.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetContactPolicy {
    _private: (),
}
impl GetContactPolicy {
    /// Creates a new builder-style object to manufacture [`GetContactPolicyInput`](crate::input::GetContactPolicyInput)
    pub fn builder() -> crate::input::get_contact_policy_input::Builder {
        crate::input::get_contact_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetContactPolicy {
    type Output = std::result::Result<
        crate::output::GetContactPolicyOutput,
        crate::error::GetContactPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_contact_policy_error(response)
        } else {
            crate::operation_deser::parse_get_contact_policy_response(response)
        }
    }
}

/// <p>Lists all contact channels for the specified contact.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListContactChannels {
    _private: (),
}
impl ListContactChannels {
    /// Creates a new builder-style object to manufacture [`ListContactChannelsInput`](crate::input::ListContactChannelsInput)
    pub fn builder() -> crate::input::list_contact_channels_input::Builder {
        crate::input::list_contact_channels_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListContactChannels {
    type Output = std::result::Result<
        crate::output::ListContactChannelsOutput,
        crate::error::ListContactChannelsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_contact_channels_error(response)
        } else {
            crate::operation_deser::parse_list_contact_channels_response(response)
        }
    }
}

/// <p>Lists all contacts and escalation plans in Incident Manager.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListContacts {
    _private: (),
}
impl ListContacts {
    /// Creates a new builder-style object to manufacture [`ListContactsInput`](crate::input::ListContactsInput)
    pub fn builder() -> crate::input::list_contacts_input::Builder {
        crate::input::list_contacts_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListContacts {
    type Output =
        std::result::Result<crate::output::ListContactsOutput, crate::error::ListContactsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_contacts_error(response)
        } else {
            crate::operation_deser::parse_list_contacts_response(response)
        }
    }
}

/// <p>Lists all engagements that have happened in an incident.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEngagements {
    _private: (),
}
impl ListEngagements {
    /// Creates a new builder-style object to manufacture [`ListEngagementsInput`](crate::input::ListEngagementsInput)
    pub fn builder() -> crate::input::list_engagements_input::Builder {
        crate::input::list_engagements_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListEngagements {
    type Output = std::result::Result<
        crate::output::ListEngagementsOutput,
        crate::error::ListEngagementsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_engagements_error(response)
        } else {
            crate::operation_deser::parse_list_engagements_response(response)
        }
    }
}

/// <p>Lists all of the engagements to contact channels that have been acknowledged. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPageReceipts {
    _private: (),
}
impl ListPageReceipts {
    /// Creates a new builder-style object to manufacture [`ListPageReceiptsInput`](crate::input::ListPageReceiptsInput)
    pub fn builder() -> crate::input::list_page_receipts_input::Builder {
        crate::input::list_page_receipts_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPageReceipts {
    type Output = std::result::Result<
        crate::output::ListPageReceiptsOutput,
        crate::error::ListPageReceiptsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_page_receipts_error(response)
        } else {
            crate::operation_deser::parse_list_page_receipts_response(response)
        }
    }
}

/// <p>Lists the engagements to a contact's contact channels.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPagesByContact {
    _private: (),
}
impl ListPagesByContact {
    /// Creates a new builder-style object to manufacture [`ListPagesByContactInput`](crate::input::ListPagesByContactInput)
    pub fn builder() -> crate::input::list_pages_by_contact_input::Builder {
        crate::input::list_pages_by_contact_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPagesByContact {
    type Output = std::result::Result<
        crate::output::ListPagesByContactOutput,
        crate::error::ListPagesByContactError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_pages_by_contact_error(response)
        } else {
            crate::operation_deser::parse_list_pages_by_contact_response(response)
        }
    }
}

/// <p>Lists the engagements to contact channels that occurred by engaging a contact.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPagesByEngagement {
    _private: (),
}
impl ListPagesByEngagement {
    /// Creates a new builder-style object to manufacture [`ListPagesByEngagementInput`](crate::input::ListPagesByEngagementInput)
    pub fn builder() -> crate::input::list_pages_by_engagement_input::Builder {
        crate::input::list_pages_by_engagement_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPagesByEngagement {
    type Output = std::result::Result<
        crate::output::ListPagesByEngagementOutput,
        crate::error::ListPagesByEngagementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_pages_by_engagement_error(response)
        } else {
            crate::operation_deser::parse_list_pages_by_engagement_response(response)
        }
    }
}

/// <p>Lists the tags of an escalation plan or contact.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <p>Adds a resource to the specified contact or escalation plan.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutContactPolicy {
    _private: (),
}
impl PutContactPolicy {
    /// Creates a new builder-style object to manufacture [`PutContactPolicyInput`](crate::input::PutContactPolicyInput)
    pub fn builder() -> crate::input::put_contact_policy_input::Builder {
        crate::input::put_contact_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutContactPolicy {
    type Output = std::result::Result<
        crate::output::PutContactPolicyOutput,
        crate::error::PutContactPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_contact_policy_error(response)
        } else {
            crate::operation_deser::parse_put_contact_policy_response(response)
        }
    }
}

/// <p>Sends an activation code to a contact channel. The contact can use this code to activate
/// the contact channel in the console or with the <code>ActivateChannel</code> operation.
/// Incident Manager can't engage a contact channel until it has been activated.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SendActivationCode {
    _private: (),
}
impl SendActivationCode {
    /// Creates a new builder-style object to manufacture [`SendActivationCodeInput`](crate::input::SendActivationCodeInput)
    pub fn builder() -> crate::input::send_activation_code_input::Builder {
        crate::input::send_activation_code_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for SendActivationCode {
    type Output = std::result::Result<
        crate::output::SendActivationCodeOutput,
        crate::error::SendActivationCodeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_send_activation_code_error(response)
        } else {
            crate::operation_deser::parse_send_activation_code_response(response)
        }
    }
}

/// <p>Starts an engagement to a contact or escalation plan. The engagement engages each
/// contact specified in the incident.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartEngagement {
    _private: (),
}
impl StartEngagement {
    /// Creates a new builder-style object to manufacture [`StartEngagementInput`](crate::input::StartEngagementInput)
    pub fn builder() -> crate::input::start_engagement_input::Builder {
        crate::input::start_engagement_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartEngagement {
    type Output = std::result::Result<
        crate::output::StartEngagementOutput,
        crate::error::StartEngagementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_engagement_error(response)
        } else {
            crate::operation_deser::parse_start_engagement_response(response)
        }
    }
}

/// <p>Stops an engagement before it finishes the final stage of the escalation plan or
/// engagement plan. Further contacts aren't engaged.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopEngagement {
    _private: (),
}
impl StopEngagement {
    /// Creates a new builder-style object to manufacture [`StopEngagementInput`](crate::input::StopEngagementInput)
    pub fn builder() -> crate::input::stop_engagement_input::Builder {
        crate::input::stop_engagement_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StopEngagement {
    type Output =
        std::result::Result<crate::output::StopEngagementOutput, crate::error::StopEngagementError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_engagement_error(response)
        } else {
            crate::operation_deser::parse_stop_engagement_response(response)
        }
    }
}

/// <p>Tags a contact or escalation plan. You can tag only contacts and escalation plans in the
/// first region of your replication set. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>Removes tags from the specified resource. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// <p>Updates the contact or escalation plan specified. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateContact {
    _private: (),
}
impl UpdateContact {
    /// Creates a new builder-style object to manufacture [`UpdateContactInput`](crate::input::UpdateContactInput)
    pub fn builder() -> crate::input::update_contact_input::Builder {
        crate::input::update_contact_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateContact {
    type Output =
        std::result::Result<crate::output::UpdateContactOutput, crate::error::UpdateContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_contact_error(response)
        } else {
            crate::operation_deser::parse_update_contact_response(response)
        }
    }
}

/// <p>Updates a contact's contact channel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateContactChannel {
    _private: (),
}
impl UpdateContactChannel {
    /// Creates a new builder-style object to manufacture [`UpdateContactChannelInput`](crate::input::UpdateContactChannelInput)
    pub fn builder() -> crate::input::update_contact_channel_input::Builder {
        crate::input::update_contact_channel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateContactChannel {
    type Output = std::result::Result<
        crate::output::UpdateContactChannelOutput,
        crate::error::UpdateContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_update_contact_channel_response(response)
        }
    }
}