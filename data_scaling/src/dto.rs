// src/models/flora_response.rs
use chrono::NaiveDate;
use derive_more::Display;

use crate::models::flora::FloraType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateFloraRequest {
    pub common_name: String,
    pub latin_name: String,
    pub blossom_season: String,
    pub planting_date: String,
    pub date_of_discovery: NaiveDate,
    pub discoverer_id: Option<u64>,
    pub flora_type: FloraType,
}

#[derive(Debug, Serialize)]
pub struct FloraResponse {
    pub id: Option<u64>,
    pub common_name: String,
    pub latin_name: String,
    pub blossom_season: String,
    pub planting_date: String,
    pub date_of_discovery: NaiveDate,
    pub discoverer_id: Option<u64>,
    pub flora_type: FloraTypeResponse,
}

#[derive(Debug, Serialize, Display, PartialEq)]
pub enum FloraTypeResponse {
    Plant(PlantPropertiesResponse),
    Fungus(FungusPropertiesResponse),
    Flower(FlowerPropertiesResponse),
    Tree(TreePropertiesResponse),
    Herb(HerbPropertiesResponse),
}

#[derive(Debug, Serialize)]
pub struct PlantPropertiesResponse {
    pub leaf_type: String,
    pub growth_habit: String,
    pub photosynthetic_mechanism: String,
    pub adaptations: String,
}

#[derive(Debug, Serialize)]
pub struct FungusPropertiesResponse {
    pub reproduction_method: String,
    pub mycorrhizal_associations: String,
    pub edibility: String,
    pub structure: String,
}

#[derive(Debug, Serialize)]
pub struct HerbPropertiesResponse {
    pub lifespan: String,
    pub growth_habit: String,
    pub culinary_or_medicinal_use: String,
    pub aroma_and_flavor: String,
}

#[derive(Debug, Serialize)]
pub struct TreePropertiesResponse {
    pub height: f64,
    pub trunk_diameter: f64,
    pub canopy_shape: String,
    pub wood_type: String,
    pub nut_or_fruit_type: String,
}

#[derive(Debug, Serialize)]
pub struct FlowerPropertiesResponse {
    pub petal_count: u32,
    pub fragrance: String,
    pub flowering_season: String,
    pub pollination_method: String,
    pub flower_shape: String,
}

