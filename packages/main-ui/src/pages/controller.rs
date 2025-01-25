use dioxus_aws::prelude::*;
use dto::*;

use crate::config;

#[derive(Debug, Clone, Copy)]
pub struct Controller {}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let conf = config::get();

        let ctrl = Self {};
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}
