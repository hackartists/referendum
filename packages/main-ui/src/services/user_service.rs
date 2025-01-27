#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::*;

use crate::config;

pub enum UserEvent {
    Signup,
    Login,
    Logout,
}

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    pub cli: Signal<UserClient>,
    pub email: Signal<String>,
    pub nickname: Signal<String>,
    pub profile_url: Signal<String>,
    pub role: Signal<UserRole>,
    pub id_token: Signal<Option<String>>,
}

impl UserService {
    pub fn init() {
        let conf = config::get();
        let cli = User::get_client(&conf.main_api_endpoint);
        let srv = Self {
            cli: Signal::new(cli),
            email: Signal::new("".to_string()),
            nickname: Signal::new("".to_string()),
            profile_url: Signal::new("".to_string()),
            role: Signal::new(UserRole::Guest),
            id_token: Signal::new(None),
        };
        rest_api::add_hook(srv);
        use_context_provider(|| srv);
    }

    pub fn role(&self) -> UserRole {
        (self.role)()
    }

    pub fn logout(&mut self) {
        self.email.set("".to_string());
        self.nickname.set("".to_string());
        self.profile_url.set("".to_string());
        self.role.set(UserRole::Guest);
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

    pub async fn login(&mut self, id_token: String, kakao_id: String) -> UserEvent {
        tracing::debug!("UserService::login");
        self.id_token.set(Some(id_token));

        let cli = (self.cli)();
        match cli.login(kakao_id).await {
            Ok(user) => {
                self.email.set(user.email);
                self.nickname.set(user.nickname);
                self.profile_url.set(user.profile_url);
                self.role.set(user.role);
                UserEvent::Login
            }
            Err(_) => UserEvent::Signup,
        }
    }

    pub async fn signup(&mut self, email: &str, nickname: &str, profile_url: &str) -> Result<()> {
        tracing::debug!(
            "UserService::signup: email={} nickname={} profile_url={}",
            email,
            nickname,
            profile_url
        );

        let cli = (self.cli)();

        match cli
            .signup(
                nickname.to_string(),
                email.to_string(),
                profile_url.to_string(),
            )
            .await
        {
            Ok(v) => {
                self.email.set(v.email);
                self.nickname.set(v.nickname);
                self.profile_url.set(v.profile_url);
                self.role.set(v.role);
            }
            Err(e) => {
                tracing::error!("UserService::signup: error={:?}", e);
                self.id_token.set(None);
                Err(e)?;
            }
        };

        Ok(())
    }
}

impl rest_api::RequestHooker for UserService {
    fn before_request(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let id_token = (self.id_token)();

        if let Some(id_token) = id_token {
            req.header("x-id-token".to_string(), id_token)
        } else {
            req
        }
    }
}
