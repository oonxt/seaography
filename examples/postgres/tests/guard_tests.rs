use std::collections::BTreeMap;

use async_graphql::{dynamic::*, Response};
use sea_orm::{Database, DatabaseConnection, RelationTrait};
use seaography::{
    async_graphql, lazy_static, Builder, BuilderContext, EntityObjectRelationBuilder,
    EntityObjectViaRelationBuilder, FnGuard, GuardsConfig,
};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let context = BuilderContext::default();
        let mut entity_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        entity_guards.insert("FilmCategory".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
        }));
        let mut field_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        field_guards.insert("Language.lastUpdate".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
        }));
        BuilderContext {
            guards: GuardsConfig {
                entity_guards,
                field_guards,
            },
            ..context
        }
    };
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    let entity_object_relation_builder = EntityObjectRelationBuilder { context: &CONTEXT };
    let entity_object_via_relation_builder = EntityObjectViaRelationBuilder { context: &CONTEXT };
    builder.register_entity::<seaography_postgres_example::entities::film_actor::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::film_actor::Entity, seaography_postgres_example::entities::actor::Entity>(
                "actor",
                seaography_postgres_example::entities::film_actor::Relation::Actor.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::film_actor::Entity, seaography_postgres_example::entities::film::Entity>(
                "film",
                seaography_postgres_example::entities::film_actor::Relation::Film.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::rental::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::rental::Entity, seaography_postgres_example::entities::customer::Entity>(
                "customer",
                seaography_postgres_example::entities::rental::Relation::Customer.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::rental::Entity, seaography_postgres_example::entities::inventory::Entity>(
                "inventory",
                seaography_postgres_example::entities::rental::Relation::Inventory.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::rental::Entity, seaography_postgres_example::entities::payment::Entity>(
                "payment",
                seaography_postgres_example::entities::rental::Relation::Payment.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::rental::Entity, seaography_postgres_example::entities::staff::Entity>(
                "staff",
                seaography_postgres_example::entities::rental::Relation::Staff.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::category::Entity>(vec![
        entity_object_via_relation_builder
            .get_relation::<seaography_postgres_example::entities::category::Entity, seaography_postgres_example::entities::film::Entity>(
                "film",
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::staff::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::staff::Entity, seaography_postgres_example::entities::address::Entity>(
                "address",
                seaography_postgres_example::entities::staff::Relation::Address.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::staff::Entity, seaography_postgres_example::entities::payment::Entity>(
                "payment",
                seaography_postgres_example::entities::staff::Relation::Payment.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::staff::Entity, seaography_postgres_example::entities::rental::Entity>(
                "rental",
                seaography_postgres_example::entities::staff::Relation::Rental.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::staff::Entity, seaography_postgres_example::entities::store::Entity>(
                "store",
                seaography_postgres_example::entities::staff::Relation::Store.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::country::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::country::Entity, seaography_postgres_example::entities::city::Entity>(
                "city",
                seaography_postgres_example::entities::country::Relation::City.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::film::Entity>(vec![
        entity_object_via_relation_builder
            .get_relation::<seaography_postgres_example::entities::film::Entity, seaography_postgres_example::entities::actor::Entity>("actor"),
        entity_object_via_relation_builder
            .get_relation::<seaography_postgres_example::entities::film::Entity, seaography_postgres_example::entities::category::Entity>(
                "category",
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::film::Entity, seaography_postgres_example::entities::inventory::Entity>(
                "inventory",
                seaography_postgres_example::entities::film::Relation::Inventory.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::film::Entity, seaography_postgres_example::entities::language::Entity>(
                "language1",
                seaography_postgres_example::entities::film::Relation::Language1.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::film::Entity, seaography_postgres_example::entities::language::Entity>(
                "language2",
                seaography_postgres_example::entities::film::Relation::Language2.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::actor::Entity>(vec![
        entity_object_via_relation_builder
            .get_relation::<seaography_postgres_example::entities::actor::Entity, seaography_postgres_example::entities::film::Entity>("film"),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::language::Entity>(vec![]);
    builder.register_entity::<seaography_postgres_example::entities::city::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::city::Entity, seaography_postgres_example::entities::address::Entity>(
                "address",
                seaography_postgres_example::entities::city::Relation::Address.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::city::Entity, seaography_postgres_example::entities::country::Entity>(
                "country",
                seaography_postgres_example::entities::city::Relation::Country.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::inventory::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::inventory::Entity, seaography_postgres_example::entities::film::Entity>(
                "film",
                seaography_postgres_example::entities::inventory::Relation::Film.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::inventory::Entity, seaography_postgres_example::entities::rental::Entity>(
                "rental",
                seaography_postgres_example::entities::inventory::Relation::Rental.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::inventory::Entity, seaography_postgres_example::entities::store::Entity>(
                "store",
                seaography_postgres_example::entities::inventory::Relation::Store.def(),
            ),
    ]);
    builder . register_entity :: < seaography_postgres_example:: entities :: film_category :: Entity > (vec ! [entity_object_relation_builder . get_relation :: < seaography_postgres_example:: entities :: film_category :: Entity , seaography_postgres_example:: entities :: category :: Entity > ("category" , seaography_postgres_example:: entities :: film_category :: Relation :: Category . def ()) , entity_object_relation_builder . get_relation :: < seaography_postgres_example:: entities :: film_category :: Entity , seaography_postgres_example:: entities :: film :: Entity > ("film" , seaography_postgres_example:: entities :: film_category :: Relation :: Film . def ())]) ;
    builder.register_entity::<seaography_postgres_example::entities::customer::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::customer::Entity, seaography_postgres_example::entities::address::Entity>(
                "address",
                seaography_postgres_example::entities::customer::Relation::Address.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::customer::Entity, seaography_postgres_example::entities::payment::Entity>(
                "payment",
                seaography_postgres_example::entities::customer::Relation::Payment.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::customer::Entity, seaography_postgres_example::entities::rental::Entity>(
                "rental",
                seaography_postgres_example::entities::customer::Relation::Rental.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::customer::Entity, seaography_postgres_example::entities::store::Entity>(
                "store",
                seaography_postgres_example::entities::customer::Relation::Store.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::store::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::store::Entity, seaography_postgres_example::entities::address::Entity>(
                "address",
                seaography_postgres_example::entities::store::Relation::Address.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::store::Entity, seaography_postgres_example::entities::customer::Entity>(
                "customer",
                seaography_postgres_example::entities::store::Relation::Customer.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::store::Entity, seaography_postgres_example::entities::inventory::Entity>(
                "inventory",
                seaography_postgres_example::entities::store::Relation::Inventory.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::store::Entity, seaography_postgres_example::entities::staff::Entity>(
                "staff",
                seaography_postgres_example::entities::store::Relation::Staff.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::payment::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::payment::Entity, seaography_postgres_example::entities::customer::Entity>(
                "customer",
                seaography_postgres_example::entities::payment::Relation::Customer.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::payment::Entity, seaography_postgres_example::entities::rental::Entity>(
                "rental",
                seaography_postgres_example::entities::payment::Relation::Rental.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::payment::Entity, seaography_postgres_example::entities::staff::Entity>(
                "staff",
                seaography_postgres_example::entities::payment::Relation::Staff.def(),
            ),
    ]);
    builder.register_entity::<seaography_postgres_example::entities::address::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::address::Entity, seaography_postgres_example::entities::city::Entity>(
                "city",
                seaography_postgres_example::entities::address::Relation::City.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::address::Entity, seaography_postgres_example::entities::customer::Entity>(
                "customer",
                seaography_postgres_example::entities::address::Relation::Customer.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::address::Entity, seaography_postgres_example::entities::staff::Entity>(
                "staff",
                seaography_postgres_example::entities::address::Relation::Staff.def(),
            ),
        entity_object_relation_builder
            .get_relation::<seaography_postgres_example::entities::address::Entity, seaography_postgres_example::entities::store::Entity>(
                "store",
                seaography_postgres_example::entities::address::Relation::Store.def(),
            ),
    ]);
    builder.register_enumeration::<seaography_postgres_example::entities::sea_orm_active_enums::MpaaRating>();
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

pub async fn get_schema() -> Schema {
    let database = Database::connect("postgres://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
    let schema = schema(database, None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn entity_guard() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    language(orderBy: { languageId: ASC }) {
                      nodes {
                        languageId
                        name
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "language": {
              "nodes": [
                {
                  "languageId": 1,
                  "name": "English             "
                },
                {
                  "languageId": 2,
                  "name": "Italian             "
                },
                {
                  "languageId": 3,
                  "name": "Japanese            "
                },
                {
                  "languageId": 4,
                  "name": "Mandarin            "
                },
                {
                  "languageId": 5,
                  "name": "French              "
                },
                {
                  "languageId": 6,
                  "name": "German              "
                }
              ]
            }
        }
        "#,
    );

    let response = schema
        .execute(
            r#"
        {
            filmCategory {
              nodes {
                filmId
              }
            }
        }
        "#,
        )
        .await;

    assert_eq!(response.errors.len(), 1);

    assert_eq!(response.errors[0].message, "Entity guard triggered.");
}

#[tokio::test]
async fn field_guard() {
    let schema = get_schema().await;

    let response = schema
        .execute(
            r#"
            {
                language {
                nodes {
                    languageId
                    name
                    lastUpdate
                }
                }
            }
        "#,
        )
        .await;

    assert_eq!(response.errors.len(), 1);

    assert_eq!(response.errors[0].message, "Field guard triggered.");
}
