// @generated automatically by Diesel CLI.
use diesel::{
    table,
    sql_types::{Nullable, Integer },
};
use crate::serde::de::Unexpected::Unsigned;

table! {
    flora (id) {
        id -> Nullable<Unsigned<Integer>>,
        common_name -> Varchar,
        latin_name -> Varchar,
        blossom_season -> Varchar,
        planting_date -> Varchar,
        date_of_discovery -> Nullable<Date>,
        discoverer_id -> Nullable<Unsigned<Integer>>,
        flora_type -> Text,
    }
}

table! {
    plants (flora_id) {
        flora_id -> Nullable<Unsigned<Integer>>,
        leaf_type -> Text,
        growth_habit -> Text,
        photosynthetic_mechanism -> Text,
        adaptations -> Text,
        // Add other fields specific to Plant
    }
}

table! {
    fungis (flora_id) {
        flora_id -> Nullable<Unsigned<Integer>>,
        reproduction_method -> Text,
        mycorrhizal_associations -> Text,
        edibility -> Text,
        structure -> Text,
        // Add other fields specific to Fungus
    }
}

table! {
    flowers (flora_id) {
        flora_id -> Nullable<Unsigned<Integer>>,
        petal_count -> Int4,
        fragrance -> Text,
        flowering_season -> Text,
        pollination_method -> Text,
        flower_shape -> Text,
        // Add other fields specific to Flower
    }
}

table! {
    trees (flora_id) {
        flora_id -> Nullable<Unsigned<Integer>>,
        height -> Float8,
        trunk_diameter -> Float8,
        canopy_shape -> Text,
        wood_type -> Text,
        nut_or_fruit_type -> Text,
        // Add other fields specific to Tree
    }
}

table! {
    herbs (flora_id) {
        flora_id -> Nullable<Unsigned<Integer>>,
        lifespan -> Text,
        growth_habit -> Text,
        culinary_or_medicinal_use -> Text,
        aroma_and_flavor -> Text,
        // Add other fields specific to Herb
    }
}

joinable!(plants -> flora (flora_id));
joinable!(fungis -> flora (flora_id));
joinable!(flowers -> flora (flora_id));
joinable!(trees -> flora (flora_id));
joinable!(herbs -> flora (flora_id));

allow_tables_to_appear_in_same_query!(
    flora,
    plants,
    fungis,
    flowers,
    trees,
    herbs,
);
