-- Your SQL goes here
CREATE TABLE plants (
  flora_id serial PRIMARY KEY REFERENCES floras(flora_id) ON DELETE CASCADE,
  leaf_type text NOT NULL,
  growth_habit text NOT NULL,
  photosynthetic_mechanism text NOT NULL,
  adaptations text NOT NULL,
  -- Add other fields specific to Plant
);

