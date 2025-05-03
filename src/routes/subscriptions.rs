use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormSubscriptionData {
    name: String,
    email: String,
}

pub async fn subscribe(
    subscription_form: web::Form<FormSubscriptionData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let name = &subscription_form.name;
    let email = &subscription_form.email;

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id,email,name,subscribed_at)
        VALUES($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        email,
        name,
        Utc::now()
    )
    // Weuse`get_ref`to getanimmutablereferencetothe`PgConnection`
    // wrappedby`web::Data`.
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
