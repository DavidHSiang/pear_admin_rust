pub trait QueryFiltersTrait {
    fn apply_filters<E: sea_orm::EntityTrait>(
        &self,
        query: sea_orm::query::Select<E>,
    ) -> sea_orm::query::Select<E>;
}
