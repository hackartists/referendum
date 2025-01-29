#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::pages::{highlighted_topic_component::HighlightedTopic, i18n::PagesTranslate};

use dioxus_translate::{translate, Language};

#[component]
pub fn HomePage(lang: Language) -> Element {
    let ctrl = super::controller::Controller::new(lang)?;
    let _tr: PagesTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col gap-[100px]",
            if let Some(ref topic) = ctrl.topic() {
                HighlightedTopic {
                    id: "topic-{topic.id}",
                    title: "{topic.title}",
                    content: "{topic.content}",
                    period: topic.period(),
                    yes: 10,
                    requirement: topic.requirement,
                    amount: 10000000,
                    lang,
                }
            }
        }
    }
}
