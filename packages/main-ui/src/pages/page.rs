#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::pages::i18n::PagesTranslate;

use dioxus_translate::{translate, Language};

#[component]
pub fn HomePage(lang: Language) -> Element {
    let ctrl = super::controller::Controller::new()?;
    let _tr: PagesTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col gap-[100px] grid grid-cols-1 mb-[20px]" }
    }
}
