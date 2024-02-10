
pub async fn init()-> Result<libsql::Connection, String> {
    let db = if let Ok(url) = std::env::var("LIBSQL_URL") {
        let token = std::env::var("LIBSQL_AUTH_TOKEN").unwrap_or_else(|_| {
            println!("LIBSQL_TOKEN not set, using empty token...");
            "".to_string()
        });

        libsql::Database::open_remote(url, token).map_err(|e|e.to_string())?
    } else {
        libsql::Database::open_in_memory().map_err(|e|e.to_string())?
    };
    let connection  = db.connect().map_err(|e|e.to_string())?;

    connection.execute(CREATE, ()).await.unwrap();



    return Ok(connection)
}

const CREATE: &str = r#"
CREATE table if not exists users (
    code varchar(255) primary key,
    user_name varchar(255),
    content BLOB
)
"#;
