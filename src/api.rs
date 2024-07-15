use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub async fn api() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // init db
    let db = Arc::new(Mutex::new(Database { users: vec![] }));

    // 3 dummy users
    {
        let mut db = db.lock().unwrap();
        db.add_user(User {
            number: "123".to_string(),
            name: "Marcelo".to_string(),
            balance: 100,
            contacts: vec!["456".to_string(), "789".to_string()],
            history: vec![],
        });

        db.add_user(User {
            number: "456".to_string(),
            name: "Juan".to_string(),
            balance: 200,
            contacts: vec!["123".to_string()],
            history: vec![],
        });

        db.add_user(User {
            number: "789".to_string(),
            name: "Pedro".to_string(),
            balance: 300,
            contacts: vec!["123".to_string()],
            history: vec![],
        });
    }

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /contacts?alias={alias}` goes to `get_contacts`
        .route("/contacts", get(get_contacts))
        // `GET /history?alias={alias}` goes to `get_history`
        .route("/history", get(get_history))
        // `POST /operation` goes to `add_operation`
        .route("/operation", get(add_operation))
        .with_state(db.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::Server::from_tcp(listener)
    //     .unwrap()
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct Database {
    users: Vec<User>,
}

// add user to Database
impl Database {
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn get_user(&self, number: &str) -> Option<&User> {
        self.users.iter().find(|u| u.number == number)
    }

    fn get_contacts(&self, number: &str) -> Vec<String> {
        let user = self.users.iter().find(|u| u.number == number);
        match user {
            Some(user) => user.contacts.clone(),
            None => vec!["User not found".to_string()],
        }
    }

    fn get_history(&self, number: &str) -> Vec<Operation> {
        let user = self.users.iter().find(|u| u.number == number);
        match user {
            Some(user) => user.history.clone(),
            None => vec![Operation {
                from: "User not found".to_string(),
                to: "User not found".to_string(),
                value: 0,
                date: "User not found".to_string(),
            }],
        }
    }

    fn add_operation(&mut self, operation: Operation) {
        let user = self
            .users
            .iter_mut()
            .find(|u| u.number == operation.from || u.number == operation.to);
        match user {
            Some(user) => user.history.push(operation),
            None => (),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ContactsQuery {
    number: String,
}

#[derive(Deserialize, Serialize)]
struct HistoryQuery {
    number: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct OperationQuery {
    from: String,
    to: String,
    value: u32,
}

#[debug_handler]
async fn get_contacts(
    Query(params): Query<ContactsQuery>,
    State(db): State<Arc<Mutex<Database>>>,
) -> Json<Vec<String>> {
    let db = db.lock().unwrap();
    Json(db.get_contacts(&params.number))
}

#[debug_handler]
async fn get_history(
    Query(params): Query<HistoryQuery>,
    State(db): State<Arc<Mutex<Database>>>,
) -> Json<Vec<Operation>> {
    let db = db.lock().unwrap();
    Json(db.get_history(&params.number))
}

#[debug_handler]
async fn add_operation(
    State(db): State<Arc<Mutex<Database>>>,
    Query(operation): Query<OperationQuery>,
) -> StatusCode {
    let mut db = db.lock().unwrap();

    let user_from = db.get_user(&operation.from);

    let user_to = db.get_user(&operation.to);

    if user_from.is_none() || user_to.is_none() {
        return StatusCode::BAD_REQUEST;
    }

    let user_from = user_from.unwrap();

    if user_from.balance < operation.value {
        return StatusCode::BAD_REQUEST;
    }

    db.add_operation(Operation {
        from: operation.from,
        to: operation.to,
        value: operation.value,
        date: chrono::offset::Local::now().to_string(),
    });
    StatusCode::CREATED
}

#[derive(Clone, Deserialize, Serialize)]
struct User {
    number: String,
    name: String,
    balance: u32,
    contacts: Vec<String>,
    history: Vec<Operation>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Operation {
    from: String,
    to: String,
    value: u32,
    date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        assert_eq!("hola", "hola")
    }
}
