use query_filters_macros::QueryFilters;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct PageParams {
    pub page_num: u64,
    pub page_size: u64,
}

impl Default for PageParams {
    fn default() -> Self {
        Self {
            page_num: 1,
            page_size: 10,
        }
    }
}

// Serialize + DeserializeOwned + Clone
#[derive(Debug, Serialize, Clone)]
pub struct PageData<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub total_pages: u64,
    pub page_num: u64,
}

#[derive(Debug, Default, QueryFilters, Deserialize)]
pub struct NoneSearchParams;
