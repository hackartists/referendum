#![allow(non_snake_case)]
use super::controller::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn NewTopicPage(lang: Language) -> Element {
    let ctrl = Controller::new()?;

    rsx! {
        div { id: "creation", "new topic" }
    }
}
