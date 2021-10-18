// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    InvalidArgumentException(crate::error::InvalidArgumentException),
    InvalidKmsResourceException(crate::error::InvalidKmsResourceException),
    LimitExceededException(crate::error::LimitExceededException),
    ResourceInUseException(crate::error::ResourceInUseException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::InvalidArgumentException(inner) => inner.fmt(f),
            Error::InvalidKmsResourceException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateDeliveryStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::CreateDeliveryStreamError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateDeliveryStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::CreateDeliveryStreamErrorKind::InvalidKmsResourceException(inner) => {
                    Error::InvalidKmsResourceException(inner)
                }
                crate::error::CreateDeliveryStreamErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateDeliveryStreamErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::CreateDeliveryStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteDeliveryStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DeleteDeliveryStreamError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteDeliveryStreamErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::DeleteDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteDeliveryStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeDeliveryStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeDeliveryStreamError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeDeliveryStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListDeliveryStreamsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListDeliveryStreamsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDeliveryStreamsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListTagsForDeliveryStreamError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListTagsForDeliveryStreamError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForDeliveryStreamErrorKind::InvalidArgumentException(
                    inner,
                ) => Error::InvalidArgumentException(inner),
                crate::error::ListTagsForDeliveryStreamErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::ListTagsForDeliveryStreamErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::ListTagsForDeliveryStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::PutRecordError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::PutRecordError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutRecordErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::PutRecordErrorKind::InvalidKmsResourceException(inner) => {
                    Error::InvalidKmsResourceException(inner)
                }
                crate::error::PutRecordErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::PutRecordErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::PutRecordErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::PutRecordBatchError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::PutRecordBatchError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutRecordBatchErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::PutRecordBatchErrorKind::InvalidKmsResourceException(inner) => {
                    Error::InvalidKmsResourceException(inner)
                }
                crate::error::PutRecordBatchErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::PutRecordBatchErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::PutRecordBatchErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::StartDeliveryStreamEncryptionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartDeliveryStreamEncryptionError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartDeliveryStreamEncryptionErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
                crate::error::StartDeliveryStreamEncryptionErrorKind::InvalidKmsResourceException(inner) => Error::InvalidKmsResourceException(inner),
                crate::error::StartDeliveryStreamEncryptionErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::StartDeliveryStreamEncryptionErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
                crate::error::StartDeliveryStreamEncryptionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::StartDeliveryStreamEncryptionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::StopDeliveryStreamEncryptionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::StopDeliveryStreamEncryptionError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopDeliveryStreamEncryptionErrorKind::InvalidArgumentException(
                    inner,
                ) => Error::InvalidArgumentException(inner),
                crate::error::StopDeliveryStreamEncryptionErrorKind::LimitExceededException(
                    inner,
                ) => Error::LimitExceededException(inner),
                crate::error::StopDeliveryStreamEncryptionErrorKind::ResourceInUseException(
                    inner,
                ) => Error::ResourceInUseException(inner),
                crate::error::StopDeliveryStreamEncryptionErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::StopDeliveryStreamEncryptionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::TagDeliveryStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::TagDeliveryStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagDeliveryStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::TagDeliveryStreamErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::TagDeliveryStreamErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::TagDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagDeliveryStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UntagDeliveryStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UntagDeliveryStreamError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagDeliveryStreamErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::UntagDeliveryStreamErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UntagDeliveryStreamErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UntagDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagDeliveryStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateDestinationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateDestinationError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateDestinationErrorKind::ConcurrentModificationException(
                    inner,
                ) => Error::ConcurrentModificationException(inner),
                crate::error::UpdateDestinationErrorKind::InvalidArgumentException(inner) => {
                    Error::InvalidArgumentException(inner)
                }
                crate::error::UpdateDestinationErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UpdateDestinationErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateDestinationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}