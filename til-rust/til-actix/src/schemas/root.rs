use juniper;
use juniper::{FieldError, FieldResult, RootNode};

use crate::db::Pool;

use super::product::{Product, ProductInput};
use super::user::{User, UserInput};
use super::util;
use futures::StreamExt;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(context: &Context) -> FieldResult<Vec<User>> {
        let mut conn = context.dbpool.get().unwrap();
        let users = conn
            .query("SELECT * FROM users", &[])
            .map(|result| util::create_users_from_rows(result))
            .unwrap();
        Ok(users)
    }

    #[graphql(description = "Get single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();
        let user = conn.query_one("SELECT * FROM users WHERE id=:id", &[&id]);
        if let Err(err) = user {
            return Err(FieldError::new(
                "User Not Found",
                graphql_value!({"not_found": "user not found"}),
            ));
        }

        Ok(util::create_user_from_row(&user.unwrap()))
    }

    #[graphql(description = "List of all products")]
    fn products(context: &Context) -> FieldResult<Vec<Product>> {
        let mut conn = context.dbpool.get().unwrap();
        let products = conn
            .query("SELECT * FROM products", &[])
            .map(|result| util::create_products_from_rows(result))
            .unwrap();
        Ok(products)
    }

    #[graphql(description = "Get single product reference by product ID")]
    fn product(context: &Context, id: String) -> FieldResult<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let product = conn.query_one("SELECT * FROM products WHERE id=:id", &[&id]);
        if let Err(err) = product {
            return Err(FieldError::new(
                "Product Not Found",
                graphql_value!({"not_found": "product not found"}),
            ));
        }

        Ok(util::create_product_from_row(&product.unwrap()))
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, user: UserInput) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        let insert = conn.execute(
            "INSERT INTO users(id, name, email) VALUES(:id, :name: email)",
            &[&new_id, &user.name, &user.email],
        );

        match insert {
            Ok(number_of_row) => Ok(User {
                id: new_id,
                name: user.name,
                email: user.email,
            }),
            Err(err) => {
                let msg = format!("{}", err);
                Err(FieldError::new(
                    "Failed to create new user",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }

    fn create_product(context: &Context, product: ProductInput) -> FieldResult<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        let insert = conn.execute(
            "INSERT INTO products(id, user_id, name, price) VALUES(:id, :user_id, :name, :price",
            &[&new_id, &product.user_id, &product.name, &product.price],
        );

        match insert {
            Ok(number_of_row) => Ok(Product {
                id: new_id,
                user_id: product.user_id,
                name: product.name,
                price: product.price,
            }),
            Err(err) => {
                let msg = format!("{}", err);
                Err(FieldError::new(
                    "Failed to create new product",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
