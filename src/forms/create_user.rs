
#[derive(FromForm)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
