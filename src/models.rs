use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[repr(C)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub owner: String,
    pub events: Vec<String>,
    pub main_file: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Runners {
    pub name: String,
    pub id: i64,
    #[serde(skip_serializing)]
    pub token: String,
    pub remote_host: Option<std::net::IpAddr>,
    pub remote_user: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Credentials {
    pub id: i64,
    pub passphrase: Option<String>,
    #[serde(skip_serializing)]
    pub private_key: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Repos {
    pub id: i64,
    pub gh_id: i64,
    pub install: Option<i64>,
    pub owner: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct JobLog {
    pub id: i64,
    pub job: i64,
    pub step: String,
    pub status: i64,
    pub output: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
/// From @ Github
pub struct AccessToken {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Installations {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Job {
    pub id: i64,
    pub assigned_runner: String,
    pub status: i64,
    pub repo: i64,
    pub triggered_by: String,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(rename_all = "snake_case")]
pub enum PipelineBackend {
    Docker,
    LXC,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Pipelines {
    pub id: i64,
    pub owned_by: i64,
    pub name: String,
    pub backend: Option<PipelineBackend>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct PipelineStep {
    pub id: i64,
    pub belongs_to: i64,
    pub name: String,
    pub run: String,
}
