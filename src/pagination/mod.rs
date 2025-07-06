use serde::Serialize;

pub mod offset;

#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: usize, // total records in DB (for frontend pagination)
    pub page: usize,
    pub page_size: usize,
}
