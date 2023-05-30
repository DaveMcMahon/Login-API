use login_api::startup::run;
use std::net::TcpListener;

#[tokio::test]
async fn check_200_status_from_login() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=username%20secondname&email=username%40domain.com";
    let response = client
        .post(&format!("{}/login", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Request failed");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn check_400_status_from_logon() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let incorrect = vec![
        ("name=david%20mcmahon", "no email specified"),
        ("email=davidmcmhn%40gmail.com", "no name specified"),
        ("", "no name or email specified"),
    ];

    for (incorrect_body, error_message) in incorrect {
        let response = client
            .post(&format!("{}/login", &address))
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
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Couldn't make the request");

    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
