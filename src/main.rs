mod api;
mod unit_testing;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    api::api().await;
}
