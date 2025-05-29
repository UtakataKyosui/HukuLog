use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use super::_entities::{outfits, outfit_clothing, clothings};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutfitParams {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub clothing_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutfitResponse {
    pub id: i32,
    pub pid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub user_id: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub clothings: Vec<ClothingInOutfit>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClothingInOutfit {
    pub id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
}

impl OutfitResponse {
    pub async fn from_model_with_clothings(
        db: &DatabaseConnection,
        outfit: outfits::Model,
    ) -> ModelResult<Self> {
        let clothings = outfit
            .find_related(clothings::Entity)
            .all(db)
            .await?;

        let clothing_responses = clothings
            .into_iter()
            .map(|clothing| ClothingInOutfit {
                id: clothing.id,
                name: clothing.name,
                image_url: clothing.image_url,
            })
            .collect();

        Ok(Self {
            id: outfit.id,
            pid: outfit.pid,
            name: outfit.name,
            description: outfit.description,
            image_url: outfit.image_url,
            user_id: outfit.user_id,
            created_at: outfit.created_at,
            updated_at: outfit.updated_at,
            clothings: clothing_responses,
        })
    }
}

impl outfits::ActiveModel {
    pub fn from_params(params: OutfitParams, user_id: i32) -> Self {
        Self {
            pid: Set(Uuid::new_v4()),
            name: Set(params.name),
            description: Set(params.description),
            image_url: Set(params.image_url),
            user_id: Set(Some(user_id)),
            ..Default::default()
        }
    }
}

pub struct OutfitService;

impl OutfitService {
    pub async fn create_outfit(
        db: &DatabaseConnection,
        params: OutfitParams,
        user_id: i32,
    ) -> ModelResult<OutfitResponse> {
        let outfit_active_model = outfits::ActiveModel::from_params(params.clone(), user_id);
        let outfit = outfit_active_model.insert(db).await?;

        // Create outfit-clothing relationships
        for clothing_id in params.clothing_ids {
            let outfit_clothing = outfit_clothing::ActiveModel {
                outfit_id: Set(outfit.pid),
                clothing_id: Set(clothing_id),
                ..Default::default()
            };
            outfit_clothing.insert(db).await?;
        }

        OutfitResponse::from_model_with_clothings(db, outfit).await
    }

    pub async fn get_user_outfits(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> ModelResult<Vec<OutfitResponse>> {
        let outfits = outfits::Entity::find()
            .filter(outfits::Column::UserId.eq(user_id))
            .all(db)
            .await?;

        let mut responses = Vec::new();
        for outfit in outfits {
            let response = OutfitResponse::from_model_with_clothings(db, outfit).await?;
            responses.push(response);
        }

        Ok(responses)
    }

    pub async fn get_outfit_by_pid(
        db: &DatabaseConnection,
        pid: Uuid,
        user_id: i32,
    ) -> ModelResult<Option<OutfitResponse>> {
        let outfit = outfits::Entity::find()
            .filter(outfits::Column::Pid.eq(pid))
            .filter(outfits::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        match outfit {
            Some(outfit) => {
                let response = OutfitResponse::from_model_with_clothings(db, outfit).await?;
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    pub async fn update_outfit(
        db: &DatabaseConnection,
        pid: Uuid,
        params: OutfitParams,
        user_id: i32,
    ) -> ModelResult<Option<OutfitResponse>> {
        let outfit = outfits::Entity::find()
            .filter(outfits::Column::Pid.eq(pid))
            .filter(outfits::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        match outfit {
            Some(outfit) => {
                let mut outfit_active_model: outfits::ActiveModel = outfit.into();
                outfit_active_model.name = Set(params.name);
                outfit_active_model.description = Set(params.description);
                outfit_active_model.image_url = Set(params.image_url);

                let updated_outfit = outfit_active_model.update(db).await?;

                // Update outfit-clothing relationships
                // First, delete existing relationships
                outfit_clothing::Entity::delete_many()
                    .filter(outfit_clothing::Column::OutfitId.eq(updated_outfit.pid))
                    .exec(db)
                    .await?;

                // Then, create new relationships
                for clothing_id in params.clothing_ids {
                    let outfit_clothing = outfit_clothing::ActiveModel {
                        outfit_id: Set(updated_outfit.pid),
                        clothing_id: Set(clothing_id),
                        ..Default::default()
                    };
                    outfit_clothing.insert(db).await?;
                }

                let response = OutfitResponse::from_model_with_clothings(db, updated_outfit).await?;
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    pub async fn delete_outfit(
        db: &DatabaseConnection,
        pid: Uuid,
        user_id: i32,
    ) -> ModelResult<bool> {
        let outfit = outfits::Entity::find()
            .filter(outfits::Column::Pid.eq(pid))
            .filter(outfits::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        match outfit {
            Some(outfit) => {
                // Delete outfit-clothing relationships first (due to foreign key constraints)
                outfit_clothing::Entity::delete_many()
                    .filter(outfit_clothing::Column::OutfitId.eq(outfit.pid))
                    .exec(db)
                    .await?;

                // Delete the outfit
                outfit.delete(db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}