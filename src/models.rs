use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use mongodb::Database;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand_core::OsRng;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub hashed_password: String,
    pub created_at: DateTime,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(db: &Database, email: String, password: String) -> mongodb::error::Result<Self> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();

        let user = User {
            id: ObjectId::new(),
            email,
            hashed_password: hashed,
            created_at: DateTime::now(),
        };

        let collection = db.collection::<User>("users");
        collection.insert_one(&user, None).await?;
        Ok(user)
    }

    pub async fn find_by_email(db: &Database, email: &str) -> mongodb::error::Result<Option<User>> {
        let collection = db.collection::<User>("users");
        collection.find_one(doc! {"email": email}, None).await
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.hashed_password).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}
