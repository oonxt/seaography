use sea_orm::{EntityTrait, QuerySelect, Select};

/// used to parse order input object and apply it to statement
pub fn apply_distinct_on<T>(
    stmt: Select<T>,
    distinct_on: Vec<T::Column>,
) -> Select<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    stmt.distinct_on(distinct_on)
}
