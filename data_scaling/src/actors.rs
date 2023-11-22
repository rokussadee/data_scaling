use crate::models::flora::{Flora};
use crate::db_utils::DbActor;
use crate::schema::flora::dsl::*;
use crate::messages::{CreateFlora, FetchFlora};

use crate::insertables::NewFlora;
use actix::Handler;
use diesel::{self, prelude::*};


impl Handler<CreateFlora> for DbActor {
  type Result = QueryResult<Flora>;

  fn handle(&mut self, msg: CreateFlora, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Create User Article: Unable to establish connection");

    let new_flora = { NewFlora{
        id: msg.id?,
        common_name: msg.common_name, 
        latin_name: msg.latin_name, 
        blossom_season: msg.blossom_season, 
        planting_date: msg.planting_date, 
        date_of_discovery: msg.date_of_discovery?,
        discoverer_id: msg.discoverer_id,
        flora_type: msg.flora_type 

    }};

    diesel::insert_into(flora)
      .values(new_flora)
      .returning((
        id,
        common_name,
        latin_name,
        blossom_season,
        planting_date,
        date_of_discovery.nullable(),
        discoverer_id.nullable(),
        flora_type

      ))
      .get_result::<Flora>(&mut conn)
  }
}
