mod api;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    api::api().await;
}
