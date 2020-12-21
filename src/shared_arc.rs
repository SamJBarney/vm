use std::sync::Arc;
use std::sync::RwLock;

pub type SharedArc<T> = Arc<RwLock<T>>;