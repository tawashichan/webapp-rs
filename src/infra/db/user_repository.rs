use diesel::prelude::*;
use crate::logic::repository::user::TUserRepository;
use crate::table::user::User;
use crate::infra::db::connection;

pub struct UserRepository {
    pool: connection::MysqlPool
}

impl UserRepository {
    pub fn new(pool: connection::MysqlPool) -> UserRepository {
        UserRepository{
            pool,
        }
    }
}

impl <'a>TUserRepository for UserRepository {
    fn find_by_id(&self,id: String) {
        use crate::schema::users::dsl::*;
        println!("loading...");
        let results = users.load::<User>(&self.pool.get().unwrap()).unwrap();
        println!("results: {:?}",results);
        /*for user in results {
            println!("{}",user.name);
        }*/
    }
}
