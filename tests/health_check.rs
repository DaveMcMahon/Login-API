use login_api::configuration::get_configuration;
use login_api::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

struct AppData {
    address_string: String,
    connection: PgPool,
}

#[tokio::test]
async fn check_200_status_from_login() {
    let app_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=username%20secondname&email=username%40domain.com";
    let response = client
        .post(&format!("{}/login", &app_data.address_string))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Request failed");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name from logins",)
        .fetch_one(&app_data.connection)
        .await
        .expect("Couldn't execute query");
    assert_eq!(saved.email, "username@domain.com");
    assert_eq!(saved.name, "username secondname");
}

#[tokio::test]
async fn check_400_status_from_logon() {
    let app_data = spawn_app().await;
    let client = reqwest::Client::new();

    let incorrect = vec![
        ("name=david%20mcmahon", "no email specified"),
        ("email=davidmcmhn%40gmail.com", "no name specified"),
        ("", "no name or email specified"),
    ];

    for (incorrect_body, error_message) in incorrect {
        let response = client
            .post(&format!("{}/login", &app_data.address_string))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(incorrect_body)
            .send()
            .await
            .expect("Request failed");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API login call did not fail when payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn health_check_works() {
    let app_data = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_data.address_string))
        .send()
        .await
        .expect("Couldn't make the request");

    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> AppData {
    let config = get_configuration().expect("Couldn't read config from config.yaml");
    let connection_string = config.database.get_connection_string();
    let db_connection = PgPool::connect(&connection_string)
        .await
        .expect("Couldn't connect to database");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, db_connection.clone()).expect("Failed to bind address");
    let address = format!("http://127.0.0.1:{}", port);

    let _ = tokio::spawn(server);

    AppData {
        address_string: address,
        connection: db_connection,
    }
}
