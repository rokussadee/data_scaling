use chrono::NaiveDate;

#[derive(Debug)]
pub struct Flora {
    flora_id: u64,
    common_name: String,
    latin_name: String,
    blossom_season: String,
    planting_date: String,
    date_of_discovery: NaiveDate,
    discoverer_id: u64,
    flora_type: FloraType,
}

#[derive(Debug)]
pub enum FloraType {
    Plant(PlantProperties),
    Fungus(FungusProperties),
    Flower(FlowerProperties),
    Tree(TreeProperties),
    Herb(HerbProperties),
}

#[derive(Debug)]
struct PlantProperties {
    leaf_type: String,
    growth_habit: String,
    photosynthetic_mechanism: String,
    adaptations: String,   
}

#[derive(Debug)]
struct FungusProperties {
    reproduction_method: String,
    mycorrhizal_associations: String,
    edibility: String,
    structure: String,
}

#[derive(Debug)]
struct HerbProperties {
    lifespan: String,
    growth_habit: String,
    culinary_or_medicinal_use: String,
    aroma_and_flavor: String,
}

#[derive(Debug)]
struct TreeProperties {
    height: f64,
    trunk_diameter: f64,
    canopy_shape: String,
    wood_type: String,
    nut_or_fruit_type: String,
}

#[derive(Debug)]
pub struct FlowerProperties {
    petal_count: u32,
    fragrance: String,
    flowering_season: String,
    pollination_method: String,
    flower_shape: String,
}
