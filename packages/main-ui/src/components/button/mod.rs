#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;

use crate::{
    components::icons::{self, Close},
    theme::Theme,
};

#[component]
pub fn TextButton(
    children: Element,
    onclick: EventHandler<Event<MouseData>>,
    #[props(default = "".to_string())] class: String,
) -> Element {
    let color: ColorTheme = use_context();
    let bg_hover = &color.button.secondary;
    let text_color = &color.button.secondary;
    let text_hover = &color.text.primary;

    rsx! {
        div { class,
            button {
                class: "w-full text-[16px] text-[{text_color}] font-bold px-[16px] py-[10px] opacity-70 hover:opacity-100 cursor-pointer rounded-[8px] hover:bg-[{bg_hover}] hover:text-[{text_hover}]",
                onclick: move |evt| onclick.call(evt),
                {children}
            }
        }
    }
}

#[component]
pub fn PrimaryButton(
    children: Element,
    color: Option<String>,
    background: Option<String>,
    onclick: EventHandler<Event<MouseData>>,
    #[props(default = "".to_string())] class: String,
) -> Element {
    let color: ColorTheme = use_context();
    let bg = match background {
        Some(bg) => bg,
        None => color.button.primary,
    };

    let hover_color = &color.button.secondary;
    let text_color = &color.text.primary;

    rsx! {
        div { class,
            button {
                class: "w-full text-[16px] text-[{text_color}] bg-[{bg}] font-bold px-[16px] py-[10px] opacity-70 hover:opacity-100 cursor-pointer rounded-[8px] hover:bg-[{hover_color}]",
                onclick: move |evt| onclick.call(evt),
                color: color.text.primary,
                {children}
            }
        }
    }
}

#[component]
pub fn RoundedYesButton(
    #[props(default = false)] disabled: bool,
    onclick: Option<EventHandler<Event<MouseData>>>,
    #[props(default = 100)] rounded: i32,
    #[props(default = "w-[291px]".to_string())] class: String,
    children: Element,
) -> Element {
    let theme_service: Theme = use_context();
    let theme: crate::theme::ThemeData = theme_service.get_data();
    let mut hover = use_signal(|| false);

    let color = if hover() && onclick.is_some() {
        theme.grey00.as_str()
    } else {
        theme.active.as_str()
    };
    let bg = if disabled {
        "rgba(141, 255, 88, 0.05)"
    } else if hover() && onclick.is_some() {
        theme.active_true.as_str()
    } else {
        "rgba(141, 255, 88, 0.2)"
    };
    let border = if hover() && onclick.is_some() {
        theme.active_true.as_str()
    } else {
        "rgba(141, 255, 88, 0.2)"
    };
    let border_class = if disabled {
        "border-[0px]"
    } else {
        "border-[1px]"
    };
    let icon = if disabled {
        rsx! {
            icons::FilledVoteYes {}
        }
    } else if hover() && onclick.is_some() {
        rsx! {
            icons::FilledVoteYes { color: theme.grey00.as_str() }
        }
    } else {
        rsx! {
            icons::OutlinedVoteYes {}
        }
    };

    rsx! {
        div { class,
            div {
                class: "w-full max-w-[600px] flex flex-col items-center justify-center rounded-[{rounded}px] {border_class} py-[8px] hover:bg-black cursor-pointer",
                onclick: move |evt| {
                    if onclick.is_some() {
                        onclick.unwrap().call(evt)
                    }
                },
                onmouseenter: move |_| {
                    tracing::debug!("hover");
                    if !disabled {
                        hover.set(true)
                    }
                },
                onmouseleave: move |_| { hover.set(false) },
                color,
                background: bg,
                border_color: border,
                div { class: "flex items-center justify-center gap-[10px]",
                    span { class: "text-[15px] font-bold", {children} }
                    {icon}
                }
            }
        }
    }
}

#[component]
pub fn RoundedNoButton(
    #[props(default = false)] disabled: bool,
    onclick: Option<EventHandler<Event<MouseData>>>,
    #[props(default = 100)] rounded: i32,
    #[props(default = "w-[291px]".to_string())] class: String,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let mut hover = use_signal(|| false);

    let color = if hover() && onclick.is_some() {
        theme.grey00.as_str()
    } else {
        theme.active01.as_str()
    };
    let bg = if disabled {
        "rgba(255, 66, 69, 0.05)"
    } else if hover() && onclick.is_some() {
        theme.active01.as_str()
    } else {
        "rgba(255, 66, 69, 0.2)"
    };
    let border = if hover() && onclick.is_some() {
        theme.active01.as_str()
    } else {
        "rgba(255, 66, 69, 0.2)"
    };
    let border_class = if disabled {
        "border-[0px]"
    } else {
        "border-[1px]"
    };
    let icon = if disabled {
        rsx! {
            icons::FilledVoteNo {}
        }
    } else if hover() {
        rsx! {
            icons::FilledVoteNo { color: theme.grey00.as_str() }
        }
    } else {
        rsx! {
            icons::OutlinedVoteNo {}
        }
    };

    rsx! {
        div {
            class: "flex flex-col items-center transition-all justify-center rounded-[{rounded}px] {border_class} py-[8px] {class} hover:bg-[{bg}] cursor-pointer",
            onclick: move |evt| {
                if onclick.is_some() {
                    onclick.unwrap().call(evt)
                }
            },
            onmouseenter: move |_| hover.set(true),
            onmouseleave: move |_| hover.set(false),
            style: "color: {color}; background: {bg}; border-color: {border};",
            div { class: "flex items-center justify-center gap-[10px]",
                span { class: "text-[15px] font-bold", "반대" }
                {icon}
            }
        }
    }
}

#[component]
pub fn CloseButton(
    #[props(default ="close_button".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    onclick: EventHandler<Event<MouseData>>,
) -> Element {
    let mut hover_close = use_signal(|| false);
    let theme_service: Theme = use_context();
    let theme: crate::theme::ThemeData = theme_service.get_data();

    rsx! {
        div {
            class: format!(
                "{class} rounded-[4px] cursor-pointer {}",
                if hover_close() { "bg-[{theme.background}]" } else { "" },
            ),
            onclick,
            onmouseenter: move |_| {
                hover_close.set(true);
            },
            onmouseleave: move |_| {
                hover_close.set(false);
            },
            Close { color: if hover_close() { "{theme.primary03}" } else { "white" } }
        }
    }
}
