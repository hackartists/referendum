#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;

#[component]
pub fn Labeling(
    title: String,
    #[props(default = false)] required: bool,
    children: Element,
) -> Element {
    rsx! {
        div { class: "w-full flex flex-col gap-[5px] items-start justify-start",
            div { class: "w-full flex flex-row items-center justify-start gap-[2px]",
                div { class: "text-[16px] font-bold", "{title}" }
                if required {
                    div { class: "text-[#FF0000] text-[16px] font-bold", "*" }
                }
            }

            {children}
        }
    }
}

#[component]
pub fn LabeledInput(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = "text".to_string())] r#type: String,
    title: String,
    value: Option<String>,
    #[props(default = false)] required: bool,
    children: Element,
    oninput: Option<EventHandler<String>>,
    #[props(default = false)] disabled: bool,
    placeholder: Option<String>,
) -> Element {
    let color = use_context::<ColorTheme>();

    rsx! {
        Labeling { title, required,
            input {
                r#type,
                class: "w-full rounded-[5px] py-[10px] px-[20px] text-[16px]",
                background: color.input.primary,
                color: if disabled { color.text.disabled } else { color.text.primary },
                value,
                disabled,
                placeholder,
                oninput: move |e| {
                    if let Some(oninput) = oninput {
                        oninput(e.value());
                    }
                },
                ..attributes,
            }
        }
    }
}
