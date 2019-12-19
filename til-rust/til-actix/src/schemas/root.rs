use juniper;
use juniper::{FieldError, FieldResult, RootNode};
use postgres::Row;

use crate::db::Pool;

use super::product;
use super::user;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(context: &Context) -> FieldResult<Vec<()>> {}

    #[graphql(description = "Get single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<()> {}

    #[graphql(description = "Listof all products")]
    fn products(context: &Context) -> FieldResult<Vec<()>> {}

    #[graphql(description = "Get single product reference by product ID")]
    fn product(context: &Context, id: String) -> FieldResult<()> {}
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, user: ()) -> FieldResult<()> {}

    fn create_product(context: &Context, prodcut: ()) -> FieldResult<()> {}
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
