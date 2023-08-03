use crate::{model::DB, response::FactListResponse};
use axum::{http::StatusCode, Json};

pub async fn facts_list_handler(db: DB) -> Result<Json<FactListResponse>, StatusCode> {
    // Acquire a write lock on the `DB` to access its contents mutably.
    let mut write_facts: std::sync::RwLockWriteGuard<'_, Vec<String>> = db.write().unwrap();

    // If there is an element, remove it from the vector and store it in the `catfact` variable.
    let catfact = write_facts.pop();

    // Create the `FactListResponse` containing the `catfact`.
    let json_response = FactListResponse { catfact };

    Ok(Json(json_response))
}
