// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_configure_logs_for_playback_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ConfigureLogsForPlaybackConfigurationInput,
) {
    {
        object.key("PercentEnabled").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.percent_enabled).into()),
        );
    }
    if let Some(var_1) = &input.playback_configuration_name {
        object.key("PlaybackConfigurationName").string(var_1);
    }
}

pub fn serialize_structure_crate_input_create_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChannelInput,
) {
    if let Some(var_2) = &input.filler_slate {
        let mut object_3 = object.key("FillerSlate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_3, var_2);
        object_3.finish();
    }
    if let Some(var_4) = &input.outputs {
        let mut array_5 = object.key("Outputs").start_array();
        for item_6 in var_4 {
            {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_request_output_item(
                    &mut object_7,
                    item_6,
                );
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.playback_mode {
        object.key("PlaybackMode").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11).string(value_12);
            }
        }
        object_10.finish();
    }
}

pub fn serialize_structure_crate_input_create_program_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProgramInput,
) {
    if let Some(var_13) = &input.ad_breaks {
        let mut array_14 = object.key("AdBreaks").start_array();
        for item_15 in var_13 {
            {
                let mut object_16 = array_14.value().start_object();
                crate::json_ser::serialize_structure_crate_model_ad_break(&mut object_16, item_15);
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.schedule_configuration {
        let mut object_18 = object.key("ScheduleConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_schedule_configuration(
            &mut object_18,
            var_17,
        );
        object_18.finish();
    }
    if let Some(var_19) = &input.source_location_name {
        object.key("SourceLocationName").string(var_19);
    }
    if let Some(var_20) = &input.vod_source_name {
        object.key("VodSourceName").string(var_20);
    }
}

pub fn serialize_structure_crate_input_create_source_location_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSourceLocationInput,
) {
    if let Some(var_21) = &input.access_configuration {
        let mut object_22 = object.key("AccessConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_access_configuration(
            &mut object_22,
            var_21,
        );
        object_22.finish();
    }
    if let Some(var_23) = &input.default_segment_delivery_configuration {
        let mut object_24 = object
            .key("DefaultSegmentDeliveryConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_default_segment_delivery_configuration(
            &mut object_24,
            var_23,
        );
        object_24.finish();
    }
    if let Some(var_25) = &input.http_configuration {
        let mut object_26 = object.key("HttpConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_http_configuration(&mut object_26, var_25);
        object_26.finish();
    }
    if let Some(var_27) = &input.tags {
        let mut object_28 = object.key("tags").start_object();
        for (key_29, value_30) in var_27 {
            {
                object_28.key(key_29).string(value_30);
            }
        }
        object_28.finish();
    }
}

pub fn serialize_structure_crate_input_create_vod_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateVodSourceInput,
) {
    if let Some(var_31) = &input.http_package_configurations {
        let mut array_32 = object.key("HttpPackageConfigurations").start_array();
        for item_33 in var_31 {
            {
                let mut object_34 = array_32.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_34,
                    item_33,
                );
                object_34.finish();
            }
        }
        array_32.finish();
    }
    if let Some(var_35) = &input.tags {
        let mut object_36 = object.key("tags").start_object();
        for (key_37, value_38) in var_35 {
            {
                object_36.key(key_37).string(value_38);
            }
        }
        object_36.finish();
    }
}

pub fn serialize_structure_crate_input_put_channel_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutChannelPolicyInput,
) {
    if let Some(var_39) = &input.policy {
        object.key("Policy").string(var_39);
    }
}

pub fn serialize_structure_crate_input_put_playback_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPlaybackConfigurationInput,
) {
    if let Some(var_40) = &input.ad_decision_server_url {
        object.key("AdDecisionServerUrl").string(var_40);
    }
    if let Some(var_41) = &input.avail_suppression {
        let mut object_42 = object.key("AvailSuppression").start_object();
        crate::json_ser::serialize_structure_crate_model_avail_suppression(&mut object_42, var_41);
        object_42.finish();
    }
    if let Some(var_43) = &input.bumper {
        let mut object_44 = object.key("Bumper").start_object();
        crate::json_ser::serialize_structure_crate_model_bumper(&mut object_44, var_43);
        object_44.finish();
    }
    if let Some(var_45) = &input.cdn_configuration {
        let mut object_46 = object.key("CdnConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_cdn_configuration(&mut object_46, var_45);
        object_46.finish();
    }
    if let Some(var_47) = &input.configuration_aliases {
        let mut object_48 = object.key("ConfigurationAliases").start_object();
        for (key_49, value_50) in var_47 {
            {
                let mut object_51 = object_48.key(key_49).start_object();
                for (key_52, value_53) in value_50 {
                    {
                        object_51.key(key_52).string(value_53);
                    }
                }
                object_51.finish();
            }
        }
        object_48.finish();
    }
    if let Some(var_54) = &input.dash_configuration {
        let mut object_55 = object.key("DashConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_configuration_for_put(
            &mut object_55,
            var_54,
        );
        object_55.finish();
    }
    if let Some(var_56) = &input.live_pre_roll_configuration {
        let mut object_57 = object.key("LivePreRollConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_live_pre_roll_configuration(
            &mut object_57,
            var_56,
        );
        object_57.finish();
    }
    if let Some(var_58) = &input.manifest_processing_rules {
        let mut object_59 = object.key("ManifestProcessingRules").start_object();
        crate::json_ser::serialize_structure_crate_model_manifest_processing_rules(
            &mut object_59,
            var_58,
        );
        object_59.finish();
    }
    if let Some(var_60) = &input.name {
        object.key("Name").string(var_60);
    }
    if input.personalization_threshold_seconds != 0 {
        object.key("PersonalizationThresholdSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.personalization_threshold_seconds).into()),
        );
    }
    if let Some(var_61) = &input.slate_ad_url {
        object.key("SlateAdUrl").string(var_61);
    }
    if let Some(var_62) = &input.tags {
        let mut object_63 = object.key("tags").start_object();
        for (key_64, value_65) in var_62 {
            {
                object_63.key(key_64).string(value_65);
            }
        }
        object_63.finish();
    }
    if let Some(var_66) = &input.transcode_profile_name {
        object.key("TranscodeProfileName").string(var_66);
    }
    if let Some(var_67) = &input.video_content_source_url {
        object.key("VideoContentSourceUrl").string(var_67);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_68) = &input.tags {
        let mut object_69 = object.key("tags").start_object();
        for (key_70, value_71) in var_68 {
            {
                object_69.key(key_70).string(value_71);
            }
        }
        object_69.finish();
    }
}

pub fn serialize_structure_crate_input_update_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChannelInput,
) {
    if let Some(var_72) = &input.outputs {
        let mut array_73 = object.key("Outputs").start_array();
        for item_74 in var_72 {
            {
                let mut object_75 = array_73.value().start_object();
                crate::json_ser::serialize_structure_crate_model_request_output_item(
                    &mut object_75,
                    item_74,
                );
                object_75.finish();
            }
        }
        array_73.finish();
    }
}

pub fn serialize_structure_crate_input_update_source_location_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSourceLocationInput,
) {
    if let Some(var_76) = &input.access_configuration {
        let mut object_77 = object.key("AccessConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_access_configuration(
            &mut object_77,
            var_76,
        );
        object_77.finish();
    }
    if let Some(var_78) = &input.default_segment_delivery_configuration {
        let mut object_79 = object
            .key("DefaultSegmentDeliveryConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_default_segment_delivery_configuration(
            &mut object_79,
            var_78,
        );
        object_79.finish();
    }
    if let Some(var_80) = &input.http_configuration {
        let mut object_81 = object.key("HttpConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_http_configuration(&mut object_81, var_80);
        object_81.finish();
    }
}

pub fn serialize_structure_crate_input_update_vod_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateVodSourceInput,
) {
    if let Some(var_82) = &input.http_package_configurations {
        let mut array_83 = object.key("HttpPackageConfigurations").start_array();
        for item_84 in var_82 {
            {
                let mut object_85 = array_83.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_85,
                    item_84,
                );
                object_85.finish();
            }
        }
        array_83.finish();
    }
}

pub fn serialize_structure_crate_model_slate_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SlateSource,
) {
    if let Some(var_86) = &input.source_location_name {
        object.key("SourceLocationName").string(var_86);
    }
    if let Some(var_87) = &input.vod_source_name {
        object.key("VodSourceName").string(var_87);
    }
}

pub fn serialize_structure_crate_model_request_output_item(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RequestOutputItem,
) {
    if let Some(var_88) = &input.dash_playlist_settings {
        let mut object_89 = object.key("DashPlaylistSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_playlist_settings(
            &mut object_89,
            var_88,
        );
        object_89.finish();
    }
    if let Some(var_90) = &input.hls_playlist_settings {
        let mut object_91 = object.key("HlsPlaylistSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_playlist_settings(
            &mut object_91,
            var_90,
        );
        object_91.finish();
    }
    if let Some(var_92) = &input.manifest_name {
        object.key("ManifestName").string(var_92);
    }
    if let Some(var_93) = &input.source_group {
        object.key("SourceGroup").string(var_93);
    }
}

pub fn serialize_structure_crate_model_ad_break(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdBreak,
) {
    if let Some(var_94) = &input.message_type {
        object.key("MessageType").string(var_94.as_str());
    }
    if input.offset_millis != 0 {
        object.key("OffsetMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.offset_millis).into()),
        );
    }
    if let Some(var_95) = &input.slate {
        let mut object_96 = object.key("Slate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_96, var_95);
        object_96.finish();
    }
    if let Some(var_97) = &input.splice_insert_message {
        let mut object_98 = object.key("SpliceInsertMessage").start_object();
        crate::json_ser::serialize_structure_crate_model_splice_insert_message(
            &mut object_98,
            var_97,
        );
        object_98.finish();
    }
}

pub fn serialize_structure_crate_model_schedule_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScheduleConfiguration,
) {
    if let Some(var_99) = &input.transition {
        let mut object_100 = object.key("Transition").start_object();
        crate::json_ser::serialize_structure_crate_model_transition(&mut object_100, var_99);
        object_100.finish();
    }
}

pub fn serialize_structure_crate_model_access_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AccessConfiguration,
) {
    if let Some(var_101) = &input.access_type {
        object.key("AccessType").string(var_101.as_str());
    }
    if let Some(var_102) = &input.secrets_manager_access_token_configuration {
        let mut object_103 = object
            .key("SecretsManagerAccessTokenConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_secrets_manager_access_token_configuration(
            &mut object_103,
            var_102,
        );
        object_103.finish();
    }
}

pub fn serialize_structure_crate_model_default_segment_delivery_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DefaultSegmentDeliveryConfiguration,
) {
    if let Some(var_104) = &input.base_url {
        object.key("BaseUrl").string(var_104);
    }
}

pub fn serialize_structure_crate_model_http_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpConfiguration,
) {
    if let Some(var_105) = &input.base_url {
        object.key("BaseUrl").string(var_105);
    }
}

pub fn serialize_structure_crate_model_http_package_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpPackageConfiguration,
) {
    if let Some(var_106) = &input.path {
        object.key("Path").string(var_106);
    }
    if let Some(var_107) = &input.source_group {
        object.key("SourceGroup").string(var_107);
    }
    if let Some(var_108) = &input.r#type {
        object.key("Type").string(var_108.as_str());
    }
}

pub fn serialize_structure_crate_model_avail_suppression(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AvailSuppression,
) {
    if let Some(var_109) = &input.mode {
        object.key("Mode").string(var_109.as_str());
    }
    if let Some(var_110) = &input.value {
        object.key("Value").string(var_110);
    }
}

pub fn serialize_structure_crate_model_bumper(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Bumper,
) {
    if let Some(var_111) = &input.end_url {
        object.key("EndUrl").string(var_111);
    }
    if let Some(var_112) = &input.start_url {
        object.key("StartUrl").string(var_112);
    }
}

pub fn serialize_structure_crate_model_cdn_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CdnConfiguration,
) {
    if let Some(var_113) = &input.ad_segment_url_prefix {
        object.key("AdSegmentUrlPrefix").string(var_113);
    }
    if let Some(var_114) = &input.content_segment_url_prefix {
        object.key("ContentSegmentUrlPrefix").string(var_114);
    }
}

pub fn serialize_structure_crate_model_dash_configuration_for_put(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashConfigurationForPut,
) {
    if let Some(var_115) = &input.mpd_location {
        object.key("MpdLocation").string(var_115);
    }
    if let Some(var_116) = &input.origin_manifest_type {
        object.key("OriginManifestType").string(var_116.as_str());
    }
}

pub fn serialize_structure_crate_model_live_pre_roll_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LivePreRollConfiguration,
) {
    if let Some(var_117) = &input.ad_decision_server_url {
        object.key("AdDecisionServerUrl").string(var_117);
    }
    if input.max_duration_seconds != 0 {
        object.key("MaxDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_duration_seconds).into()),
        );
    }
}

pub fn serialize_structure_crate_model_manifest_processing_rules(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ManifestProcessingRules,
) {
    if let Some(var_118) = &input.ad_marker_passthrough {
        let mut object_119 = object.key("AdMarkerPassthrough").start_object();
        crate::json_ser::serialize_structure_crate_model_ad_marker_passthrough(
            &mut object_119,
            var_118,
        );
        object_119.finish();
    }
}

pub fn serialize_structure_crate_model_dash_playlist_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashPlaylistSettings,
) {
    if input.manifest_window_seconds != 0 {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
    if input.min_buffer_time_seconds != 0 {
        object.key("MinBufferTimeSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.min_buffer_time_seconds).into()),
        );
    }
    if input.min_update_period_seconds != 0 {
        object.key("MinUpdatePeriodSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.min_update_period_seconds).into()),
        );
    }
    if input.suggested_presentation_delay_seconds != 0 {
        object.key("SuggestedPresentationDelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.suggested_presentation_delay_seconds).into()),
        );
    }
}

pub fn serialize_structure_crate_model_hls_playlist_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsPlaylistSettings,
) {
    if input.manifest_window_seconds != 0 {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
}

pub fn serialize_structure_crate_model_splice_insert_message(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SpliceInsertMessage,
) {
    if input.avail_num != 0 {
        object.key("AvailNum").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.avail_num).into()),
        );
    }
    if input.avails_expected != 0 {
        object.key("AvailsExpected").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.avails_expected).into()),
        );
    }
    if input.splice_event_id != 0 {
        object.key("SpliceEventId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.splice_event_id).into()),
        );
    }
    if input.unique_program_id != 0 {
        object.key("UniqueProgramId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.unique_program_id).into()),
        );
    }
}

pub fn serialize_structure_crate_model_transition(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Transition,
) {
    if let Some(var_120) = &input.relative_position {
        object.key("RelativePosition").string(var_120.as_str());
    }
    if let Some(var_121) = &input.relative_program {
        object.key("RelativeProgram").string(var_121);
    }
    if input.scheduled_start_time_millis != 0 {
        object.key("ScheduledStartTimeMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.scheduled_start_time_millis).into()),
        );
    }
    if let Some(var_122) = &input.r#type {
        object.key("Type").string(var_122);
    }
}

pub fn serialize_structure_crate_model_secrets_manager_access_token_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SecretsManagerAccessTokenConfiguration,
) {
    if let Some(var_123) = &input.header_name {
        object.key("HeaderName").string(var_123);
    }
    if let Some(var_124) = &input.secret_arn {
        object.key("SecretArn").string(var_124);
    }
    if let Some(var_125) = &input.secret_string_key {
        object.key("SecretStringKey").string(var_125);
    }
}

pub fn serialize_structure_crate_model_ad_marker_passthrough(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdMarkerPassthrough,
) {
    if input.enabled {
        object.key("Enabled").boolean(input.enabled);
    }
}