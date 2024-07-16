#[cfg(test)]
mod test {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};

    // Importa la función api desde el archivo principal
    use crate::api::*;

    #[tokio::test]
    async fn test_valid_operation() {
        // Inicializa el servidor en segundo plano
        tokio::spawn(async {
            api("0.0.0.0:3000".to_string()).await;
        });

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Crea un cliente de reqwest
        let client = Client::new();

        // Envía una solicitud de operación válida
        let response = client
            .get("http://127.0.0.1:3000/operation")
            .query(&OperationQuery {
                from: "123".to_string(),
                to: "456".to_string(),
                value: 50,
            })
            .send()
            .await
            .unwrap();

        let body = response.text().await.unwrap();

        assert_eq!(body, "Operation made");
    }

    #[tokio::test]
    async fn test_valid_user_contacts() {
        // Inicializa el servidor en segundo plano
        tokio::spawn(async {
            api("0.0.0.0:3001".to_string()).await;
        });

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Crea un cliente de reqwest
        let client = Client::new();

        // Envía una solicitud de operación válida
        let response = client
            .get("http://127.0.0.1:3001/contacts")
            .query(&ContactsQuery {
                number: "123".to_string(),
            })
            .send()
            .await
            .unwrap();

        let body = response.text().await.unwrap();

        assert_ne!(body, "User not found");
    }

    #[tokio::test]
    async fn test_invalid_user() {
        // Inicializa el servidor en segundo plano
        tokio::spawn(async {
            api("0.0.0.0:3002".to_string()).await;
        });

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Crea un cliente de reqwest
        let client = Client::new();

        // Envía una solicitud de operación inválida
        let response = client
            .get("http://127.0.0.1:3002/operation")
            .query(&OperationQuery {
                from: "123".to_string(),
                to: "999".to_string(), // Usuario inválido
                value: 50,
            })
            .send()
            .await
            .unwrap();

        let body = response.text().await.unwrap();

        assert_eq!(body, "Operation with invalid user");
    }

    #[tokio::test]
    async fn test_invalid_balance() {
        // Inicializa el servidor en segundo plano
        tokio::spawn(async {
            api("0.0.0.0:3003".to_string()).await;
        });

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Crea un cliente de reqwest
        let client = Client::new();

        // Envía una solicitud de operación inválida
        let response = client
            .get("http://127.0.0.1:3003/operation")
            .query(&OperationQuery {
                from: "123".to_string(),
                to: "456".to_string(),
                value: 300, // Balance insuficiente
            })
            .send()
            .await
            .unwrap();

        let body = response.text().await.unwrap();

        assert_eq!(body, "Operation with insufficient funds");
    }

    #[tokio::test]
    async fn test_zero_operation() {
        // Inicializa el servidor en segundo plano
        tokio::spawn(async {
            api("0.0.0.0:3004".to_string()).await;
        });

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Crea un cliente de reqwest
        let client = Client::new();

        // Envía una solicitud de operación inválida
        let response = client
            .get("http://127.0.0.1:3004/operation")
            .query(&OperationQuery {
                from: "123".to_string(),
                to: "456".to_string(),
                value: 0, // Valor 0
            })
            .send()
            .await
            .unwrap();

        let body = response.text().await.unwrap();

        assert_eq!(body, "Operation with value 0");
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct OperationQuery {
        from: String,
        to: String,
        value: u32,
    }

    #[derive(Deserialize, Serialize)]
    struct ContactsQuery {
        number: String,
    }
}

