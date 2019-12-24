use postgres::{Connection, TlsMode};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = Connection::connect(db_url.as_str(), TlsMode::None).unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
}
