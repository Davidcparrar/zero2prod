use actix_web::{HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
pub struct FormSubscriptionData {
    name: String,
    email: String,
}

pub async fn subscribe(subscription_form: web::Form<FormSubscriptionData>) -> impl Responder {
    let name = &subscription_form.name;
    let email = &subscription_form.email;
    println!("Received subscription from {} <{}>", name, email);
    HttpResponse::Ok().finish()
}
