use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ClientToServerMessage {
    //requests that the server starts the connection process with the given username
    ConnectionStart(String),
    RegisterAccount {
        username: String,
        password: String
    },
    LoginAccount {
        username: String,
        password: String
    },
}

#[derive(Serialize, Deserialize)]
pub enum ServerToClientMessage {
    ConnectionRequestRegister,
    ConnectionRequestLogin,
    AccountCreationResult(AccountCreationResult),
    AccountLoginResult(AccountLoginResult)
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum AccountLoginResult {
    OK,
    IncorrectCredentials
}

#[derive(Serialize, Deserialize)]
pub enum AccountCreationResult {
    OK,
    InvalidPassword,
    InvalidUsername,
    PasswordTooLong,
    PasswordTooShort,
    UsernameTooLong,
    UsernameTooShort,
    UnknownError,
    UsernameAlreadyExists
}