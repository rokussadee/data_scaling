use actix::{Actor, SyncContext, Addr, Message};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
    prelude::*,
};
use std::io::{Error, ErrorKind};

use crate::{messages::CreateFlora, models::flora::PlantProperties};
use crate::schema::{*};
use crate::dto::FloraResponse;
use crate::models::flora::{Flora, FloraType};
use crate::dto::*;


// Import other necessary modules and structs

pub struct AppState {
    pub db: Addr<DbActor>,
}

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl actix::Handler<CreateFlora> for DbActor {
    type Result = Result<FloraResponse, Error>;

    fn handle(&mut self, msg: CreateFlora, _: &mut Self::Context) -> Self::Result {
        // Get a connection from the pool
        let conn = self.0.get().map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

        // Create a new flora record in the database
//        let new_flora = NewFlora {
//            common_name: msg.common_name.to_string(),
//            latin_name: msg.latin_name.to_string(),
//            blossom_season: msg.blossom_season.to_string(),
//            planting_date: msg.planting_date.to_string(),
//            date_of_discovery: msg.date_of_discovery,
//            discoverer_id: msg.discoverer_id,
//            flora_type: msg.flora_type.to_string(),
//        };
//
        let new_flora = msg.into();

        let inserted_flora: Flora = diesel::insert_into(crate::schema::flora::table)
            .values(&new_flora)
            .get_result(&conn)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

        // Return the inserted flora as a FloraResponse
       // Ok(FloraResponse {
       //     id: inserted_flora.id,
       //     common_name: inserted_flora.common_name.clone(),
       //     latin_name: inserted_flora.latin_name.clone(),
       //     blossom_season: inserted_flora.blossom_season.clone(),
       //     planting_date: inserted_flora.planting_date.clone(),
       //     date_of_discovery: inserted_flora.date_of_discovery,
       //     discoverer_id: inserted_flora.discoverer_id,
       //     flora_type: map_flora_type(&inserted_flora.flora_type),
       // });
        Ok(inserted_flora.into());
        Err(Error::new(ErrorKind::Other, "FloraResponse could not be created."))
    }
}

fn map_flora_type(flora_type: &FloraType) -> FloraTypeResponse {
    match flora_type {
        FloraType::Plant(plant_properties) => FloraTypeResponse::Plant(PlantPropertiesResponse {
            leaf_type: plant_properties.leaf_type.clone(),
            growth_habit: plant_properties.growth_habit.clone(),
            photosynthetic_mechanism: plant_properties.photosynthetic_mechanism.clone(),
            adaptations: plant_properties.adaptations.clone(),
        }),
        FloraType::Fungus(fungus_properties) => FloraTypeResponse::Fungus(FungusPropertiesResponse {
            reproduction_method: fungus_properties.reproduction_method.clone(),
            mycorrhizal_associations: fungus_properties.mycorrhizal_associations.clone(),
            edibility: fungus_properties.edibility.clone(),
            structure: fungus_properties.structure.clone(),
        }),
        FloraType::Flower(flower_properties) => FloraTypeResponse::Flower(FlowerPropertiesResponse {
            petal_count: flower_properties.petal_count,
            fragrance: flower_properties.fragrance.clone(),
            flowering_season: flower_properties.flowering_season.clone(),
            pollination_method: flower_properties.pollination_method.clone(),
            flower_shape: flower_properties.flower_shape.clone(),
        }),
        FloraType::Tree(tree_properties) => FloraTypeResponse::Tree(TreePropertiesResponse {
            height: tree_properties.height,
            trunk_diameter: tree_properties.trunk_diameter,
            canopy_shape: tree_properties.canopy_shape.clone(),
            wood_type: tree_properties.wood_type.clone(),
            nut_or_fruit_type: tree_properties.nut_or_fruit_type.clone(),
        }),
        FloraType::Herb(herb_properties) => FloraTypeResponse::Herb(HerbPropertiesResponse {
            lifespan: herb_properties.lifespan.clone(),
            growth_habit: herb_properties.growth_habit.clone(),
            culinary_or_medicinal_use: herb_properties.culinary_or_medicinal_use.clone(),
            aroma_and_flavor: herb_properties.aroma_and_flavor.clone(),
        }),
    }
}

pub fn get_all_flora_with_properties(conn: &PgConnection) {
    flora::table
        .left_join(plants::table.on(flora::id.eq(plants::flora_id)))
        .left_join(fungis::table.on(flora::id.eq(fungis::flora_id)))
        .left_join(flowers::table.on(flora::id.eq(flowers::flora_id)))
        .left_join(trees::table.on(flora::id.eq(trees::flora_id)))
        .left_join(herbs::table.on(flora::id.eq(herbs::flora_id)))
        .select((flora::all_columns,
                 plants::all_columns.nullable(),
                 fungis::all_columns.nullable(),
                 flowers::all_columns.nullable(),
                 trees::all_columns.nullable(),
                 herbs::all_columns.nullable()))
        .load(conn)
}

pub fn get_flora_by_id(flora_id: i64, conn: &PgConnection) -> Result<Flora, diesel::result::Error> {
    flora::table
        .filter(flora::id.eq(flora_id))
        .first(conn)
        .load(conn)
}

pub fn get_flora_with_properties(flora_id: u64, conn: &PgConnection) {
    flora::table
        .filter(flora::id.eq(flora_id))
        .left_join(plants::table)
        .select((flora::all_columns, plants::all_columns.nullable()))
        .first(conn)
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
  let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
  Pool::builder().build(manager).expect("Error building a connection pool")
}
