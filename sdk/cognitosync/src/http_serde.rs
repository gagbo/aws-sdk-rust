// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_update_records(
    input: &crate::input::UpdateRecordsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.client_context {
        let formatted_2 = AsRef::<str>::as_ref(inner_1);
        if !formatted_2.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "client_context",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-Client-Context", header_value);
        }
    }
    Ok(builder)
}