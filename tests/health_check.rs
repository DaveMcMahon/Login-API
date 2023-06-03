use login_api::connection::database_init;

#[tokio::test]
async fn check_200_status_from_login() {
    let app_data = database_init::spawn_app().await;
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
    let app_data = database_init::spawn_app().await;
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
    let app_data = database_init::spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_data.address_string))
        .send()
        .await
        .expect("Couldn't make the request");

    assert_eq!(Some(0), response.content_length());
}
