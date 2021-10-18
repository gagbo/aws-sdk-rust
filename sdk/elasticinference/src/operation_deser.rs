// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_accelerator_offerings_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAcceleratorOfferingsOutput,
    crate::error::DescribeAcceleratorOfferingsError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeAcceleratorOfferingsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DescribeAcceleratorOfferingsError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::DescribeAcceleratorOfferingsError {
            meta: generic,
            kind: crate::error::DescribeAcceleratorOfferingsErrorKind::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAcceleratorOfferingsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::DescribeAcceleratorOfferingsError {
            meta: generic,
            kind: crate::error::DescribeAcceleratorOfferingsErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAcceleratorOfferingsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::DescribeAcceleratorOfferingsError {
            meta: generic,
            kind: crate::error::DescribeAcceleratorOfferingsErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAcceleratorOfferingsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeAcceleratorOfferingsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_accelerator_offerings_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAcceleratorOfferingsOutput,
    crate::error::DescribeAcceleratorOfferingsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_accelerator_offerings_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_accelerator_offerings(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeAcceleratorOfferingsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_accelerators_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAcceleratorsOutput,
    crate::error::DescribeAcceleratorsError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeAcceleratorsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeAcceleratorsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::DescribeAcceleratorsError {
            meta: generic,
            kind: crate::error::DescribeAcceleratorsErrorKind::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAcceleratorsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::DescribeAcceleratorsError {
            meta: generic,
            kind: crate::error::DescribeAcceleratorsErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAcceleratorsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::DescribeAcceleratorsError {
            meta: generic,
            kind: crate::error::DescribeAcceleratorsErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAcceleratorsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeAcceleratorsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_accelerators_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAcceleratorsOutput,
    crate::error::DescribeAcceleratorsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_accelerators_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_accelerators(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeAcceleratorsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_accelerator_types_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAcceleratorTypesOutput,
    crate::error::DescribeAcceleratorTypesError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeAcceleratorTypesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DescribeAcceleratorTypesError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::DescribeAcceleratorTypesError {
            meta: generic,
            kind: crate::error::DescribeAcceleratorTypesErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAcceleratorTypesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeAcceleratorTypesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_accelerator_types_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAcceleratorTypesOutput,
    crate::error::DescribeAcceleratorTypesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_accelerator_types_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_accelerator_types(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeAcceleratorTypesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_tags_for_resource_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::ListTagsForResourceOutput,
    crate::error::ListTagsForResourceError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListTagsForResourceError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListTagsForResourceError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::ListTagsForResourceError {
            meta: generic,
            kind: crate::error::ListTagsForResourceErrorKind::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::ListTagsForResourceError {
            meta: generic,
            kind: crate::error::ListTagsForResourceErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::ListTagsForResourceError {
            meta: generic,
            kind: crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListTagsForResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_tags_for_resource_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::ListTagsForResourceOutput,
    crate::error::ListTagsForResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_tags_for_resource_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_tags_for_resource(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::ListTagsForResourceError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_tag_resource_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::TagResourceError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::TagResourceError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::TagResourceError {
            meta: generic,
            kind: crate::error::TagResourceErrorKind::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::TagResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::TagResourceError {
            meta: generic,
            kind: crate::error::TagResourceErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::TagResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::TagResourceError {
            meta: generic,
            kind: crate::error::TagResourceErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::TagResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::TagResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_tag_resource_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::tag_resource_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_untag_resource_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::UntagResourceError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::UntagResourceError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::UntagResourceError {
            meta: generic,
            kind: crate::error::UntagResourceErrorKind::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::UntagResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::UntagResourceError {
            meta: generic,
            kind: crate::error::UntagResourceErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::UntagResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::UntagResourceError {
            meta: generic,
            kind: crate::error::UntagResourceErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::UntagResourceError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::UntagResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_untag_resource_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::untag_resource_output::Builder::default();
        let _ = response;
        output.build()
    })
}