use std::any::Any;

use chrono::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    models::{Job, PipelineStep, Pipelines},
    websocket::WebsocketEvent,
};

macro_rules! impl_websocket_event {
    ($name:ident) => {
        #[typetag::serde]
        impl WebsocketEvent for $name {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hello {
    pub heartbeat: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identify {
    pub name: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateJobRun {
    pub job: Job,
    pub pipeline: Pipelines,
    pub steps: Vec<PipelineStep>,
}
impl_websocket_event!(Hello);
impl_websocket_event!(Identify);
impl_websocket_event!(CreateJobRun);
