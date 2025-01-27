use crate::pages::*;
use dioxus::prelude::*;
use dioxus_oauth::component::OAuthPopup;
use dioxus_translate::Language;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/")]
            HomePage { lang: Language },

            #[nest("/topics")]
                #[route("/")]
                TopicsPage { lang: Language },
                #[route("/new")]
                NewTopicPage { lang: Language },
            #[end_nest]
        #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::HomePage { lang: Language::Ko })]
    #[route("/oauth/kakao")]
    OAuthPopup {},

    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
