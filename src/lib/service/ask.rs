use crate::domain::clip::field;
use crate::Shortcode;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
    pub shortcode: field::Shortcode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shortcode: Shortcode,
    pub password: field::Password, 
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: Shortcode::from(shortcode),
            password: field::Password::default()
        }
    }
}

impl From<Shortcode> for GetClip {
    fn from(shortcode: Shortcode) -> Self {
        Self {
            shortcode,
            password: field::Password::default()
        }
    }
}

impl From<&str> for GetClip {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}
