use crate::logic::login::user;

#[derive(Queryable,Debug)]
pub struct User {
    pub id: String,
    pub name: String
}

impl User {
    pub fn map_to_user(&self) -> user::User {
        user::User{
            id: self.id.clone(),
            name: self.name.clone(),
        }
    }
}