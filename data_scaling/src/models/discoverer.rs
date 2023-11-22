// src/models/discoverer.use relm4::{gtk, SimpleComponent, ComponentSender, ComponentParts};
//

#[derive(Debug)]
pub struct Discoverer {
    pub id: Option<u64>,
    pub last_name: String,
    pub first_name: String,
    pub description: String,
}
