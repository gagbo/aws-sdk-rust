// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_aliases_output_next_marker(
    input: &crate::output::ListAliasesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_grants_output_next_marker(
    input: &crate::output::ListGrantsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_key_policies_output_next_marker(
    input: &crate::output::ListKeyPoliciesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_keys_output_next_marker(
    input: &crate::output::ListKeysOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_aliases_output_aliases(
    input: crate::output::ListAliasesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::AliasListEntry>> {
    let input = match input.aliases {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_grants_output_grants(
    input: crate::output::ListGrantsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::GrantListEntry>> {
    let input = match input.grants {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_key_policies_output_policy_names(
    input: crate::output::ListKeyPoliciesOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.policy_names {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_keys_output_keys(
    input: crate::output::ListKeysOutput,
) -> std::option::Option<std::vec::Vec<crate::model::KeyListEntry>> {
    let input = match input.keys {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}