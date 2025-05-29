use crate::models::{
    clothings::{CreateClothingParams, UpdateClothingParams},
    _entities::{clothings, users},
};
use axum::{debug_handler, extract::Query};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub clothings: Vec<clothings::Model>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

/// Create a new clothing item
#[debug_handler]
async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateClothingParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let clothing = clothings::Model::create(&ctx.db, user.id, &params).await?;
    format::json(clothing)
}

/// Get a specific clothing item by ID
#[debug_handler]
async fn get_one(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let clothing = clothings::Model::find_by_id_and_user(&ctx.db, id, user.id).await?;
    format::json(clothing)
}

/// Update a clothing item
#[debug_handler]
async fn update(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(params): Json<UpdateClothingParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let clothing = clothings::Model::update_by_id(&ctx.db, id, user.id, &params).await?;
    format::json(clothing)
}

/// Delete a clothing item
#[debug_handler]
async fn delete(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    clothings::Model::delete_by_id_and_user(&ctx.db, id, user.id).await?;
    format::empty()
}

/// List clothing items with pagination
#[debug_handler]
async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<ListParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    
    let (clothings, total) = clothings::Model::list_by_user(&ctx.db, user.id, page, page_size).await?;
    
    let total_pages = (total + page_size - 1) / page_size;
    
    let response = ListResponse {
        clothings,
        total,
        page,
        page_size,
        total_pages,
    };
    
    format::json(response)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/clothings")
        .add("/", post(create))
        .add("/", get(list))
        .add("/{id}", get(get_one))
        .add("/{id}", put(update))
        .add("/{id}", axum::routing::delete(delete))
}