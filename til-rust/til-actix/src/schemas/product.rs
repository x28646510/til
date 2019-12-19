use juniper;
use postgres::Row;

use crate::schemas::root::Context;
use crate::schemas::user::User;
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
        let user: Result<Row, Error> =
            conn.query_one("SELECT * FROM users WHERE id=:id", &[self.user_id()]);
        if let Err(err) = user {
            None
        } else {
            let id = user.unwrap().get("id");
            let name = user.unwrap().get("name");
            let email = user.unwrap().get("email");
            Some(User { id, name, email })
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
