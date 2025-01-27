use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::{Topic, TopicClient};

use crate::route::Route;

#[derive(Clone, Copy)]
pub struct Controller {
    pub title: Signal<String>,
    pub start: Signal<i64>,
    pub end: Signal<i64>,
    pub requirement: Signal<i64>,
    pub content: Signal<String>,
    pub nav: Navigator,
    pub lang: Language,
    pub cli: Signal<TopicClient>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            title: Signal::new("".to_string()),
            start: Signal::new(0),
            end: Signal::new(0),
            requirement: Signal::new(0),
            content: Signal::new("".to_string()),
            nav: use_navigator(),
            lang,
            cli: Signal::new(Topic::get_client(&crate::config::get().main_api_endpoint)),
        };

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn handle_requirement(&mut self, requirement: String) {
        tracing::debug!("handle_requirement: {:?}", requirement);
        if let Ok(requirement) = requirement.parse::<i64>() {
            self.requirement.set(requirement);
        }
    }

    pub fn handle_start(&mut self, date: String) {
        tracing::debug!("handle_start: {:?}", date);
        if let Some(epoch) = self.date_to_epoch(date) {
            self.start.set(epoch);
        }
    }

    pub fn handle_end(&mut self, date: String) {
        tracing::debug!("handle_end: {:?}", date);
        if let Some(epoch) = self.date_to_epoch(date) {
            self.end.set(epoch);
        }
    }

    pub fn date_to_epoch(&mut self, date: String) -> Option<i64> {
        if date.is_empty() {
            return None;
        }

        let date = date.split('-').collect::<Vec<&str>>();
        let year = date[0].parse::<i32>().ok()?;
        let month = date[1].parse::<u32>().ok()?;
        let day = date[2].parse::<u32>().ok()?;

        let datetime = chrono::NaiveDate::from_ymd_opt(year, month, day)?.and_hms_opt(0, 0, 0)?;
        Some(datetime.and_utc().timestamp())
    }

    pub fn cancel(&mut self) {
        if self.nav.can_go_back() {
            self.nav.go_back();
        } else {
            self.nav.replace(Route::TopicsPage { lang: self.lang });
        }
    }

    pub async fn submit(&mut self) {
        let cli = (self.cli)();
        let title = (self.title)();
        let content = (self.content)();
        let started_at = (self.start)();
        let ended_at = (self.end)();
        let requirement = (self.requirement)();

        match cli
            .create(title, content, started_at, ended_at, requirement)
            .await
        {
            Ok(topic) => {
                tracing::debug!("Created topic: {:?}", topic);
                self.nav.replace(Route::TopicsPage { lang: self.lang });
            }
            Err(e) => {
                tracing::error!("Failed to create topic: {:?}", e);
            }
        }
    }
}
