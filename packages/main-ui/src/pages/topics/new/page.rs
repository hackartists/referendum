#![allow(non_snake_case)]
use crate::{
    components::{
        button::{PrimaryButton, TextButton},
        inputs::LabeledInput,
    },
    pages::topics::new::i18n::NewTopicTranslate,
};

use super::controller::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn NewTopicPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: NewTopicTranslate = translate(&lang);

    rsx! {
        div {
            id: "creation",
            class: "w-full flex flex-col gap-[10px] items-start justify-start",

            div { class: "w-full h-[30px] flex-col justify-center items-center inline-flex",
                div { class: "self-stretch py-[3px] border-b border-[#414462] justify-center items-center gap-2.5 inline-flex",
                    div { class: "grow shrink basis-0 text-white text-xl font-semibold font-['Inter']",
                        "{tr.title}"
                    }
                }
            }

            div { class: "w-full flex flex-col gap-[10px] py-[10px]",
                LabeledInput {
                    title: tr.topic_title,
                    required: true,
                    oninput: move |e| ctrl.title.set(e),
                }
                LabeledInput {
                    title: tr.topic_start,
                    r#type: "date",
                    required: true,
                    oninput: move |e| ctrl.handle_start(e),
                }
                LabeledInput {
                    title: tr.topic_end,
                    r#type: "date",
                    required: true,
                    oninput: move |e| ctrl.handle_end(e),
                }
                LabeledInput {
                    title: tr.topic_requirement,
                    r#type: "number",
                    required: true,
                    oninput: move |e| ctrl.handle_requirement(e),
                }
                LabeledInput {
                    title: tr.topic_content,
                    oninput: move |e| ctrl.content.set(e),
                    height: "150px",
                }
            }

            div { class: "w-full flex flex-row gap-[30px] justify-center items-center",
                TextButton { class: "w-[400px]", onclick: move |_| ctrl.cancel(), "{tr.cancel}" }
                PrimaryButton {
                    class: "w-[400px]",
                    onclick: move |_| async move { ctrl.submit().await },
                    "{tr.submit}"
                }
            }
        }
    }
}
