#![allow(non_snake_case)]
use crate::components::icons::RightArrow;
use crate::components::page_title::PageHeader;
use crate::components::page_title::PageHeaderTail;

use super::controller::*;
use super::i18n::*;
use by_components::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn TopicsPage(lang: Language) -> Element {
    let ctrl = Controller::new()?;
    let tr: TopicsTranslate = translate(&lang);

    let color: ColorTheme = use_context();

    rsx! {
        div {
            id: "topics",
            class: "flex flex-col justify-start items-start gap-[20px]",
            div { class: "w-full flex flex-col",
                PageHeader { title: "{tr.title}",
                    PageHeaderTail { onclick: move |_| { ctrl.navigate_to_create_topic(&lang) },
                        "{tr.create_topic}"
                        RightArrow {}
                    }
                }
                div { class: "text-[12px]", "Results: {ctrl.total_count()}" }
            }
            div { class: "w-full flex flex-col justify-start items-start gap-[5px]",

                for topic in ctrl.topics() {
                    button { class: "w-full flex flex-col justify-start items-start gap-[10px] rounded-md p-4 bg-[{color.card.primary}] hover:border hover:border-[{color.card.secondary}] shadow-md",
                        div { class: "hover-effect w-full flex flex-row justify-between items-center",
                            div { class: "text-lg font-bold", "{topic.title}" }
                        }
                        div { class: "hover-effect w-full flex flex-row justify-between items-center",
                            div { class: "text-sm", "{topic.content}" }
                        }
                        div { class: "hover-effect w-full flex flex-row justify-between items-center",
                            div { class: "text-sm", "{topic.period()}" }
                            div { class: "text-sm", "{topic.requirement} {tr.unit}" }
                        }
                    }
                }
            }
        }
    }
}
