use dioxus_aws::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;
use dto::*;

use crate::{config, pages::voting_popup::VotingPopup};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub topic: Resource<Topic>,
    pub popup: PopupService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let conf = config::get();
        let mut popup: PopupService = use_context();

        use_effect(move || {
            tracing::debug!("Controller effect");
            popup
                .open(rsx! {
                    VotingPopup { lang, topic_id: "1", topic_title: "title" }
                })
                .with_title("");
        });

        let ctrl = Self {
            popup,
            topic: use_server_future(move || async move {
                let cli = Topic::get_client(&conf.main_api_endpoint);
                match cli.get_topic().await {
                    Ok(topic) => {
                        tracing::debug!("Topic: {:?}", topic);
                        topic
                    }
                    Err(err) => {
                        tracing::error!("Error: {:?}", err);
                        Topic::default()
                    }
                }
            })?,
        };
        use_context_provider(|| ctrl);
        tracing::debug!("Controller initialized");

        Ok(ctrl)
    }

    pub fn topic(&self) -> Option<Topic> {
        self.topic.with(|topic| topic.clone())
    }
}
