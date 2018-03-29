use chrono::naive::NaiveDate;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate
}
