extern crate diesel;

use juniper;
use juniper::{FieldError, FieldResult, RootNode};

use diesel::prelude::*;

use crate::db::Pool;
use crate::schema::{products, users};

use super::product::{Product, ProductInput};
use super::user::{User, UserInput};

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
        let all_users = users::table
            .select(users::all_columns)
            .load::<User>(&*conn)
            .unwrap();
        Ok(all_users)
    }

    #[graphql(description = "Get single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();
        let user = users::table.find(id).first::<User>(&*conn);
        match user {
            Ok(user) => Ok(user),
            Err(err) => Err(FieldError::new(
                "User Not Found",
                graphql_value!({"not_found": "user not found"}),
            )),
        }
    }

    #[graphql(description = "List of all products")]
    fn products(context: &Context) -> FieldResult<Vec<Product>> {
        let mut conn = context.dbpool.get().unwrap();
        let all_products = products::table
            .select(products::all_columns)
            .load::<Product>(&*conn)
            .unwrap();
        Ok(all_products)
    }

    #[graphql(description = "Get single product reference by product ID")]
    fn product(context: &Context, id: String) -> FieldResult<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let product = products::table.find(id).first::<Product>(&*conn);
        match product {
            Ok(product) => Ok(product),
            Err(err) => Err(FieldError::new(
                "Product Not Found",
                graphql_value!({"not_found": "product not found"}),
            )),
        }
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, user: UserInput) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        let new_user = User {
            id: new_id,
            name: user.name,
            email: user.email,
        };
        let insert = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&*conn);
        match insert {
            Ok(_) => Ok(new_user),
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

        let new_product = Product {
            id: new_id,
            user_id: product.user_id,
            name: product.name,
            price: product.price,
        };
        let insert = diesel::insert_into(products::table)
            .values(&new_product)
            .execute(&*conn);
        match insert {
            Ok(_) => Ok(new_product),
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
