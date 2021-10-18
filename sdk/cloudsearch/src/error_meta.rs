// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    BaseException(crate::error::BaseException),
    DisabledOperationException(crate::error::DisabledOperationException),
    InternalException(crate::error::InternalException),
    InvalidTypeException(crate::error::InvalidTypeException),
    LimitExceededException(crate::error::LimitExceededException),
    ResourceAlreadyExistsException(crate::error::ResourceAlreadyExistsException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BaseException(inner) => inner.fmt(f),
            Error::DisabledOperationException(inner) => inner.fmt(f),
            Error::InternalException(inner) => inner.fmt(f),
            Error::InvalidTypeException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceAlreadyExistsException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::BuildSuggestersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::BuildSuggestersError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BuildSuggestersErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::BuildSuggestersErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::BuildSuggestersErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::BuildSuggestersErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateDomainError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateDomainErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::CreateDomainErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::CreateDomainErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateDomainErrorKind::ResourceAlreadyExistsException(inner) => {
                    Error::ResourceAlreadyExistsException(inner)
                }
                crate::error::CreateDomainErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateDomainErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DefineAnalysisSchemeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DefineAnalysisSchemeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DefineAnalysisSchemeErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DefineExpressionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DefineExpressionError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DefineExpressionErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DefineExpressionErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DefineExpressionErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DefineExpressionErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DefineExpressionErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DefineExpressionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DefineIndexFieldError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DefineIndexFieldError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DefineIndexFieldErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DefineSuggesterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DefineSuggesterError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DefineSuggesterErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DefineSuggesterErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DefineSuggesterErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DefineSuggesterErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DefineSuggesterErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DefineSuggesterErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteAnalysisSchemeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DeleteAnalysisSchemeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteAnalysisSchemeErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteAnalysisSchemeErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteAnalysisSchemeErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DeleteAnalysisSchemeErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteAnalysisSchemeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteDomainError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteDomainErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteDomainErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteDomainErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteExpressionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteExpressionError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteExpressionErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteExpressionErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteExpressionErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DeleteExpressionErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteExpressionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteIndexFieldError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteIndexFieldError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteIndexFieldErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteIndexFieldErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteIndexFieldErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DeleteIndexFieldErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteIndexFieldErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteSuggesterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteSuggesterError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteSuggesterErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteSuggesterErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteSuggesterErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DeleteSuggesterErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteSuggesterErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeAnalysisSchemesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeAnalysisSchemesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAnalysisSchemesErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeAnalysisSchemesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeAnalysisSchemesErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeAnalysisSchemesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeAvailabilityOptionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeAvailabilityOptionsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAvailabilityOptionsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeAvailabilityOptionsErrorKind::DisabledOperationException(
                    inner,
                ) => Error::DisabledOperationException(inner),
                crate::error::DescribeAvailabilityOptionsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeAvailabilityOptionsErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DescribeAvailabilityOptionsErrorKind::LimitExceededException(
                    inner,
                ) => Error::LimitExceededException(inner),
                crate::error::DescribeAvailabilityOptionsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeAvailabilityOptionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeDomainEndpointOptionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeDomainEndpointOptionsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeDomainEndpointOptionsErrorKind::BaseException(inner) => Error::BaseException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::DisabledOperationException(inner) => Error::DisabledOperationException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::InternalException(inner) => Error::InternalException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeDomainsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeDomainsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeDomainsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeDomainsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeDomainsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeExpressionsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeExpressionsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeExpressionsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeExpressionsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeExpressionsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeExpressionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeIndexFieldsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeIndexFieldsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeIndexFieldsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeIndexFieldsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeIndexFieldsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeIndexFieldsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeScalingParametersError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeScalingParametersError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeScalingParametersErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeScalingParametersErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeScalingParametersErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeScalingParametersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeServiceAccessPoliciesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeServiceAccessPoliciesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeServiceAccessPoliciesErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeServiceAccessPoliciesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeServiceAccessPoliciesErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeServiceAccessPoliciesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeSuggestersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeSuggestersError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeSuggestersErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeSuggestersErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeSuggestersErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeSuggestersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::IndexDocumentsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::IndexDocumentsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::IndexDocumentsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::IndexDocumentsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::IndexDocumentsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::IndexDocumentsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListDomainNamesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListDomainNamesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDomainNamesErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::ListDomainNamesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateAvailabilityOptionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateAvailabilityOptionsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateAvailabilityOptionsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::DisabledOperationException(
                    inner,
                ) => Error::DisabledOperationException(inner),
                crate::error::UpdateAvailabilityOptionsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateAvailabilityOptionsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateDomainEndpointOptionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateDomainEndpointOptionsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateDomainEndpointOptionsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::UpdateDomainEndpointOptionsErrorKind::DisabledOperationException(
                    inner,
                ) => Error::DisabledOperationException(inner),
                crate::error::UpdateDomainEndpointOptionsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateDomainEndpointOptionsErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::UpdateDomainEndpointOptionsErrorKind::LimitExceededException(
                    inner,
                ) => Error::LimitExceededException(inner),
                crate::error::UpdateDomainEndpointOptionsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateDomainEndpointOptionsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateDomainEndpointOptionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateScalingParametersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateScalingParametersError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateScalingParametersErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::UpdateScalingParametersErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateScalingParametersErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::UpdateScalingParametersErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateScalingParametersErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateScalingParametersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateServiceAccessPoliciesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateServiceAccessPoliciesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateServiceAccessPoliciesErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::UpdateServiceAccessPoliciesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateServiceAccessPoliciesErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::UpdateServiceAccessPoliciesErrorKind::LimitExceededException(
                    inner,
                ) => Error::LimitExceededException(inner),
                crate::error::UpdateServiceAccessPoliciesErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateServiceAccessPoliciesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}