// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_check_layer_availability_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchCheckLayerAvailabilityInput,
) {
    if let Some(var_1) = &input.registry_id {
        object.key("registryId").string(var_1);
    }
    if let Some(var_2) = &input.repository_name {
        object.key("repositoryName").string(var_2);
    }
    if let Some(var_3) = &input.layer_digests {
        let mut array_4 = object.key("layerDigests").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5);
            }
        }
        array_4.finish();
    }
}

pub fn serialize_structure_crate_input_batch_delete_image_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDeleteImageInput,
) {
    if let Some(var_6) = &input.registry_id {
        object.key("registryId").string(var_6);
    }
    if let Some(var_7) = &input.repository_name {
        object.key("repositoryName").string(var_7);
    }
    if let Some(var_8) = &input.image_ids {
        let mut array_9 = object.key("imageIds").start_array();
        for item_10 in var_8 {
            {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_image_identifier(
                    &mut object_11,
                    item_10,
                );
                object_11.finish();
            }
        }
        array_9.finish();
    }
}

pub fn serialize_structure_crate_input_complete_layer_upload_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CompleteLayerUploadInput,
) {
    if let Some(var_12) = &input.registry_id {
        object.key("registryId").string(var_12);
    }
    if let Some(var_13) = &input.repository_name {
        object.key("repositoryName").string(var_13);
    }
    if let Some(var_14) = &input.upload_id {
        object.key("uploadId").string(var_14);
    }
    if let Some(var_15) = &input.layer_digests {
        let mut array_16 = object.key("layerDigests").start_array();
        for item_17 in var_15 {
            {
                array_16.value().string(item_17);
            }
        }
        array_16.finish();
    }
}

pub fn serialize_structure_crate_input_create_repository_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRepositoryInput,
) {
    if let Some(var_18) = &input.repository_name {
        object.key("repositoryName").string(var_18);
    }
    if let Some(var_19) = &input.catalog_data {
        let mut object_20 = object.key("catalogData").start_object();
        crate::json_ser::serialize_structure_crate_model_repository_catalog_data_input(
            &mut object_20,
            var_19,
        );
        object_20.finish();
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("tags").start_array();
        for item_23 in var_21 {
            {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_24, item_23);
                object_24.finish();
            }
        }
        array_22.finish();
    }
}

pub fn serialize_structure_crate_input_delete_repository_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRepositoryInput,
) {
    if let Some(var_25) = &input.registry_id {
        object.key("registryId").string(var_25);
    }
    if let Some(var_26) = &input.repository_name {
        object.key("repositoryName").string(var_26);
    }
    if input.force {
        object.key("force").boolean(input.force);
    }
}

pub fn serialize_structure_crate_input_delete_repository_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRepositoryPolicyInput,
) {
    if let Some(var_27) = &input.registry_id {
        object.key("registryId").string(var_27);
    }
    if let Some(var_28) = &input.repository_name {
        object.key("repositoryName").string(var_28);
    }
}

pub fn serialize_structure_crate_input_describe_images_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeImagesInput,
) {
    if let Some(var_29) = &input.registry_id {
        object.key("registryId").string(var_29);
    }
    if let Some(var_30) = &input.repository_name {
        object.key("repositoryName").string(var_30);
    }
    if let Some(var_31) = &input.image_ids {
        let mut array_32 = object.key("imageIds").start_array();
        for item_33 in var_31 {
            {
                let mut object_34 = array_32.value().start_object();
                crate::json_ser::serialize_structure_crate_model_image_identifier(
                    &mut object_34,
                    item_33,
                );
                object_34.finish();
            }
        }
        array_32.finish();
    }
    if let Some(var_35) = &input.next_token {
        object.key("nextToken").string(var_35);
    }
    if let Some(var_36) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_36).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_image_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeImageTagsInput,
) {
    if let Some(var_37) = &input.registry_id {
        object.key("registryId").string(var_37);
    }
    if let Some(var_38) = &input.repository_name {
        object.key("repositoryName").string(var_38);
    }
    if let Some(var_39) = &input.next_token {
        object.key("nextToken").string(var_39);
    }
    if let Some(var_40) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_40).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_registries_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeRegistriesInput,
) {
    if let Some(var_41) = &input.next_token {
        object.key("nextToken").string(var_41);
    }
    if let Some(var_42) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_42).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_repositories_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeRepositoriesInput,
) {
    if let Some(var_43) = &input.registry_id {
        object.key("registryId").string(var_43);
    }
    if let Some(var_44) = &input.repository_names {
        let mut array_45 = object.key("repositoryNames").start_array();
        for item_46 in var_44 {
            {
                array_45.value().string(item_46);
            }
        }
        array_45.finish();
    }
    if let Some(var_47) = &input.next_token {
        object.key("nextToken").string(var_47);
    }
    if let Some(var_48) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_48).into()),
        );
    }
}

pub fn serialize_structure_crate_input_get_repository_catalog_data_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRepositoryCatalogDataInput,
) {
    if let Some(var_49) = &input.registry_id {
        object.key("registryId").string(var_49);
    }
    if let Some(var_50) = &input.repository_name {
        object.key("repositoryName").string(var_50);
    }
}

pub fn serialize_structure_crate_input_get_repository_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRepositoryPolicyInput,
) {
    if let Some(var_51) = &input.registry_id {
        object.key("registryId").string(var_51);
    }
    if let Some(var_52) = &input.repository_name {
        object.key("repositoryName").string(var_52);
    }
}

pub fn serialize_structure_crate_input_initiate_layer_upload_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::InitiateLayerUploadInput,
) {
    if let Some(var_53) = &input.registry_id {
        object.key("registryId").string(var_53);
    }
    if let Some(var_54) = &input.repository_name {
        object.key("repositoryName").string(var_54);
    }
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_55) = &input.resource_arn {
        object.key("resourceArn").string(var_55);
    }
}

pub fn serialize_structure_crate_input_put_image_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutImageInput,
) {
    if let Some(var_56) = &input.registry_id {
        object.key("registryId").string(var_56);
    }
    if let Some(var_57) = &input.repository_name {
        object.key("repositoryName").string(var_57);
    }
    if let Some(var_58) = &input.image_manifest {
        object.key("imageManifest").string(var_58);
    }
    if let Some(var_59) = &input.image_manifest_media_type {
        object.key("imageManifestMediaType").string(var_59);
    }
    if let Some(var_60) = &input.image_tag {
        object.key("imageTag").string(var_60);
    }
    if let Some(var_61) = &input.image_digest {
        object.key("imageDigest").string(var_61);
    }
}

pub fn serialize_structure_crate_input_put_registry_catalog_data_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRegistryCatalogDataInput,
) {
    if let Some(var_62) = &input.display_name {
        object.key("displayName").string(var_62);
    }
}

pub fn serialize_structure_crate_input_put_repository_catalog_data_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRepositoryCatalogDataInput,
) {
    if let Some(var_63) = &input.registry_id {
        object.key("registryId").string(var_63);
    }
    if let Some(var_64) = &input.repository_name {
        object.key("repositoryName").string(var_64);
    }
    if let Some(var_65) = &input.catalog_data {
        let mut object_66 = object.key("catalogData").start_object();
        crate::json_ser::serialize_structure_crate_model_repository_catalog_data_input(
            &mut object_66,
            var_65,
        );
        object_66.finish();
    }
}

pub fn serialize_structure_crate_input_set_repository_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SetRepositoryPolicyInput,
) {
    if let Some(var_67) = &input.registry_id {
        object.key("registryId").string(var_67);
    }
    if let Some(var_68) = &input.repository_name {
        object.key("repositoryName").string(var_68);
    }
    if let Some(var_69) = &input.policy_text {
        object.key("policyText").string(var_69);
    }
    if input.force {
        object.key("force").boolean(input.force);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_70) = &input.resource_arn {
        object.key("resourceArn").string(var_70);
    }
    if let Some(var_71) = &input.tags {
        let mut array_72 = object.key("tags").start_array();
        for item_73 in var_71 {
            {
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_74, item_73);
                object_74.finish();
            }
        }
        array_72.finish();
    }
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_75) = &input.resource_arn {
        object.key("resourceArn").string(var_75);
    }
    if let Some(var_76) = &input.tag_keys {
        let mut array_77 = object.key("tagKeys").start_array();
        for item_78 in var_76 {
            {
                array_77.value().string(item_78);
            }
        }
        array_77.finish();
    }
}

pub fn serialize_structure_crate_input_upload_layer_part_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UploadLayerPartInput,
) {
    if let Some(var_79) = &input.registry_id {
        object.key("registryId").string(var_79);
    }
    if let Some(var_80) = &input.repository_name {
        object.key("repositoryName").string(var_80);
    }
    if let Some(var_81) = &input.upload_id {
        object.key("uploadId").string(var_81);
    }
    if let Some(var_82) = &input.part_first_byte {
        object.key("partFirstByte").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_82).into()),
        );
    }
    if let Some(var_83) = &input.part_last_byte {
        object.key("partLastByte").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_83).into()),
        );
    }
    if let Some(var_84) = &input.layer_part_blob {
        object
            .key("layerPartBlob")
            .string_unchecked(&smithy_types::base64::encode(var_84));
    }
}

pub fn serialize_structure_crate_model_image_identifier(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImageIdentifier,
) {
    if let Some(var_85) = &input.image_digest {
        object.key("imageDigest").string(var_85);
    }
    if let Some(var_86) = &input.image_tag {
        object.key("imageTag").string(var_86);
    }
}

pub fn serialize_structure_crate_model_repository_catalog_data_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RepositoryCatalogDataInput,
) {
    if let Some(var_87) = &input.description {
        object.key("description").string(var_87);
    }
    if let Some(var_88) = &input.architectures {
        let mut array_89 = object.key("architectures").start_array();
        for item_90 in var_88 {
            {
                array_89.value().string(item_90);
            }
        }
        array_89.finish();
    }
    if let Some(var_91) = &input.operating_systems {
        let mut array_92 = object.key("operatingSystems").start_array();
        for item_93 in var_91 {
            {
                array_92.value().string(item_93);
            }
        }
        array_92.finish();
    }
    if let Some(var_94) = &input.logo_image_blob {
        object
            .key("logoImageBlob")
            .string_unchecked(&smithy_types::base64::encode(var_94));
    }
    if let Some(var_95) = &input.about_text {
        object.key("aboutText").string(var_95);
    }
    if let Some(var_96) = &input.usage_text {
        object.key("usageText").string(var_96);
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_97) = &input.key {
        object.key("Key").string(var_97);
    }
    if let Some(var_98) = &input.value {
        object.key("Value").string(var_98);
    }
}