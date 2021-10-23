use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
pub struct Response {
    message: String
}

#[get("/health")]
pub fn serve() -> Json<Response> {
    Json(Response { message: "Health Check...".to_string() })
}