use crate::schema::flora;
use diesel::Insertable;
use serde::Serialize;
use chrono::NaiveDate;
use crate::models::flora::FloraType;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=flora)]
pub struct NewFlora{
    pub id: Option<u64>,
    pub common_name: String,
    pub latin_name: String,
    pub blossom_season: String,
    pub planting_date: String,
    pub date_of_discovery: Option<NaiveDate>,
    pub discoverer_id: Option<u64>,
    pub flora_type: FloraType,
}
