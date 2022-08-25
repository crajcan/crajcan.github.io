use derive_more::{Display, Error};
use std::error::Error;

#[derive(Debug, Display, Error)]
struct myError {
    message: String,
}

impl From<JsValue> for myError {
    fn from(error: JsValue) -> Self {
        myError {
            message: format!("{:?}", error),
        }
    }
}
