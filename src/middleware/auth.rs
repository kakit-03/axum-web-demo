use std::sync::Arc;
use axum::{body::Body, Extension, extract::{Json, Request}, http, http::{Response, StatusCode}, middleware::Next};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
// Define a structure for holding claims data used in JWT tokens
pub struct Claims {
    pub exp: usize,  // Expiry time of the token
    pub iat: usize,  // Issued at time of the token
    pub email: String,  // Email associated with the token
}

// Define a structure for holding sign-in data
#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,  // Email entered during sign-in
    pub password: String,  // Password entered during sign-in
}

// Function to handle sign-in requests
pub async fn sign_in(
    Json(user_data): Json<SignInData>,  // JSON payload containing sign-in data
) -> Result<Json<String>, StatusCode> {  // Return type is a JSON-wrapped string or an HTTP status code

    // Attempt to retrieve user information based on the provided email
    let user = match retrieve_user_by_email(&user_data.email) {
        Some(user) => user,  // User found, proceed with authentication
        None => return Err(StatusCode::UNAUTHORIZED), // User not found, return unauthorized status
    };

    // Verify the password provided against the stored hash
    if !verify_password(&user_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? // Handle bcrypt errors
    {
        return Err(StatusCode::UNAUTHORIZED); // Password verification failed, return unauthorized status
    }

    // Generate a JWT token for the authenticated user
    let token = encode_jwt(user.email)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Handle JWT encoding errors

    // Return the token as a JSON-wrapped string
    Ok(Json(token))
}

#[derive(Clone)]
pub struct CurrentUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

// Function to simulate retrieving user data from a database based on email
fn retrieve_user_by_email(email: &str) -> Option<CurrentUser> {
    // For demonstration purposes, a hardcoded user is returned based on the provided email
    let current_user: CurrentUser = CurrentUser {
        email: "myemail@gmail.com".to_string(),
        first_name: "Eze".to_string(),
        last_name: "Sunday".to_string(),
        password_hash: "$2b$12$Gwf0uvxH3L7JLfo0CC/NCOoijK2vQ/wbgP.LeNup8vj6gg31IiFkm".to_string(),
    };
    Some(current_user) // Return the hardcoded user
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let secret: String = "randomStringTypicallyFromEnv".to_string();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, email };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = "randomStringTypicallyFromEnv".to_string();
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    //RSA 加密算法
    // let result = encode(&Header::new(Algorithm::RS256), &my_claims, &EncodingKey::from_rsa_pem(include_bytes!("privkey.pem"))?)?;
    //RSA解密
    //let result = decode::<Claims>(&jwt_token, &DecodingKey::from_rsa_components(jwk["n"], jwk["e"]), &Validation::new(Algorithm::RS256))?;
    result
}


pub async fn kakit_authorization_middleware(Extension(state): Extension<Arc<AppState>>,mut req: Request, next: Next) -> Result<Response<Body>, AppError> {
    // let conn = &state.conn;
    let redis_pool = &state.redis;
    let mut conn = redis_pool.get().await.unwrap();
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AppError::auth_err("Empty header is not allowed"))?,
        None => return Err(AppError::auth_err("Please add the JWT token to the header")),
    };
    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(), header.next());
    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => return Err(AppError::un_auth_err("Unable to decode token")),
    };
    // Fetch the user details from the database
    let current_user = match retrieve_user_by_email(&token_data.claims.email) {
        Some(user) => user,
        None => return Err(AppError::un_auth_err("You are not an authorized user")),
    };
    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}