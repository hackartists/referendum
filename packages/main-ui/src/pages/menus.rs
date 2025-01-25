#![allow(non_snake_case)]
use super::i18n::*;
use crate::{route::*, services::user_service::UserService};
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::*;

#[component]
pub fn Menus(
    #[props(default ="menus".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
) -> Element {
    let tr: MenusTranslate = translate(&lang);
    let user: UserService = use_context();

    rsx! {
        div { id, class,
            div { class: "flex flex-row rounded-full bg-[#323342]",
                ScrollLink { "{tr.home}" }
                ScrollLink { "{tr.about}" }
                ScrollLink { "{tr.contact_us}" }

                if user.role() == UserRole::Admin {
                    MenuItem { to: Route::TopicsPage { lang }, "{tr.topic}" }
                }
            }
        }
    }
}

#[component]
pub fn MenuItem(#[props(into)] to: NavigationTarget, children: Element) -> Element {
    rsx! {
        Link {
            class: "px-[10px] py-[5px] text-[15px] font-light leading-[22.5px] text-[#adbcd7] hover:text-white hover:bg-[#424563] rounded-full",
            to,
            {children}
        }
    }
}

#[component]
pub fn ScrollLink(children: Element) -> Element {
    rsx! {
        div { class: "px-[10px] py-[5px] text-[15px] font-light leading-[22.5px] text-[#adbcd7] hover:text-white hover:bg-[#424563] rounded-full cursor-pointer",
            {children}

        }
    }
}

#[component]
pub fn Test(
    #[props(default ="test".to_string())] id: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { id, ..attributes, "Test" }
    }
}
