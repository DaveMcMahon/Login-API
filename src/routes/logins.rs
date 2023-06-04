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
    log::info!("Logging new users logon details in the Database");

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
                log::info!("Users details weree stored successfulyly in the database");
                HttpResponse::Ok().finish()
            },
        Err(e) => {
            log::error!("Failed to execute query -> {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
