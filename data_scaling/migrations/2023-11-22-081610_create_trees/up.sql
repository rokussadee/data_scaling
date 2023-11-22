-- Your SQL goes here
CREATE TABLE trees (
  flora_id serial PRIMARY KEY REFERENCES floras(flora_id) ON DELETE CASCADE,
  height double precision NOT NULL,
  trunk_diameter double precision NOT NULL,
  canopy_shape text NOT NULL,
  wood_type text NOT NULL,
  nut_or_fruit_type text NOT NULL,
  -- Add other fields specific to Tree
);

