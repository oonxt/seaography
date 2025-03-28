use std::str::FromStr;
use async_graphql::dynamic::{Enum, ValueAccessor};
use sea_orm::{EntityTrait, Iterable};

use crate::{BuilderContext, EntityObjectBuilder};

pub struct DistinctOnConfig {
    pub type_name: crate::SimpleNamingFn,
}

impl Default for DistinctOnConfig {
    fn default() -> Self {
        DistinctOnConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{object_name}DistinctOn")
            }),
        }
    }
}

pub struct DistinctOnEnumBuilder {
    pub context: &'static BuilderContext,
}

impl DistinctOnEnumBuilder {
    pub fn type_name(&self, object_name: &str) -> String {
        self.context.distinct_on_input.type_name.as_ref()(object_name)
    }

    /// used to get the OrderInput object of a SeaORM entity
    pub fn to_enum<T>(&self) -> Enum
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };

        let object_name = entity_object_builder.type_name::<T>();
        let name = self.type_name(&object_name);

        T::Column::iter().fold(Enum::new(name), |object, column| {
            object.item(entity_object_builder.column_name::<T>(&column))
        })
    }

    pub fn parse_object<T>(
        &self,
        value: Option<ValueAccessor<'_>>,
    ) -> Vec<T::Column>
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        match value {
            Some(value) => {
                let mut data = Vec::new();

                let distinct_on = value.list().unwrap();

                for distinct_on in distinct_on.iter() {
                    let distinct_on = distinct_on.enum_name().unwrap();

                    let column = T::Column::from_str(distinct_on);

                    if let Ok(column) = column {
                        data.push(column);
                    }
                }
                data
            }
            None => Vec::new(),
        }
    }
}

