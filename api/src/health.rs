use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct Response {
    message: String,
}

#[get("/health")]
pub fn serve() -> Json<Response> {
    Json(Response {
        message: "Health Check...".to_string(),
    })
}
