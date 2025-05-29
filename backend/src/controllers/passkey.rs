use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{
    models::users,
    webauthn::create_webauthn,
};

// In-memory storage for WebAuthn states (in production, use Redis)
lazy_static::lazy_static! {
    static ref REG_STATE_STORE: Arc<Mutex<HashMap<String, PasskeyRegistration>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    static ref AUTH_STATE_STORE: Arc<Mutex<HashMap<String, PasskeyAuthentication>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasskeyRegistrationStartRequest {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasskeyRegistrationFinishRequest {
    pub email: String,
    pub credential: RegisterPublicKeyCredential,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasskeyAuthenticationStartRequest {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasskeyAuthenticationFinishRequest {
    pub email: String,
    pub credential: PublicKeyCredential,
}

/// Start passkey registration process
#[debug_handler]
pub async fn registration_start(
    State(ctx): State<AppContext>,
    Json(params): Json<PasskeyRegistrationStartRequest>,
) -> Result<Json<CreationChallengeResponse>> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;
    
    let webauthn = create_webauthn(&ctx)?;
    
    let user_unique_id = Uuid::parse_str(&user.pid.to_string())
        .map_err(|e| Error::BadRequest(format!("Invalid UUID: {}", e)))?;
    let existing_creds = user.get_passkey_credentials()?;
    
    // Convert passkeys to credential IDs for exclusion
    let exclude_credentials = if existing_creds.is_empty() {
        None
    } else {
        Some(existing_creds.iter().map(|pk| pk.cred_id().clone()).collect())
    };
    
    let (ccr, reg_state) = webauthn
        .start_passkey_registration(
            user_unique_id,
            &params.email,
            &user.name,
            exclude_credentials,
        )
        .map_err(|e| Error::BadRequest(format!("WebAuthn registration start error: {}", e)))?;

    // Store the registration state in memory (in production, use Redis)
    let challenge_id = format!("reg_{}_{}", user.id, chrono::Utc::now().timestamp());
    let expiration = chrono::Utc::now() + chrono::Duration::minutes(5);
    
    {
        let mut store = REG_STATE_STORE.lock().unwrap();
        store.insert(challenge_id.clone(), reg_state);
    }
    
    // Store a reference to the challenge in the database
    let active_user = user.into_active_model();
    active_user.set_passkey_challenge(&ctx.db, &challenge_id, expiration).await?;

    Ok(Json(ccr))
}

/// Finish passkey registration process
#[debug_handler]
pub async fn registration_finish(
    State(ctx): State<AppContext>,
    Json(params): Json<PasskeyRegistrationFinishRequest>,
) -> Result<Json<serde_json::Value>> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;
    
    let webauthn = create_webauthn(&ctx)?;
    
    // Retrieve and verify the registration state
    let challenge_id = user.passkey_challenge
        .as_ref()
        .ok_or_else(|| Error::BadRequest("No registration in progress".to_string()))?;
    
    if !user.verify_passkey_challenge(challenge_id) {
        return Err(Error::BadRequest("Registration challenge expired or invalid".to_string()));
    }
    
    // Get the registration state from memory
    let reg_state = {
        let mut store = REG_STATE_STORE.lock().unwrap();
        store.remove(challenge_id)
            .ok_or_else(|| Error::BadRequest("Registration state not found".to_string()))?
    };
    
    let sk = webauthn
        .finish_passkey_registration(&params.credential, &reg_state)
        .map_err(|e| Error::BadRequest(format!("WebAuthn registration finish error: {}", e)))?;
    
    // Store the new credential
    let mut existing_creds = user.get_passkey_credentials()?;
    existing_creds.push(sk);
    
    let active_user = user.into_active_model();
    let updated_user = active_user.store_passkey_credentials(&ctx.db, &existing_creds).await?;
    let active_updated_user = updated_user.into_active_model();
    let _ = active_updated_user.clear_passkey_challenge(&ctx.db).await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Passkey registered successfully"
    })))
}

/// Start passkey authentication process
#[debug_handler]
pub async fn authentication_start(
    State(ctx): State<AppContext>,
    Json(params): Json<PasskeyAuthenticationStartRequest>,
) -> Result<Json<RequestChallengeResponse>> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;
    
    let webauthn = create_webauthn(&ctx)?;
    
    let existing_creds = user.get_passkey_credentials()?;
    
    if existing_creds.is_empty() {
        return Err(Error::BadRequest("No passkeys registered for this user".to_string()));
    }
    
    let (rcr, auth_state) = webauthn
        .start_passkey_authentication(&existing_creds)
        .map_err(|e| Error::BadRequest(format!("WebAuthn authentication start error: {}", e)))?;

    // Store the authentication state in memory
    let challenge_id = format!("auth_{}_{}", user.id, chrono::Utc::now().timestamp());
    let expiration = chrono::Utc::now() + chrono::Duration::minutes(5);
    
    {
        let mut store = AUTH_STATE_STORE.lock().unwrap();
        store.insert(challenge_id.clone(), auth_state);
    }
    
    let active_user = user.into_active_model();
    active_user.set_passkey_challenge(&ctx.db, &challenge_id, expiration).await?;

    Ok(Json(rcr))
}

/// Finish passkey authentication process
#[debug_handler]
pub async fn authentication_finish(
    State(ctx): State<AppContext>,
    Json(params): Json<PasskeyAuthenticationFinishRequest>,
) -> Result<Json<serde_json::Value>> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;
    
    let webauthn = create_webauthn(&ctx)?;
    
    // Retrieve and verify the authentication state
    let challenge_id = user.passkey_challenge
        .as_ref()
        .ok_or_else(|| Error::BadRequest("No authentication in progress".to_string()))?;
    
    if !user.verify_passkey_challenge(challenge_id) {
        return Err(Error::BadRequest("Authentication challenge expired or invalid".to_string()));
    }
    
    // Get the authentication state from memory
    let auth_state = {
        let mut store = AUTH_STATE_STORE.lock().unwrap();
        store.remove(challenge_id)
            .ok_or_else(|| Error::BadRequest("Authentication state not found".to_string()))?
    };
    
    let _auth_result = webauthn
        .finish_passkey_authentication(&params.credential, &auth_state)
        .map_err(|e| Error::BadRequest(format!("WebAuthn authentication finish error: {}", e)))?;

    // Generate JWT token for successful authentication
    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = user.generate_jwt(&jwt_secret.secret, jwt_secret.expiration)?;

    // Clear the challenge
    let active_user = user.into_active_model();
    let updated_user = active_user.clear_passkey_challenge(&ctx.db).await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Authentication successful",
        "token": token,
        "user": {
            "id": updated_user.id,
            "email": updated_user.email,
            "name": updated_user.name,
            "verified": updated_user.email_verified_at.is_some()
        }
    })))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("passkey")
        .add("/registration/start", post(registration_start))
        .add("/registration/finish", post(registration_finish))
        .add("/authentication/start", post(authentication_start))
        .add("/authentication/finish", post(authentication_finish))
}