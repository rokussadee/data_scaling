//use crate::db_models::{Flora};
use crate::models::flora::Flora;
use actix::Message;
use diesel::QueryResult;
use chrono::NaiveDate;
use crate::models::flora::FloraType;

//#[derive(Message)]
//pub struct FetchFlora;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Flora>>")]
pub struct FetchFlora{
  pub id: u64,
}


#[derive(Message)]
#[rtype(result = "QueryResult<Flora>")]
pub struct CreateFlora {
    pub common_name: String,
    pub latin_name: String,
    pub blossom_season: String,
    pub planting_date: String,
    pub date_of_discovery: NaiveDate,
    pub discoverer_id: Option<u64>,
    pub flora_type: FloraType,
}
