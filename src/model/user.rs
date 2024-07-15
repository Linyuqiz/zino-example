use serde::{Deserialize, Serialize};
use zino::prelude::*;
use zino_derive::{DecodeRow, Model, ModelAccessor, ModelHooks, Schema};

#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    DecodeRow,
    Schema,
    ModelAccessor,
    ModelHooks,
    Model,
)]
#[serde(default)]
pub struct User {
    #[schema(primary_key)]
    id: Uuid,
    #[schema(not_null)]
    name: String,
    #[schema(not_null)]
    description: String,
    #[schema(default_value = "now")]
    create_time: DateTime,
    #[schema(default_value = "now")]
    update_time: DateTime,
}
