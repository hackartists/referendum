#![allow(unused)]
use by_types::QueryParam;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::*;

use crate::route::Route;

#[derive(Clone, Copy)]
pub struct Controller {
    pub size: usize,
    pub nav: Navigator,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let size = 10;

        let ctrl = Self {
            size,
            nav: use_navigator(),
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn navigate_to_create_topic(&self, lang: &Language) {
        self.nav.push(Route::NewTopicPage { lang: lang.clone() });
    }
}
