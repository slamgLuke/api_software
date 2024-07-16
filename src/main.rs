mod api;
mod unit_testing;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    api::api("0.0.0.0:3004".to_string()).await;
}
