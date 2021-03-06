use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct Subtitles {
    pub id: String,
    // @TODO: ISO 639-2
    pub lang: String,
    pub url: Url,
}
