use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct FactListResponse {
    pub catfact: Option<String>,
}
