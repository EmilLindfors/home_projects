use crate::error::{Error, HttpError, Result, ValidationErrorItem};
use validator::Validate;
use axum::{
    async_trait,
    body::{HttpBody},
    extract::{FromRequest, RequestParts},
    BoxError,
};
use serde::{de::DeserializeOwned};
use std::{borrow::Cow, error::Error as StdError};

/// Recursively searches a [`validator::ValidationErrors`] tree into a linear list of errors to be
/// sent to the user
fn validation_errors(
    acc_path: Vec<String>,
    err: validator::ValidationErrors,
) -> Vec<ValidationErrorItem> {
    err.into_errors()
        .into_iter()
        .flat_map(|(k, v)| {
            // Add the next field to the location
            let mut loc = acc_path.clone();
            loc.push(k.to_owned());

            match v {
                // If it's a [`validator::ValidationErrorsKind::Field`], then it will be a vector of
                // errors to map to [`ValidationErrorItem`]s and insert to [`out`] before the next
                // iteration
                validator::ValidationErrorsKind::Field(vec) => vec
                    .into_iter()
                    .map(|err| ValidationErrorItem {
                        loc: loc.clone(),
                        msg: err
                            .message
                            .unwrap_or(Cow::Borrowed("Validation error"))
                            .to_string(),
                        ty: "value_error".to_owned(),
                    })
                    .collect(),
                // If it is a [`validator::ValidationErrorsKind::Struct`], then it will be another
                // [`validator::ValidationErrors`] to search
                validator::ValidationErrorsKind::Struct(errors) => validation_errors(loc, *errors),

                // If it is a [`validator::ValidationErrorsKind::List`], then it will be an
                // [`std::collections::BTreeMap`] of [`validator::ValidationErrors`] to search
                validator::ValidationErrorsKind::List(map) => map
                    .into_iter()
                    .flat_map(|(k, v)| {
                        // Add the list index to the location
                        let mut loc = loc.clone();
                        loc.push(format!("[{}]", k));

                        validation_errors(loc, *v)
                    })
                    .collect(),
            }
        })
        .collect()
}


#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let b = bytes::Bytes::from_request(req).await.map_err(|e| {
            tracing::error!("Error reading body as bytes: {}", e);
            HttpError::internal_server_errer(None, Some("Failed to read request body".to_owned()))
        })?;
        let mut de = serde_json::Deserializer::from_slice(&b);

        let value: T = serde_path_to_error::deserialize(&mut de).map_err(|e| {
            let mut path = e
                .path()
                .to_string()
                .split('.')
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>();
            let inner = e.inner();

            let mut loc = vec!["body".to_owned()];
            loc.append(&mut path);
            HttpError::unprocessable_entity(vec![ValidationErrorItem {
                loc,
                msg: inner
                    .source()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| e.to_string()),
                ty: "value_error.jsondecode".to_owned(),
            }])
        })?;

        value.validate().map_err(|e| {
            HttpError::unprocessable_entity(validation_errors(vec!["body".to_owned()], e))
        })?;
        Ok(ValidatedJson(value))
    }
}