use std::time::{SystemTime, UNIX_EPOCH};
use rocket::{get, post, Request, request::FromRequest, request::Outcome, State};
use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize, json::Json};
use bcrypt::{verify, hash, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use std::env;
use crate::database::DbClient;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub user_id: i32,
    pub username: String,
    pub exp: usize,
    pub role: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SignupResponse {
    pub message: String,
}


pub struct JwtToken(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
    type Error = std::io::Error;

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // Get the `Authorization` header from the request.
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            // Check if the header starts with "Bearer ".
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                match validate_jwt(token) {
                    Ok(claims) => {
                        // Return the decoded claims wrapped in the JwtToken.
                        Outcome::Success(JwtToken(claims))
                    }
                    Err(_) => {
                        // Token decoding failed, return Unauthorized status.
                        Outcome::Error((Status::Unauthorized, std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid token")))
                    }
                }
            } else {
                Outcome::Error((Status::Unauthorized, std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing Bearer token")))
            }
        } else {
            Outcome::Error((Status::Unauthorized, std::io::Error::new(std::io::ErrorKind::NotFound, "Authorization header missing")))
        }
    }
}


fn get_secret_key() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| String::from("secret"))
}

pub fn create_jwt(user_id: i32, username: &str, role: &str) -> String {
    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Issue with time...?")
        .as_secs() + 3600;
    let expiration = expiration_time as usize;
    let claims = Claims {
        user_id,
        username: username.to_owned(),
        exp: expiration,
        role: role.to_owned(),
    };

    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(get_secret_key().as_ref());

    encode(&header, &claims, &key).expect("Failed to encode JWT")
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(get_secret_key().as_ref());
    let validation = Validation::new(Algorithm::HS256);

    //TODO: handle banned users
    decode::<Claims>(token, &key, &validation).map(|data| data.claims)
}



// Signup route to create a new user
#[post("/signup", data = "<signup_data>")]
pub async fn signup(signup_data: Json<LoginData>, db_client: &State<DbClient>) -> Result<Json<SignupResponse>, Status> {
    let client = db_client.client.lock().await;

    // Check if the username already exists
    let stmt = client.prepare("SELECT user_id FROM users WHERE username = $1").await
        .map_err(|_| Status::InternalServerError)?;

    let rows = client.query(&stmt, &[&signup_data.username]).await
        .map_err(|_| Status::InternalServerError)?;

    if !rows.is_empty() {
        return Err(Status::BadRequest);  // Username already taken
    }

    // Hash the password using bcrypt
    let password_hash = hash(&signup_data.password, DEFAULT_COST)
        .map_err(|_| Status::InternalServerError)?;

    // Insert the new user into the database
    let stmt = client.prepare("INSERT INTO users (username, password_hash) VALUES ($1, $2)").await
        .map_err(|_| Status::InternalServerError)?;

    client.execute(&stmt, &[&signup_data.username, &password_hash]).await
        .map_err(|_| Status::InternalServerError)?;

    // Return a response indicating successful signup
    Ok(Json(SignupResponse {
        message: "User created successfully".to_string(),
    }))
}


#[post("/login", data = "<login_data>")]
pub async fn login(login_data: Json<LoginData>, db_client: &State<DbClient>) -> Result<Json<LoginResponse>, Status> {
    let client = db_client.client.lock().await;

    // Prepare a statement to fetch the user from the database
    let stmt = client.prepare("SELECT user_id, username, password_hash, role FROM users WHERE username = $1").await
        .map_err(|_| Status::InternalServerError)?;

    // Query the database for the user
    let rows = client.query(&stmt, &[&login_data.username]).await
        .map_err(|_| Status::Unauthorized)?;

    if rows.is_empty() {
        return Err(Status::Unauthorized);
    }

    let row = &rows[0];
    let stored_hash: String = row.get(2);  // Get the stored password hash

    // Verify the password
    if !verify(&login_data.password, &stored_hash).unwrap_or(false) {
        return Err(Status::Unauthorized);
    }

    let user_id: i32 = row.get(0);
    let role: String = row.get(3);

    // Create JWT if credentials are valid
    let jwt = create_jwt(user_id, &login_data.username, &role);

    Ok(Json(LoginResponse { token: jwt }))
}

#[get("/test/protected")]
pub async fn protected_route(jwt_token: JwtToken) -> Result<Json<String>, Status> {
    // You can now use jwt_token.0 (the username/sub claim) directly
    Ok(Json(format!("Welcome, {}", jwt_token.0.username)))
}
