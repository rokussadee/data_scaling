use std::io::Write;

// src/models/flora.rs
use chrono::NaiveDate;
use derive_more::Display;
use diesel::{
    backend::{Backend, RawValue},
    sql_types::Text,
    pg::{Pg, PgValue},
    serialize::{ToSql, Output, Result as SerializationResult},
    deserialize::{self, FromSql, Result as DeserializationResult}

};

use crate::schema::flora;
use crate::models::discoverer::Discoverer;


// #[allow(clippy::all)]

#[derive(Debug, Queryable, Insertable, Identifiable, Associations)]
#[diesel(table_name=flora)]
#[diesel(belongs_to(Discoverer))]
pub struct Flora {
    pub id: Option<u64>,
    pub common_name: String,
    pub latin_name: String,
    pub blossom_season: String,
    pub planting_date: String,
    pub date_of_discovery: NaiveDate,
    pub discoverer_id: Option<u64>,
    pub flora_type: FloraType,
}

#[derive(Debug, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum FloraType {
    Plant(PlantProperties),
    Fungus(FungusProperties),
    Flower(FlowerProperties),
    Tree(TreeProperties),
    Herb(HerbProperties),
}

impl ToSql<Text, Pg> for FloraType {
    fn to_sql<'a>(&'a self, out: &mut Output<'a, '_, Pg>) -> SerializationResult {
//        <String as ToSql<Text, Pg>>::to_sql(&self.to_string(), out)
//        <String as ToSql<Text, Pg>>::to_sql(&format!("{}", self), out)
        let type_str = match self {
            FloraType::Plant(_) => "Plant",
            FloraType::Fungus(_) => "Fungus",
            FloraType::Flower(_) => "Flower",
            FloraType::Tree(_) => "Tree",
            FloraType::Herb(_) => "Herb",
        };
        ToSql::<Text, Pg>::to_sql(&type_str, out)

    }
}


//impl FromSql<Text, Pg> for FloraType {
//    fn from_sql(bytes: PgValue<'_>) -> DeserializationResult<Self> {
//    //fn from_sql(bytes: Option<&[u8]>) -> DeserializationResult<Self> {
//        let s = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
//        match s.as_str() {
//            "Plant" => Ok(FloraType::Plant),
//            "Fungus" => Ok(FloraType::Fungus),
//            "Flower" => Ok(FloraType::Flower),
//            "Tree" => Ok(FloraType::Tree),
//            "Herb" => Ok(FloraType::Herb),
//            _ => Err("Invalid value for FloraType".into()),
//        }
//    }
//}

#[derive(Debug, PartialEq, AsExpression)]
#[diesel(sql_type = Text)]
pub struct PlantProperties {
    pub leaf_type: String,
    pub growth_habit: String,
    pub photosynthetic_mechanism: String,
    pub adaptations: String,
}

#[derive(Debug, PartialEq, AsExpression)]
#[diesel(sql_type = Text)]
pub struct FungusProperties {
    pub reproduction_method: String,
    pub mycorrhizal_associations: String,
    pub edibility: String,
    pub structure: String,
}

#[derive(Debug, PartialEq, AsExpression)]
#[diesel(sql_type = Text)]
pub struct HerbProperties {
    pub lifespan: String,
    pub growth_habit: String,
    pub culinary_or_medicinal_use: String,
    pub aroma_and_flavor: String,
}

#[derive(Debug, PartialEq , AsExpression)]
#[diesel(sql_type = Text)]
pub struct TreeProperties {
    pub height: f64,
    pub trunk_diameter: f64,
    pub canopy_shape: String,
    pub wood_type: String,
    pub nut_or_fruit_type: String,
}

#[derive(Debug, PartialEq , AsExpression)]
#[diesel(sql_type = Text)]
pub struct FlowerProperties {
    pub petal_count: u32,
    pub fragrance: String,
    pub flowering_season: String,
    pub pollination_method: String,
    pub flower_shape: String,
}

