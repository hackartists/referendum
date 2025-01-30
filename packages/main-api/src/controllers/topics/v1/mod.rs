pub mod votes;

use std::time::SystemTime;

use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::Role;
use dto::*;
use votes::VoteControllerV1;

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    repo: TopicRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl TopicControllerV1 {
    pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Topic::get_repository(pool.clone());

        repo.create_table().await?;

        let ctrl = TopicControllerV1 {
            repo,
            pool: pool.clone(),
        };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_topic).post(Self::act_topic_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_topic).get(Self::list_topic))
            .with_state(ctrl.clone())
            .nest("/:id/votes", VoteControllerV1::route(pool.clone()).await?))
    }

    pub async fn act_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(authz): Extension<Option<Authorization>>,
        Json(body): Json<TopicAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("list_topic {:?}", body);
        match authz {
            Some(Authorization::Bearer { claims }) if claims.role == Role::Admin => {}
            _ => return Err(ServiceError::Unauthorized),
        }

        match body {
            TopicAction::Create(TopicCreateRequest {
                title,
                content,
                started_at,
                ended_at,
                requirement,
            }) => {
                let topic = ctrl
                    .repo
                    .insert(title, content, started_at, ended_at, requirement)
                    .await?;
                Ok(Json(topic))
            }
        }
    }

    pub async fn act_topic_by_id(
        State(_ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Authorization>>,
        Path(id): Path<String>,
        Json(body): Json<TopicByIdAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("list_topic {:?} {:?}", id, body);
        Ok(Json(Topic::default()))
    }

    pub async fn get_topic(
        State(_ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("get_topic {:?}", id);
        Ok(Json(Topic::default()))
    }

    pub async fn list_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_authz): Extension<Option<Authorization>>,
        Query(param): Query<TopicParam>,
    ) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("list_topic {:?}", param);

        match param {
            TopicParam::Query(q) => {
                let topics = ctrl.repo.find(&q).await?;
                Ok(Json(TopicGetResponse::Query(topics)))
            }
            TopicParam::Read(action) => ctrl.read_action(action).await,
        }
    }
}

impl TopicControllerV1 {
    async fn read_action(&self, action: TopicReadAction) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("read_action {:?}", action);

        self.get_topic_for_voting().await
    }

    async fn get_topic_for_voting(&self) -> Result<Json<TopicGetResponse>> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let topic: Topic = sqlx::query("SELECT * FROM topics WHERE started_at > $1")
            .bind(now)
            .map(|row: sqlx::postgres::PgRow| row.into())
            .fetch_one(&self.pool)
            .await?;

        tracing::debug!("get_topic_for_voting {:?}", topic);

        Ok(Json(TopicGetResponse::Read(topic)))
    }
}
