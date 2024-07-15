use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
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
            balance: 0,
            contacts: vec!["456".to_string()],
            history: vec![],
        });
    }

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /contacts?alias={alias}` goes to `get_contacts`
        .route("/contacts", get(get_contacts))
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

struct Database {
    users: Vec<User>,
}

// add user to Database
impl Database {
    fn add_user(&mut self, user: User) {
        self.users.push(user);
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

#[derive(Deserialize)]
struct ContactsQuery {
    number: String,
}

#[derive(Deserialize)]
struct HistoryQuery {
    number: String,
}

#[derive(Serialize)]
struct OperationQuery {
    from: String,
    to: String,
    value: u32,
}

async fn get_contacts(
    Query(params): Query<ContactsQuery>,
    State(db): State<Arc<Mutex<Database>>>,
) -> Json<Vec<String>> {
    let db = db.lock().unwrap();
    Json(db.get_contacts(&params.alias))
}

struct User {
    number: String,
    name: String,
    balance: u32,
    contacts: Vec<String>,
    history: Vec<Operation>,
}

#[derive(Clone)]
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
