// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AlreadyExistsException(crate::error::AlreadyExistsException),
    ClientTokenConflictException(crate::error::ClientTokenConflictException),
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    ConcurrentOperationException(crate::error::ConcurrentOperationException),
    GeneralServiceException(crate::error::GeneralServiceException),
    HandlerFailureException(crate::error::HandlerFailureException),
    HandlerInternalFailureException(crate::error::HandlerInternalFailureException),
    InvalidCredentialsException(crate::error::InvalidCredentialsException),
    InvalidRequestException(crate::error::InvalidRequestException),
    NetworkFailureException(crate::error::NetworkFailureException),
    NotStabilizedException(crate::error::NotStabilizedException),
    NotUpdatableException(crate::error::NotUpdatableException),
    PrivateTypeException(crate::error::PrivateTypeException),
    RequestTokenNotFoundException(crate::error::RequestTokenNotFoundException),
    ResourceConflictException(crate::error::ResourceConflictException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceInternalErrorException(crate::error::ServiceInternalErrorException),
    ServiceLimitExceededException(crate::error::ServiceLimitExceededException),
    ThrottlingException(crate::error::ThrottlingException),
    TypeNotFoundException(crate::error::TypeNotFoundException),
    UnsupportedActionException(crate::error::UnsupportedActionException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlreadyExistsException(inner) => inner.fmt(f),
            Error::ClientTokenConflictException(inner) => inner.fmt(f),
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::ConcurrentOperationException(inner) => inner.fmt(f),
            Error::GeneralServiceException(inner) => inner.fmt(f),
            Error::HandlerFailureException(inner) => inner.fmt(f),
            Error::HandlerInternalFailureException(inner) => inner.fmt(f),
            Error::InvalidCredentialsException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::NetworkFailureException(inner) => inner.fmt(f),
            Error::NotStabilizedException(inner) => inner.fmt(f),
            Error::NotUpdatableException(inner) => inner.fmt(f),
            Error::PrivateTypeException(inner) => inner.fmt(f),
            Error::RequestTokenNotFoundException(inner) => inner.fmt(f),
            Error::ResourceConflictException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceInternalErrorException(inner) => inner.fmt(f),
            Error::ServiceLimitExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::TypeNotFoundException(inner) => inner.fmt(f),
            Error::UnsupportedActionException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CancelResourceRequestError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::CancelResourceRequestError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CancelResourceRequestErrorKind::ConcurrentModificationException(
                    inner,
                ) => Error::ConcurrentModificationException(inner),
                crate::error::CancelResourceRequestErrorKind::RequestTokenNotFoundException(
                    inner,
                ) => Error::RequestTokenNotFoundException(inner),
                crate::error::CancelResourceRequestErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::CreateResourceErrorKind::ClientTokenConflictException(inner) => {
                    Error::ClientTokenConflictException(inner)
                }
                crate::error::CreateResourceErrorKind::ConcurrentOperationException(inner) => {
                    Error::ConcurrentOperationException(inner)
                }
                crate::error::CreateResourceErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::CreateResourceErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::CreateResourceErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::CreateResourceErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::CreateResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::CreateResourceErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::CreateResourceErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::CreateResourceErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::CreateResourceErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::CreateResourceErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::CreateResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateResourceErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::CreateResourceErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::CreateResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateResourceErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::CreateResourceErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::CreateResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::DeleteResourceErrorKind::ClientTokenConflictException(inner) => {
                    Error::ClientTokenConflictException(inner)
                }
                crate::error::DeleteResourceErrorKind::ConcurrentOperationException(inner) => {
                    Error::ConcurrentOperationException(inner)
                }
                crate::error::DeleteResourceErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::DeleteResourceErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::DeleteResourceErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::DeleteResourceErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::DeleteResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DeleteResourceErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::DeleteResourceErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::DeleteResourceErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::DeleteResourceErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::DeleteResourceErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::DeleteResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteResourceErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::DeleteResourceErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::DeleteResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DeleteResourceErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::DeleteResourceErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::DeleteResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::GetResourceErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::GetResourceErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::GetResourceErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::GetResourceErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::GetResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::GetResourceErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::GetResourceErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::GetResourceErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::GetResourceErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::GetResourceErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::GetResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetResourceErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::GetResourceErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::GetResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetResourceErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::GetResourceErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::GetResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetResourceRequestStatusError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetResourceRequestStatusError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetResourceRequestStatusErrorKind::RequestTokenNotFoundException(
                    inner,
                ) => Error::RequestTokenNotFoundException(inner),
                crate::error::GetResourceRequestStatusErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListResourceRequestsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListResourceRequestsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListResourceRequestsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListResourcesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListResourcesErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::ListResourcesErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::ListResourcesErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::ListResourcesErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::ListResourcesErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::ListResourcesErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListResourcesErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::ListResourcesErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::ListResourcesErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::ListResourcesErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::ListResourcesErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::ListResourcesErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListResourcesErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::ListResourcesErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::ListResourcesErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListResourcesErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::ListResourcesErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::ListResourcesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::UpdateResourceErrorKind::ClientTokenConflictException(inner) => {
                    Error::ClientTokenConflictException(inner)
                }
                crate::error::UpdateResourceErrorKind::ConcurrentOperationException(inner) => {
                    Error::ConcurrentOperationException(inner)
                }
                crate::error::UpdateResourceErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::UpdateResourceErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::UpdateResourceErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::UpdateResourceErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::UpdateResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UpdateResourceErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::UpdateResourceErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::UpdateResourceErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::UpdateResourceErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::UpdateResourceErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::UpdateResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateResourceErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::UpdateResourceErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::UpdateResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UpdateResourceErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::UpdateResourceErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::UpdateResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}