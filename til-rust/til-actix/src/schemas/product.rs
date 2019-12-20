use juniper;
use postgres::Row;

use super::root::Context;
use super::user::User;
use super::util;
use postgres::error::Error;

/// Product
#[derive(Default, Debug)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub price: f64,
}

#[juniper::object(Context = Context)]
impl Product {
    fn id(&self) -> &str {
        &self.id
    }
    fn user_id(&self) -> &str {
        &self.user_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn price(&self) -> f64 {
        self.price
    }

    fn user(&self, context: Context) -> Option<User> {
        let mut conn = context.dbpool.get().unwrap();
        let user =
            conn.query_one("SELECT * FROM users WHERE id=:id", &[&self.user_id()]);
        if let Err(err) = user {
            None
        } else {
            Some(util::create_user_from_row(&user.unwrap()))
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Product Input")]
pub struct ProductInput {
    pub user_id: String,
    pub name: String,
    pub price: f64,
}
