pub mod models;
pub mod prelude;

#[macro_export]
macro_rules! auto_service {
    ($service_name:ident, $module:ident) => {
        #[derive(Debug)]
        pub struct $service_name {}

        impl $service_name {
            #[allow(unused)]
            pub async fn get_by_id<M>(db: &DatabaseTransaction, id: i32) -> Result<M>
            where
                M: FromQueryResult,
            {
                let mut select = $module::Entity::find();
                select = select.filter($module::Column::Id.eq(id));

                let res = match select.into_model::<M>().one(db).await? {
                    Some(m) => m,
                    None => return Err(anyhow::anyhow!("数据不存在")),
                };

                Ok(res)
            }

            #[allow(unused)]
            pub async fn add_and_fetch(
                txn: &DatabaseTransaction,
                params: impl TryInto<$module::ActiveModel, Error: Into<anyhow::Error>>,
            ) -> Result<$module::Model> {
                let user: $module::ActiveModel = params.try_into().map_err(Into::into)?;
                let re = user.insert(txn).await?;
                Ok(re)
            }

            #[allow(unused)]
            pub async fn add(
                txn: &DatabaseTransaction,
                params: impl TryInto<$module::ActiveModel, Error: Into<anyhow::Error>>,
            ) -> Result<()> {
                Self::add_and_fetch(txn, params).await.map(|_| ())
            }

            #[allow(unused)]
            pub async fn add_and_fetch_into<M>(
                txn: &DatabaseTransaction,
                params: impl TryInto<$module::ActiveModel, Error: Into<anyhow::Error>>,
            ) -> Result<M>
            where
                M: FromQueryResult,
            {
                let re: $module::Model = Self::add_and_fetch(txn, params).await?;
                let re = Self::get_by_id::<M>(txn, re.id).await?;
                Ok(re)
            }

            #[allow(unused)]
            pub async fn _page<M, F>(
                txn: &DatabaseTransaction,
                page_params: impl TryInto<PageParams, Error: Into<anyhow::Error>>,
                search_params: impl TryInto<F, Error: Into<anyhow::Error>>,
            ) -> Result<PageData<M>>
            where
                M: FromQueryResult + Send + Sync,
                F: QueryFiltersTrait,
            {
                let page_params: PageParams = page_params.try_into().map_err(Into::into)?;
                let search_params: F = search_params.try_into().map_err(Into::into)?;

                let mut select = $module::Entity::find();
                select = search_params.apply_filters(select);

                // 获取全部数据条数

                let paginator = select
                    .order_by_asc($module::Column::Id)
                    .into_model::<M>()
                    .paginate(txn, page_params.page_size);
                let ItemsAndPagesNumber {
                    number_of_items: total,
                    number_of_pages: total_pages,
                } = paginator.num_items_and_pages().await?;

                let list = paginator.fetch_page(page_params.page_num - 1).await?;

                info!("total: {}, total_pages: {}", total, total_pages);

                Ok(PageData {
                    list,
                    total,
                    total_pages,
                    page_num: page_params.page_num,
                })
            }
        }
    };
}

#[cfg(test)]
mod tests {}
