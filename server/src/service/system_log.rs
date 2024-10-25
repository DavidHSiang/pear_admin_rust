use query_filters_macros::QueryFilters;
use serde::Deserialize;

use crate::auto_service;
use crate::models::system_log;
use crate::utils::auto_service::prelude::*;

auto_service!(SystemLogService, system_log, {});

#[derive(Debug, Default, QueryFilters, Deserialize)]
pub struct SystemLogSearchParams {
    #[filter(with = system_log::Column::LogType.eq)]
    pub log_type: Option<String>,
}

impl SystemLogService {
    pub async fn page<M>(
        txn: &DatabaseTransaction,
        page_params: impl Into<PageParams>,
        search_params: impl Into<SystemLogSearchParams>,
    ) -> Result<PageData<M>>
    where
        M: FromQueryResult + Send + Sync,
    {
        Self::_page::<M, SystemLogSearchParams>(txn, page_params, search_params).await
    }
}
