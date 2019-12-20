use super::product::Product;
use super::user::User;
use postgres::Row;

pub fn create_user_from_row(row: &Row) -> User {
    let id = row.get("id");
    let name = row.get("name");
    let email = row.get("email");
    User { id, name, email }
}

pub fn create_users_from_rows(rows: Vec<Row>) -> Vec<User> {
    rows.iter().map(|row| create_user_from_row(row)).collect()
}

pub fn create_product_from_row(row: &Row) -> Product {
    let id = row.get("id");
    let user_id = row.get("user_id");
    let name = row.get("name");
    let price = row.get("price");
    Product {
        id,
        user_id,
        name,
        price,
    }
}

pub fn create_products_from_rows(rows: Vec<Row>) -> Vec<Product> {
    rows.iter()
        .map(|row| create_product_from_row(row))
        .collect()
}
