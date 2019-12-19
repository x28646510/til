use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::db::Pool;
use crate::schemas;
