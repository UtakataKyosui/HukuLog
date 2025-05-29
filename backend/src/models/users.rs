use async_trait::async_trait;
use chrono::offset::Local;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use uuid::Uuid;
use webauthn_rs::prelude::*;

pub use super::_entities::users::{self, ActiveModel, Entity, Model};



#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterParams {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(custom(function = "validation::is_valid_email"))]
    pub email: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            name: self.name.as_ref().to_owned(),
            email: self.email.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::users::ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        if insert {
            let mut this = self;
            this.pid = ActiveValue::Set(Uuid::new_v4());
            this.api_key = ActiveValue::Set(format!("lo-{}", Uuid::new_v4()));
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

#[async_trait]
impl Authenticable for Model {
    async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
        Self::find_by_pid(db, claims_key).await
    }
}

impl Model {
    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, email)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided verification token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::EmailVerificationToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }



    /// finds a user by the provided reset token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_reset_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ResetToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Pid, parse_uuid)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided api key
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    ///
    /// # Errors
    ///
    /// when could not verify password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Asynchronously creates a user with a password and saves it to the
    /// database.
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    pub async fn create_with_password(
        db: &DatabaseConnection,
        params: &RegisterParams,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        if users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, &params.email)
                    .build(),
            )
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists {});
        }

        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        let user = users::ActiveModel {
            email: ActiveValue::set(params.email.to_string()),
            password: ActiveValue::set(password_hash),
            name: ActiveValue::set(params.name.to_string()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(user)
    }

    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_jwt(&self, secret: &str, expiration: u64) -> ModelResult<String> {
        Ok(jwt::JWT::new(secret).generate_token(expiration, self.pid.to_string(), Map::new())?)
    }
}

impl ActiveModel {
    /// Sets the email verification information for the user and
    /// updates it in the database.
    ///
    /// This method is used to record the timestamp when the email verification
    /// was sent and generate a unique verification token for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        self.email_verification_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.email_verification_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Sets the information for a reset password request,
    /// generates a unique reset password token, and updates it in the
    /// database.
    ///
    /// This method records the timestamp when the reset password token is sent
    /// and generates a unique token for the user.
    ///
    /// # Arguments
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_forgot_password_sent(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.reset_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.reset_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Records the verification time when a user verifies their
    /// email and updates it in the database.
    ///
    /// This method sets the timestamp when the user successfully verifies their
    /// email.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn verified(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.email_verified_at = ActiveValue::set(Some(Local::now().into()));
        Ok(self.update(db).await?)
    }

    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(
        mut self,
        db: &DatabaseConnection,
        password: &str,
    ) -> ModelResult<Model> {
        self.password =
            ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        self.reset_token = ActiveValue::Set(None);
        self.reset_sent_at = ActiveValue::Set(None);
        Ok(self.update(db).await?)
    }



    /// Store passkey credentials
    ///
    /// # Errors
    /// - Returns an error if database update fails or serialization fails
    pub async fn store_passkey_credentials(
        mut self,
        db: &DatabaseConnection,
        credentials: &[Passkey],
    ) -> ModelResult<Model> {
        let creds_json = serde_json::to_value(credentials)
            .map_err(|e| ModelError::Any(e.into()))?;
        
        self.passkey_credentials = ActiveValue::set(Some(creds_json));
        Ok(self.update(db).await?)
    }

    /// Set passkey challenge
    ///
    /// # Errors
    /// - Returns an error if database update fails
    pub async fn set_passkey_challenge(
        mut self,
        db: &DatabaseConnection,
        challenge: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> ModelResult<Model> {
        self.passkey_challenge = ActiveValue::set(Some(challenge.to_string()));
        self.passkey_challenge_expiration = ActiveValue::set(Some(expiration.into()));
        Ok(self.update(db).await?)
    }

    /// Clear passkey challenge
    ///
    /// # Errors
    /// - Returns an error if database update fails
    pub async fn clear_passkey_challenge(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.passkey_challenge = ActiveValue::set(None);
        self.passkey_challenge_expiration = ActiveValue::set(None);
        Ok(self.update(db).await?)
    }
}

impl Model {
    /// Get stored passkey credentials
    ///
    /// # Errors
    /// - Returns an error if credentials cannot be deserialized
    pub fn get_passkey_credentials(&self) -> ModelResult<Vec<Passkey>> {
        match &self.passkey_credentials {
            Some(creds_json) => {
                let creds: Vec<Passkey> = serde_json::from_value(creds_json.clone())
                    .map_err(|e| ModelError::Any(e.into()))?;
                Ok(creds)
            }
            None => Ok(vec![]),
        }
    }

    /// Verify passkey challenge
    #[must_use]
    pub fn verify_passkey_challenge(&self, challenge: &str) -> bool {
        if let (Some(stored_challenge), Some(expiration)) = 
            (&self.passkey_challenge, &self.passkey_challenge_expiration) {
            let now = chrono::Utc::now().fixed_offset();
            return stored_challenge == challenge && expiration > &now;
        }
        false
    }

    /// Convert to ActiveModel for updating
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::unchanged(self.id),
            pid: ActiveValue::unchanged(self.pid),
            email: ActiveValue::unchanged(self.email),
            password: ActiveValue::unchanged(self.password),
            api_key: ActiveValue::unchanged(self.api_key),
            name: ActiveValue::unchanged(self.name),
            reset_token: ActiveValue::unchanged(self.reset_token),
            reset_sent_at: ActiveValue::unchanged(self.reset_sent_at),
            email_verification_token: ActiveValue::unchanged(self.email_verification_token),
            email_verification_sent_at: ActiveValue::unchanged(self.email_verification_sent_at),
            email_verified_at: ActiveValue::unchanged(self.email_verified_at),
            passkey_credentials: ActiveValue::unchanged(self.passkey_credentials),
            passkey_challenge: ActiveValue::unchanged(self.passkey_challenge),
            passkey_challenge_expiration: ActiveValue::unchanged(self.passkey_challenge_expiration),
            created_at: ActiveValue::unchanged(self.created_at),
            updated_at: ActiveValue::unchanged(self.updated_at),
        }
    }
}
