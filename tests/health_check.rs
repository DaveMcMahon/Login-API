use std::net::TcpListener;
use login_api::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client.get(&format!("{}/health_check", &address))
                        .send()
                        .await
                        .expect("Couldn't make the request");

    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
   let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port"); 
   let port  = listener.local_addr().unwrap().port();
   let server = run(listener).expect("Failed to bind address");
   let _ = tokio::spawn(server);
   format!("http://127.0.0.1:{}", port)
}
