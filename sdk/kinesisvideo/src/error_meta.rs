// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    AccountChannelLimitExceededException(crate::error::AccountChannelLimitExceededException),
    AccountStreamLimitExceededException(crate::error::AccountStreamLimitExceededException),
    ClientLimitExceededException(crate::error::ClientLimitExceededException),
    DeviceStreamLimitExceededException(crate::error::DeviceStreamLimitExceededException),
    InvalidArgumentException(crate::error::InvalidArgumentException),
    InvalidDeviceException(crate::error::InvalidDeviceException),
    InvalidResourceFormatException(crate::error::InvalidResourceFormatException),
    NotAuthorizedException(crate::error::NotAuthorizedException),
    ResourceInUseException(crate::error::ResourceInUseException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    TagsPerResourceExceededLimitException(crate::error::TagsPerResourceExceededLimitException),
    VersionMismatchException(crate::error::VersionMismatchException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::AccountChannelLimitExceededException(inner) => inner.fmt(f),
            Error::AccountStreamLimitExceededException(inner) => inner.fmt(f),
            Error::ClientLimitExceededException(inner) => inner.fmt(f),
            Error::DeviceStreamLimitExceededException(inner) => inner.fmt(f),
            Error::InvalidArgumentException(inner) => inner.fmt(f),
            Error::InvalidDeviceException(inner) => inner.fmt(f),
            Error::InvalidResourceFormatException(inner) => inner.fmt(f),
            Error::NotAuthorizedException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TagsPerResourceExceededLimitException(inner) => inner.fmt(f),
            Error::VersionMismatchException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateSignalingChannelError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::CreateSignalingChannelError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateSignalingChannelErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateSignalingChannelErrorKind::AccountChannelLimitExceededException(inner) => Error::AccountChannelLimitExceededException(inner),
                crate::error::CreateSignalingChannelErrorKind::ClientLimitExceededException(inner) => Error::ClientLimitExceededException(inner),
                crate::error::CreateSignalingChannelErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
                crate::error::CreateSignalingChannelErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
                crate::error::CreateSignalingChannelErrorKind::TagsPerResourceExceededLimitException(inner) => Error::TagsPerResourceExceededLimitException(inner),
                crate::error::CreateSignalingChannelErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateStreamErrorKind::AccountStreamLimitExceededException(inner) => {
                    Error::AccountStreamLimitExceededException(inner)
                }
                crate::error::CreateStreamErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::CreateStreamErrorKind::DeviceStreamLimitExceededException(inner) => {
                    Error::DeviceStreamLimitExceededException(inner)
                }
                crate::error::CreateStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::CreateStreamErrorKind::InvalidDeviceException(inner) => {
                    Error::InvalidDeviceException(inner)
                }
                crate::error::CreateStreamErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::CreateStreamErrorKind::TagsPerResourceExceededLimitException(
                    inner,
                ) => Error::TagsPerResourceExceededLimitException(inner),
                crate::error::CreateStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteSignalingChannelError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DeleteSignalingChannelError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteSignalingChannelErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeleteSignalingChannelErrorKind::ClientLimitExceededException(
                    inner,
                ) => Error::ClientLimitExceededException(inner),
                crate::error::DeleteSignalingChannelErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::DeleteSignalingChannelErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::DeleteSignalingChannelErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteSignalingChannelErrorKind::VersionMismatchException(inner) => {
                    Error::VersionMismatchException(inner)
                }
                crate::error::DeleteSignalingChannelErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteStreamErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::DeleteStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::DeleteStreamErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DeleteStreamErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::DeleteStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteStreamErrorKind::VersionMismatchException(inner) => {
                    Error::VersionMismatchException(inner)
                }
                crate::error::DeleteStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeSignalingChannelError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeSignalingChannelError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeSignalingChannelErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DescribeSignalingChannelErrorKind::ClientLimitExceededException(
                    inner,
                ) => Error::ClientLimitExceededException(inner),
                crate::error::DescribeSignalingChannelErrorKind::InvalidArgumentException(
                    inner,
                ) => Error::InvalidArgumentException(inner),
                crate::error::DescribeSignalingChannelErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeSignalingChannelErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeStreamErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::DescribeStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::DescribeStreamErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DescribeStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetDataEndpointError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetDataEndpointError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDataEndpointErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::GetDataEndpointErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::GetDataEndpointErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::GetDataEndpointErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetDataEndpointErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetSignalingChannelEndpointError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetSignalingChannelEndpointError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSignalingChannelEndpointErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetSignalingChannelEndpointErrorKind::ClientLimitExceededException(inner) => Error::ClientLimitExceededException(inner),
                crate::error::GetSignalingChannelEndpointErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
                crate::error::GetSignalingChannelEndpointErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
                crate::error::GetSignalingChannelEndpointErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetSignalingChannelEndpointErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListSignalingChannelsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListSignalingChannelsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListSignalingChannelsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListSignalingChannelsErrorKind::ClientLimitExceededException(
                    inner,
                ) => Error::ClientLimitExceededException(inner),
                crate::error::ListSignalingChannelsErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::ListSignalingChannelsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListStreamsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListStreamsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListStreamsErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::ListStreamsErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::ListStreamsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListTagsForStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForStreamErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::ListTagsForStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::ListTagsForStreamErrorKind::InvalidResourceFormatException(inner) => {
                    Error::InvalidResourceFormatException(inner)
                }
                crate::error::ListTagsForStreamErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::ListTagsForStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListTagsForStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::TagResourceErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::TagResourceErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::TagsPerResourceExceededLimitException(
                    inner,
                ) => Error::TagsPerResourceExceededLimitException(inner),
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::TagStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::TagStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagStreamErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::TagStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::TagStreamErrorKind::InvalidResourceFormatException(inner) => {
                    Error::InvalidResourceFormatException(inner)
                }
                crate::error::TagStreamErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::TagStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagStreamErrorKind::TagsPerResourceExceededLimitException(inner) => {
                    Error::TagsPerResourceExceededLimitException(inner)
                }
                crate::error::TagStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UntagResourceErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::UntagResourceErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UntagStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UntagStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagStreamErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::UntagStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::UntagStreamErrorKind::InvalidResourceFormatException(inner) => {
                    Error::InvalidResourceFormatException(inner)
                }
                crate::error::UntagStreamErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::UntagStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateDataRetentionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateDataRetentionError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateDataRetentionErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::UpdateDataRetentionErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::UpdateDataRetentionErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::UpdateDataRetentionErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UpdateDataRetentionErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateDataRetentionErrorKind::VersionMismatchException(inner) => {
                    Error::VersionMismatchException(inner)
                }
                crate::error::UpdateDataRetentionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateSignalingChannelError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateSignalingChannelError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateSignalingChannelErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateSignalingChannelErrorKind::ClientLimitExceededException(
                    inner,
                ) => Error::ClientLimitExceededException(inner),
                crate::error::UpdateSignalingChannelErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::UpdateSignalingChannelErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UpdateSignalingChannelErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateSignalingChannelErrorKind::VersionMismatchException(inner) => {
                    Error::VersionMismatchException(inner)
                }
                crate::error::UpdateSignalingChannelErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateStreamErrorKind::ClientLimitExceededException(inner) => {
                    Error::ClientLimitExceededException(inner)
                }
                crate::error::UpdateStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::UpdateStreamErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::UpdateStreamErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UpdateStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateStreamErrorKind::VersionMismatchException(inner) => {
                    Error::VersionMismatchException(inner)
                }
                crate::error::UpdateStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}