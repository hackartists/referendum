#![allow(non_snake_case)]
use crate::components::icons;
use crate::components::icons::RightArrow;
use crate::components::page_title::PageHeader;
use crate::components::page_title::PageHeaderTail;
use crate::route::Route;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::*;

#[component]
pub fn TopicsPage(lang: Language) -> Element {
    let ctrl = Controller::new()?;
    let tr: TopicsTranslate = translate(&lang);

    rsx! {
        div {
            id: "topics",
            class: "flex flex-col justify-start items-start gap-[10px]",
            PageHeader { title: "{tr.title}",
                PageHeaderTail { onclick: move |_| { ctrl.navigate_to_create_topic(&lang) },
                    "{tr.create_topic}"
                    RightArrow {}
                }

            }
            div { class: "w-full flex flex-col justify-start items-start", "body" }
        }
    }
}
