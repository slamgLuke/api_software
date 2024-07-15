use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};




#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        assert_eq!("hola", "hola")
    }
}
