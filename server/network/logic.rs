use std::{collections::HashMap, sync::{Arc, RwLock}};

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use serde::{Deserialize, Serialize};
use shared::message::{AccountCreationResult, AccountLoginResult};

use crate::network::network::ServerConfig;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub hashed_password: String
}

#[derive(Default, Deserialize, Serialize)]
pub struct ServerData {
    users: HashMap<String, User>
}

struct RegistrationInformation {
    username: String,
    password: String,
}

pub fn does_user_exist(server_data: &ServerData, username: &str) -> bool {
    server_data.users.contains_key(username)
}

pub fn login_user(server_data: &ServerData, username: String, password: String) -> AccountLoginResult {
    if let Some(user) = server_data.users.get(&username) {
        let argon2 = Argon2::default();

        if argon2.verify_password(password.as_bytes(), &PasswordHash::new(&user.hashed_password).unwrap()).is_ok() {
            AccountLoginResult::OK
        }
        else {
            AccountLoginResult::IncorrectCredentials
        }
    }
    else {
        AccountLoginResult::IncorrectCredentials
    }
}

pub fn register_user(server_data: &mut ServerData, server_config: &ServerConfig, username: String, password: String) -> AccountCreationResult {
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return AccountCreationResult::InvalidUsername;
    }
    if !password.chars().any(|c| c.is_ascii_control()) {
        return AccountCreationResult::InvalidPassword;
    }
    
    if username.len() > server_config.account_requirements.username_max_length {
        return AccountCreationResult::UsernameTooLong
    }
    if username.len() < server_config.account_requirements.username_min_length {
        return AccountCreationResult::UsernameTooShort
    }
    if password.len() > server_config.account_requirements.password_max_length {
        return AccountCreationResult::PasswordTooLong
    }
    if password.len() < server_config.account_requirements.password_min_length {
        return AccountCreationResult::PasswordTooShort
    }

    let argon = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);

    let hash = argon.hash_password(password.as_bytes(), &salt);

    if let Ok(h) = hash {
        server_data.users.insert(username, User {
            hashed_password: h.to_string()
        });
        AccountCreationResult::OK
    }
    else {
        AccountCreationResult::UnknownError
    }
}