use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use axum::extract::Json;

use stripe::{Client, Customer, ListCustomers};

mod supabase;

#[derive(Serialize)]
pub struct SubscriptionResponse {
    plan_id: String,
    current_period_end: i64,
}

#[derive(Deserialize)]
pub struct CustomerRequest {
    email: String,
}

#[derive(Serialize)]
pub struct CustomerResponse {
    session_id: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/hello", get(hello_handler))
        .route("/api/confirm-subscription", post(confirm_sub_handler))
        .route("/api/verify-subscription", post(verify_sub_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> &'static str {
    "hello!"
}

async fn verify_sub_handler(Json(payload): Json<CustomerRequest>) -> Json<SubscriptionResponse> {
    let email = payload.email;
    let customer = get_customer(&email).await;
    let subscription = get_subscription(&customer.unwrap()).await;

    match subscription {
        Some(sub) => {
            if let Some(item) = sub.items.data.into_iter().next() {
                if let Some(plan) = item.plan {
                    if plan.active.unwrap_or(false) {
                        return Json(SubscriptionResponse {
                            plan_id: plan.product.unwrap().id().to_string(),
                            current_period_end: sub.current_period_end,
                        });
                    }
                }
            }
        }
        None => {}
    }

    Json(SubscriptionResponse {
        plan_id: String::from(""),
        current_period_end: 0,
    })
}

async fn get_customer(email: &str) -> Option<Customer> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

    let client = Client::new(secret_key);

    let mut list_customers = ListCustomers::new();
    list_customers.email = Some(email);

    match Customer::list(&client, &list_customers).await {
        Ok(customers) => customers.data.into_iter().next(),
        Err(err) => {
            eprintln!("Error retrieving customer: {:?}", err);
            None
        }
    }
}

async fn get_subscription(customer: &Customer) -> Option<stripe::Subscription> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let client = Client::new(secret_key);

    let mut list_subscriptions = stripe::ListSubscriptions::new();
    list_subscriptions.customer = Some(customer.id.clone());

    match stripe::Subscription::list(&client, &list_subscriptions).await {
        Ok(subscriptions) => subscriptions.data.into_iter().next(),
        Err(err) => {
            eprintln!("Error retrieving subscription: {:?}", err);
            None
        }
    }
}
