table! {
    products (id) {
        id -> Varchar,
        user_id -> Varchar,
        name -> Varchar,
        price -> Double,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
    }
}

joinable!(products -> users (user_id));

allow_tables_to_appear_in_same_query!(products, users,);
