use anyhow::{bail, Result};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Request has encountered an error. {0} - {1} ")]
pub struct ApiRequestError<'a>(u32, &'a str);

impl ApiRequestError<'_> {
    pub fn handler_status(status_code: StatusCode) -> Result<()> {
        match status_code {
            StatusCode::BAD_REQUEST => bail!(ApiRequestError(400, "Invalid request, please make sure you follow the documentation for proper syntax.")),
            StatusCode::UNAUTHORIZED => bail!(ApiRequestError(401, "Problem authenticating the user.")),
            StatusCode::FORBIDDEN => bail!(ApiRequestError(403, "User was authenticated, but access denied due to lack of permission or rate limiting.")),
            StatusCode::SERVICE_UNAVAILABLE => bail!(ApiRequestError(502, "Pocket's sync server is down for scheduled maintenance.")),
            _ => Ok(()),
        }
    }
}
