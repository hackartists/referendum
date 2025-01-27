#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Kakao(
    #[props(default ="kakao".to_string())] id: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { id, ..attributes,
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                height: "33",
                "viewBox": "0 0 36 33",
                width: "36",
                fill: "none",
                path {
                    fill: "#3E2723",
                    d: "M18 0.351074C8.33495 0.351074 0.5 6.52874 0.5 14.1492C0.5 19.0759 3.77553 23.3989 8.70279 25.84C8.43474 26.7645 6.98022 31.7873 6.92233 32.1819C6.92233 32.1819 6.8875 32.4784 7.0795 32.5915C7.27149 32.7045 7.49731 32.6167 7.49731 32.6167C8.04788 32.5398 13.8819 28.4418 14.8917 27.7302C15.9005 27.873 16.9392 27.9472 18 27.9472C27.665 27.9472 35.5 21.7697 35.5 14.1492C35.5 6.52874 27.665 0.351074 18 0.351074Z",
                }
            }
        }
    }
}
