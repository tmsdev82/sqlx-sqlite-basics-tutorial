use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Clone, FromRow, Debug)]
struct User {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(&db).await.unwrap();
    println!("Create user table result: {:?}", result);

    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&db)
    .await
    .unwrap();

    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    let result = sqlx::query("INSERT INTO users (name) VALUES ('billy')")
        .execute(&db)
        .await
        .unwrap();

    println!("Query result: {:?}", result);

    let user_results = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&db)
        .await
        .unwrap();

    for user in user_results {
        println!("[{}] name: {}", user.id, &user.name);
    }
}
