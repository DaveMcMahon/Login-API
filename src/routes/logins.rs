use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn login(form: web::Form<FormData>, connection_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    log::info!(
        "Request ID: {} -> {} {} is logging in..",
        request_id,
        form.name,
        form.email
    );
    log::info!(
        "Request ID: {} -> Logging new users logon details in the Database",
        request_id
    );

    match sqlx::query!(
        r#"
        INSERT INTO logins (id, email, name, loggedin_at)
        VALUES ($1, $2, $3, $4)
   "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection_pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "Request ID: {} -> Users details were stored successfully in the database",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "Request ID: {} -> Failed to execute query -> {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
