use super::Settings;
use crate::constants::OFFICIAL_ADDONS;
use crate::types::addons::Descriptor;
use crate::types::api::Auth;
use crate::types::UID;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub auth: Option<Auth>,
    pub addons: Vec<Descriptor>,
    pub settings: Settings,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            auth: None,
            addons: OFFICIAL_ADDONS.to_owned(),
            settings: Settings::default(),
        }
    }
}

impl Profile {
    pub fn uid(&self) -> UID {
        UID(self.auth.as_ref().map(|auth| auth.user.id.to_owned()))
    }
}
