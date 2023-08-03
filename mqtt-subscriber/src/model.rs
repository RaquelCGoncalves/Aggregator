use std::sync::{Arc, RwLock};

pub type DB = Arc<RwLock<Vec<String>>>;
