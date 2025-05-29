use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{
    outfits::{OutfitParams, OutfitService},
    users,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOutfitRequest {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub clothing_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateOutfitRequest {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub clothing_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct OutfitListResponse {
    pub outfits: Vec<crate::models::outfits::OutfitResponse>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
}

#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let outfits = OutfitService::get_user_outfits(&ctx.db, user.id).await?;
    
    let total = outfits.len();
    let response = OutfitListResponse {
        outfits,
        total,
        page: 1,
        page_size: total,
        total_pages: 1,
    };
    
    format::json(response)
}

#[debug_handler]
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateOutfitRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    
    let outfit_params = OutfitParams {
        name: params.name,
        description: params.description,
        image_url: params.image_url,
        clothing_ids: params.clothing_ids,
    };
    
    let outfit = OutfitService::create_outfit(&ctx.db, outfit_params, user.id).await?;
    format::json(outfit)
}

#[debug_handler]
pub async fn get_one(
    auth: auth::JWT,
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let outfit = OutfitService::get_outfit_by_pid(&ctx.db, id, user.id).await?;
    
    match outfit {
        Some(outfit) => format::json(outfit),
        None => format::empty_json(),
    }
}

#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateOutfitRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    
    let outfit_params = OutfitParams {
        name: params.name,
        description: params.description,
        image_url: params.image_url,
        clothing_ids: params.clothing_ids,
    };
    
    let outfit = OutfitService::update_outfit(&ctx.db, id, outfit_params, user.id).await?;
    
    match outfit {
        Some(outfit) => format::json(outfit),
        None => format::empty_json(),
    }
}

#[debug_handler]
pub async fn remove(
    auth: auth::JWT,
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let deleted = OutfitService::delete_outfit(&ctx.db, id, user.id).await?;
    
    if deleted {
        format::empty()
    } else {
        format::empty_json()
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/outfits")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", put(update))
        .add("/{id}", delete(remove))
}