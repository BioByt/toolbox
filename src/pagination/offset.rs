use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OffsetPagination {
    pub page: usize,      // 1-based page number
    pub page_size: usize, // Number of items per page
}
