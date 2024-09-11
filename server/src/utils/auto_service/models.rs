use serde::Serialize;

#[derive(Clone, Debug)]
pub struct PageParams {
    pub page_num: u64,
    pub page_size: u64,
}

// Serialize + DeserializeOwned + Clone
#[derive(Debug, Serialize, Clone)]
pub struct PageData<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub total_pages: u64,
    pub page_num: u64,
}
