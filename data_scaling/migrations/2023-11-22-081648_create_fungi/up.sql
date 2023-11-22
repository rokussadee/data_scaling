-- Your SQL goes here
CREATE TABLE fungi (
  flora_id serial PRIMARY KEY REFERENCES floras(flora_id) ON DELETE CASCADE,
  reproduction_method text NOT NULL,
  mycorrhizal_associations text NOT NULL,
  edibility text NOT NULL,
  structure text NOT NULL,
  -- Add other fields specific to Fungus
);

