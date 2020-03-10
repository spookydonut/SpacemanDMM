use std::env;
use warp::Filter;

/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    pretty_env_logger::init();

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(warp::log("todos"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

const secretprefix &str = "sha1=";
fn check_payload_signature(payload: String, signature_with_prefix: String) -> bool {

}

fn receive_request
match event_name {
    "pull_request" => {
        // queue up a pull request payload processing job
    },

    "check_suite" => {
        // queue up a checksuite payload processing job
    },

    "check_run" => {
        // queue up a checksuite payload processing job
    },
}
