#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq)]
pub enum SizeBase {
    Width,
    #[default]
    Height,
}

#[component]
pub fn OutlinedHandshakeIcon(
    #[props(default = 48.0)] size: f64,
    #[props(default = SizeBase::default())] size_base: SizeBase,
) -> Element {
    let (base_width, base_height) = (48.0, 48.0);

    let ratio = match size_base {
        SizeBase::Width => size / base_width,
        SizeBase::Height => size / base_height,
    };

    let width = base_width * ratio;
    let height = base_height * ratio;
    let stroke_width = 4.0 * ratio;

    tracing::debug!(
        "width: {}, height: {} stroke: {}",
        width,
        height,
        stroke_width
    );

    let color: ColorTheme = use_context();

    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            "viewBox": "0 0 48 48",
            xmlns: "http://www.w3.org/2000/svg",
            g {
                fill: "none",
                "stroke-linecap": "round",
                stroke: "{color.icon.primary}",
                "stroke-linejoin": "round",
                "stroke-width": "{stroke_width}",
                path { d: "m24 40l12-12l-4 4l-4 4zm0 0L4 20L16 8l8 8" }
                path { d: "M17 23L32 8l12 12l-8 8l-8-8l-6 6zm0 0l7-7m4 20l-3-3m7-1l-3-3" }
            }
        }
    }
}

#[component]
pub fn HandshakeIcon(
    #[props(default = 147.0)] size: f64,
    #[props(default = SizeBase::default())] size_base: SizeBase,
) -> Element {
    let (base_width, base_height) = (147.0, 117.0);

    let (width, height) = match size_base {
        SizeBase::Width => {
            let ratio = size / base_width;
            tracing::debug!("ratio: {}", ratio);
            (size, base_height * ratio)
        }
        SizeBase::Height => {
            let ratio = size / base_height;
            tracing::debug!("ratio: {}", ratio);
            (base_width * ratio, size)
        }
    };

    tracing::debug!("width: {}, height: {}", width, height);

    let color: ColorTheme = use_context();

    rsx! {

        svg {
            "viewBox": "0 0 147 117",
            fill: "none",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            height: "{height}",
            path {
                d: "M126 57.125V57.8984L142.336 41.5625C147.469 36.4297 147.469 28.1328 142.336 23L124.031 4.71875C118.898 -0.414063 110.602 -0.414063 105.469 4.71875L96.0703 14.1172C95.4375 14.0469 94.7812 14 94.125 14H67.875C59.1797 14 52.0312 20.5625 51.0938 29H51V57.125C51 62.3047 55.1953 66.5 60.375 66.5C65.5547 66.5 69.75 62.3047 69.75 57.125V36.5H107.25C117.609 36.5 126 44.8906 126 55.25V57.125ZM77.25 44V57.125C77.25 66.4531 69.7031 74 60.375 74C51.0469 74 43.5 66.4531 43.5 57.125V29.3281C35.0859 30.7812 28.0781 36.8984 25.6875 45.3125L21.8203 58.8125L4.66406 75.9688C-0.46875 81.1016 -0.46875 89.3984 4.66406 94.5312L22.9688 112.836C28.1016 117.969 36.3984 117.969 41.5312 112.836L50.3672 104C50.5781 104 50.7891 104.023 51 104.023H88.5C94.7109 104.023 99.75 98.9844 99.75 92.7734C99.75 91.4609 99.5156 90.1953 99.1172 89.0234H99.75C105.961 89.0234 111 83.9844 111 77.7734C111 74.7734 109.828 72.0547 107.906 70.0391C113.93 68.8672 118.477 63.5703 118.5 57.1953V57.1016C118.477 49.8828 112.617 44.0234 105.375 44.0234H77.25V44Z",
                fill: "{color.icon.primary}",
            }
        }
    }
}
