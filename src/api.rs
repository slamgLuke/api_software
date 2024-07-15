use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub async fn api() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // init db
    let db = Arc::new(Mutex::new(Database {
        users: vec![],
        messages: vec![],
    }));

    // 3 dummy users
    {
        let mut db = db.lock().unwrap();
        db.add_user(User {
            alias: "anlec".to_string(),
            name: "Marcelo".to_string(),
            contacts: vec!["rudolfinsito".to_string(), "lenx".to_string()],
        });

        db.add_user(User {
            alias: "rudolfinsito".to_string(),
            name: "Lucas".to_string(),
            contacts: vec!["anlec".to_string()],
        });

        db.add_user(User {
            alias: "lenx".to_string(),
            name: "Lenin".to_string(),
            contacts: vec!["anlec".to_string()],
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
    messages: Vec<Message>,
}

// add user to Database
impl Database {
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn get_contacts(&self, alias: &str) -> Vec<String> {
        let user = self.users.iter().find(|u| u.alias == alias);
        match user {
            Some(user) => user.contacts.clone(),
            None => vec!["User not found".to_string()],
        }
    }

    fn send_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn get_messages(&self, alias: &str) -> Vec<Message> {
        self.messages
            .iter()
            .filter(|m| m.to == alias)
            .cloned()
            .collect()
    }
}

#[derive(Deserialize)]
struct ContactsQuery {
    alias: String,
}

async fn get_contacts(
    Query(params): Query<ContactsQuery>,
    State(db): State<Arc<Mutex<Database>>>,
) -> Json<Vec<String>> {
    let db = db.lock().unwrap();
    Json(db.get_contacts(&params.alias))
}

struct User {
    alias: String,
    name: String,
    contacts: Vec<String>,
}

#[derive(Clone)]
struct Message {
    from: String,
    to: String,
    content: String,
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
