use axum::{
    debug_handler,
    extract::{Query, State},
    routing::get,
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
        .route("/operation", get(make_operation))
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

    fn get_contacts(&self, number: &str) -> Option<Vec<(String, String)>> {
        let user = self.users.iter().find(|u| u.number == number);
        if user.is_none() {
            return None;
        }

        let user = user.unwrap();
        let contacts = user
            .contacts
            .iter()
            .map(|c| {
                let contact = self.users.iter().find(|u| u.number == *c);
                match contact {
                    Some(contact) => Some((contact.name.clone(), contact.number.clone())),
                    None => None,
                }
            })
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect::<Vec<(String, String)>>();
        Some(contacts)
    }

    fn get_history(&self, number: &str) -> Option<(User, Vec<Operation>)> {
        let user = self.users.iter().find(|u| u.number == number);
        let received_transactions = self
            .users
            .iter()
            .filter(|u| u.number != number)
            .filter(|u| u.contacts.contains(&number.to_string()))
            .flat_map(|u| u.history.iter().cloned())
            .collect::<Vec<Operation>>();
        if user.is_none() {
            return None;
        }
        let user = user.unwrap();
        Some((user.clone(), received_transactions))
    }

    fn make_operation(&mut self, operation: Operation) {
        let user_from = self.users.iter_mut().find(|u| u.number == operation.from);

        match user_from {
            Some(user) => user.balance -= operation.value,
            None => (),
        }

        let user_to = self.users.iter_mut().find(|u| u.number == operation.to);

        match user_to {
            Some(user) => user.balance += operation.value,
            None => (),
        }

        self.add_operation(operation);
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
) -> String {
    let db = db.lock().unwrap();
    let contacts = db.get_contacts(&params.number);

    if contacts.is_none() {
        return "User not found".to_string();
    }

    let contacts = contacts.unwrap();
    let mut str = "".to_string();
    for (name, number) in contacts.iter() {
        str.push_str(&format!("{}: {}\n", name, number));
    }
    str
}

#[debug_handler]
async fn get_history(
    Query(params): Query<HistoryQuery>,
    State(db): State<Arc<Mutex<Database>>>,
) -> String {
    let db = db.lock().unwrap();
    let data = db.get_history(&params.number);
    if data.is_none() {
        return "User not found".to_string();
    }
    let (user_data, received_transactions) = data.unwrap();
    let mut combined_transactions = user_data
        .history
        .iter()
        .chain(received_transactions.iter())
        .collect::<Vec<&Operation>>();

    // sort by date (string)
    combined_transactions.sort_by(|a, b| a.date.cmp(&b.date));
    combined_transactions.reverse();

    let mut str = format!(
        "{}'s balance: {}\n{}'s operations\n",
        user_data.name, user_data.balance, user_data.name
    );
    for operation in combined_transactions.iter() {
        if operation.from == user_data.number {
            str.push_str(&format!(
                "Sent {} to {} at {}\n",
                operation.value, operation.to, operation.date
            ));
        } else {
            str.push_str(&format!(
                "Received {} from {} at {}\n",
                operation.value, operation.from, operation.date
            ));
        }
    }

    str
}

#[debug_handler]
async fn make_operation(
    State(db): State<Arc<Mutex<Database>>>,
    Query(operation): Query<OperationQuery>,
) -> String {
    let mut db = db.lock().unwrap();
    let user_from = db.get_user(&operation.from);
    let user_to = db.get_user(&operation.to);

    if user_from.is_none() || user_to.is_none() {
        println!("Operation with invalid user");
        return "Operation with invalid user".to_string();
    }

    let user_from = user_from.unwrap();

    if user_from.balance < operation.value {
        println!("Operation with insufficient funds");
        return "Operation with insufficient funds".to_string();
    }

    let operation = Operation {
        from: operation.from,
        to: operation.to,
        value: operation.value,
        date: chrono::offset::Local::now().to_string(),
    };

    println!("Operation made: {:#?}", operation);
    db.make_operation(operation);
    "Operation made".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct User {
    number: String,
    name: String,
    balance: u32,
    contacts: Vec<String>,
    history: Vec<Operation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Operation {
    from: String,
    to: String,
    value: u32,
    date: String,
}
