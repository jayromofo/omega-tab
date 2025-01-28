// use axum::extract::Json;
use stripe::{Client, Customer, ListCustomers};
// use serde::{Deserialize, Serialize};


pub struct StripeClient {}

impl StripeClient {
    pub async fn get_customer(email: &str) -> Option<Customer> {
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

    pub async fn get_subscription(customer: &Customer) -> Option<stripe::Subscription> {
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
}
