use dioxus_aws::prelude::*;
use dto::*;

use crate::config;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub topic: Resource<Topic>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let conf = config::get();

        let ctrl = Self {
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
