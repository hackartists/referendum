#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::*;

use crate::config;

pub enum UserEvent {
    Signup(String, String, String, String),
    Login,
    Logout,
}

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    pub cli: Signal<UserClient>,
    pub email: Signal<String>,
    pub nickname: Signal<String>,
    pub profile_url: Signal<String>,
    pub principal: Signal<String>,
}

impl UserService {
    pub fn init() {
        let conf = config::get();
        let cli = User::get_client(&conf.main_api_endpoint);

        use_context_provider(|| Self {
            cli: Signal::new(cli),
            email: Signal::new("".to_string()),
            nickname: Signal::new("".to_string()),
            profile_url: Signal::new("".to_string()),
            principal: Signal::new("".to_string()),
        });
    }

    pub fn logout(&mut self) {
        self.email.set("".to_string());
        self.nickname.set("".to_string());
        self.profile_url.set("".to_string());
        self.principal.set("".to_string());
    }

    pub async fn get_user_info_from_server(&mut self) {
        let cli = (self.cli)();

        let user: User = match cli.user_info().await {
            Ok(v) => v,
            Err(e) => match e {
                ServiceError::NotFound => {
                    return;
                }
                e => {
                    tracing::error!("UserService::get_user_info_from_server: error={:?}", e);
                    return;
                }
            },
        };

        self.nickname.set(user.nickname);
        self.profile_url.set(user.profile_url);
    }

    pub fn get_user_info(&self) -> Option<(String, String)> {
        let nickname = (self.nickname)();
        let profile_url = (self.profile_url)();

        if profile_url.is_empty() || nickname.is_empty() {
            return None;
        }

        Some((nickname, profile_url))
    }

    pub async fn login(&mut self) -> UserEvent {
        tracing::debug!("UserService::login");

        UserEvent::Logout
    }

    pub async fn login_or_signup(
        &self,
        principal: &str,
        email: &str,
        nickname: &str,
        profile_url: &str,
    ) -> Result<()> {
        tracing::debug!(
            "UserService::signup: principal={} email={} nickname={} profile_url={}",
            principal,
            email,
            nickname,
            profile_url
        );

        let cli = (self.cli)();

        let res: User = match cli
            .signup(
                nickname.to_string(),
                email.to_string(),
                profile_url.to_string(),
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("UserService::signup: error={:?}", e);
                rest_api::remove_signer();
                return Err(e);
            }
        };

        tracing::debug!("UserService::signup: user={:?}", res);
        Ok(())
    }
}
