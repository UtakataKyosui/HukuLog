use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect, PaginatorTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub use super::_entities::clothings::{self, ActiveModel, Entity, Model};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateClothingParams {
    pub name: String,
    pub purchased_at: Option<chrono::NaiveDate>,
    pub purchase_price: Option<Decimal>,
    pub image_url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClothingParams {
    pub name: Option<String>,
    pub purchased_at: Option<chrono::NaiveDate>,
    pub purchase_price: Option<Decimal>,
    pub image_url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 1, message = "Name must not be empty."))]
    pub name: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            name: self.name.as_ref().to_owned(),
        })
    }
}

impl Model {
    /// Creates a new clothing item
    ///
    /// # Errors
    ///
    /// When could not save the clothing into the DB
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        params: &CreateClothingParams,
    ) -> ModelResult<Self> {
        let clothing = clothings::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            name: ActiveValue::Set(params.name.clone()),
            purchased_at: ActiveValue::Set(params.purchased_at),
            purchase_price: ActiveValue::Set(params.purchase_price),
            image_url: ActiveValue::Set(params.image_url.clone()),
            notes: ActiveValue::Set(params.notes.clone()),
            user_id: ActiveValue::Set(Some(user_id)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(clothing)
    }

    /// Updates an existing clothing item
    ///
    /// # Errors
    ///
    /// When could not update the clothing in the DB
    pub async fn update_by_id(
        db: &DatabaseConnection,
        id: Uuid,
        user_id: i32,
        params: &UpdateClothingParams,
    ) -> ModelResult<Self> {
        let clothing = clothings::Entity::find_by_id(id)
            .filter(clothings::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        let mut active_model: clothings::ActiveModel = clothing.into();

        if let Some(name) = &params.name {
            active_model.name = ActiveValue::Set(name.clone());
        }
        if let Some(purchased_at) = params.purchased_at {
            active_model.purchased_at = ActiveValue::Set(Some(purchased_at));
        }
        if let Some(purchase_price) = params.purchase_price {
            active_model.purchase_price = ActiveValue::Set(Some(purchase_price));
        }
        if let Some(image_url) = &params.image_url {
            active_model.image_url = ActiveValue::Set(Some(image_url.clone()));
        }
        if let Some(notes) = &params.notes {
            active_model.notes = ActiveValue::Set(Some(notes.clone()));
        }
        


        Ok(active_model.update(db).await?)
    }

    /// Finds a clothing item by ID and user ID
    ///
    /// # Errors
    ///
    /// When could not find clothing or DB query error
    pub async fn find_by_id_and_user(
        db: &DatabaseConnection,
        id: Uuid,
        user_id: i32,
    ) -> ModelResult<Self> {
        let clothing = clothings::Entity::find_by_id(id)
            .filter(clothings::Column::UserId.eq(user_id))
            .one(db)
            .await?;
        clothing.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Lists all clothing items for a user with pagination
    ///
    /// # Errors
    ///
    /// When DB query error occurs
    pub async fn list_by_user(
        db: &DatabaseConnection,
        user_id: i32,
        page: u64,
        page_size: u64,
    ) -> ModelResult<(Vec<Self>, u64)> {
        let offset = (page - 1) * page_size;
        
        let clothings = clothings::Entity::find()
            .filter(clothings::Column::UserId.eq(user_id))
            .order_by_desc(clothings::Column::CreatedAt)
            .offset(offset)
            .limit(page_size)
            .all(db)
            .await?;

        let total = clothings::Entity::find()
            .filter(clothings::Column::UserId.eq(user_id))
            .count(db)
            .await?;

        Ok((clothings, total))
    }

    /// Deletes a clothing item by ID and user ID
    ///
    /// # Errors
    ///
    /// When could not delete clothing or DB query error
    pub async fn delete_by_id_and_user(
        db: &DatabaseConnection,
        id: Uuid,
        user_id: i32,
    ) -> ModelResult<()> {
        let result = clothings::Entity::delete_many()
            .filter(clothings::Column::Id.eq(id))
            .filter(clothings::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        if result.rows_affected == 0 {
            return Err(ModelError::EntityNotFound);
        }

        Ok(())
    }
}