use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub(crate) struct SignUp {
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) confirm_password: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LogIn {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserDb {
    pub(crate) id: i32,
    pub(crate) password: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserLinksDashBoard {
    pub(crate) short_code: String,
    pub(crate) destination_url: String,
    pub(crate) views: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserLinkCreate {
    pub(crate) destination_url: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LinkRedirect {
    pub(crate) destination_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AuthedUser {
    pub(crate) user_id: i32,
}
