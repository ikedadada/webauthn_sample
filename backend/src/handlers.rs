use actix_web::{web, HttpResponse, Result, http::header};
use serde_json::{json, Value};
use sqlx::MySqlPool;
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::{RegisterRequest, LoginRequest};
use crate::db;

// Simple in-memory storage for demo purposes
lazy_static::lazy_static! {
    static ref CHALLENGES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType::html())
        .body("WebAuthn Backend API"))
}

pub async fn register_request(
    pool: web::Data<MySqlPool>,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    // Check if user already exists
    let existing_user = db::get_user_by_username(&pool, &form.username).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
    
    if existing_user.is_some() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "User already exists"
        })));
    }
    
    // Generate a simple challenge for demo purposes
    let challenge = URL_SAFE_NO_PAD.encode(&Uuid::new_v4().as_bytes());
    let user_id = Uuid::new_v4();
    
    // Store challenge temporarily
    {
        let mut challenges = CHALLENGES.lock().unwrap();
        challenges.insert(form.username.clone(), challenge.clone());
    }
    
    // Return a simplified WebAuthn creation options
    let response = json!({
        "challenge": challenge,
        "rp": {
            "name": "WebAuthn Sample",
            "id": "localhost"
        },
        "user": {
            "id": URL_SAFE_NO_PAD.encode(user_id.as_bytes()),
            "name": form.username,
            "displayName": form.username
        },
        "pubKeyCredParams": [
            {
                "type": "public-key",
                "alg": -7
            },
            {
                "type": "public-key", 
                "alg": -257
            }
        ],
        "authenticatorSelection": {
            "authenticatorAttachment": "platform",
            "userVerification": "required"
        },
        "timeout": 300000,
        "attestation": "direct"
    });
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn register_response(
    pool: web::Data<MySqlPool>,
    reg: web::Json<Value>,
) -> Result<HttpResponse> {
    // Extract credential ID from the response
    let credential_id = reg["rawId"].as_str()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing credential ID"))?;
    
    let credential_id_bytes = URL_SAFE_NO_PAD.decode(credential_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid credential ID"))?;
    
    // Extract username from clientDataJSON for demo
    // In production, you would properly parse and validate the clientDataJSON
    let client_data_json = reg["response"]["clientDataJSON"].as_str()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing clientDataJSON"))?;
    
    let client_data_bytes = URL_SAFE_NO_PAD.decode(client_data_json)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid clientDataJSON"))?;
    
    let client_data_str = String::from_utf8(client_data_bytes)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid clientDataJSON encoding"))?;
    
    // For demo, extract username from challenge data or use a stored mapping
    // This is a simplified approach - in production you would store and retrieve properly
    let username = if let Some(first_username) = CHALLENGES.lock().unwrap().keys().next().cloned() {
        first_username
    } else {
        return Err(actix_web::error::ErrorBadRequest("No pending registration"));
    };
    
    let user_id = db::create_user(&pool, &username).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create user"))?;
    
    // For demo, we'll store a placeholder public key
    let placeholder_pubkey = b"placeholder_public_key_data";
    
    db::save_credential(&pool, user_id, &credential_id_bytes, placeholder_pubkey).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save credential"))?;
    
    // Clear the challenge
    CHALLENGES.lock().unwrap().remove(&username);
    
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn login_request(
    pool: web::Data<MySqlPool>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Check if user exists
    let user = db::get_user_by_username(&pool, &form.username).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorBadRequest("User not found"))?;
    
    let credentials = db::get_credentials_by_user_id(&pool, user.id).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
    
    if credentials.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "No credentials found for user"
        })));
    }
    
    // Generate challenge
    let challenge = URL_SAFE_NO_PAD.encode(&Uuid::new_v4().as_bytes());
    
    // Store challenge
    {
        let mut challenges = CHALLENGES.lock().unwrap();
        challenges.insert(form.username.clone(), challenge.clone());
    }
    
    // Return authentication options
    let allow_credentials: Vec<Value> = credentials.into_iter().map(|cred| {
        json!({
            "type": "public-key",
            "id": URL_SAFE_NO_PAD.encode(&cred.credential_id)
        })
    }).collect();
    
    let response = json!({
        "challenge": challenge,
        "timeout": 300000,
        "rpId": "localhost",
        "allowCredentials": allow_credentials,
        "userVerification": "required"
    });
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn login_response(
    pool: web::Data<MySqlPool>,
    auth: web::Json<Value>,
) -> Result<HttpResponse> {
    // Extract credential ID from the response
    let credential_id = auth["rawId"].as_str()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing credential ID"))?;
    
    let credential_id_bytes = URL_SAFE_NO_PAD.decode(credential_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid credential ID"))?;
    
    // Get the username from stored challenge data
    let username = if let Some(first_username) = CHALLENGES.lock().unwrap().keys().next().cloned() {
        first_username
    } else {
        return Err(actix_web::error::ErrorBadRequest("No pending authentication"));
    };
    
    let user = db::get_user_by_username(&pool, &username).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorBadRequest("User not found"))?;
    
    let credentials = db::get_credentials_by_user_id(&pool, user.id).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
    
    let credential_exists = credentials.iter().any(|cred| cred.credential_id == credential_id_bytes);
    
    if !credential_exists {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Invalid credential"
        })));
    }
    
    // Clear the challenge
    CHALLENGES.lock().unwrap().remove(&username);
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "username": username
    })))
}