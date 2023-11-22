-- Your SQL goes here
CREATE TABLE flowers (
  flora_id serial PRIMARY KEY REFERENCES floras(flora_id) ON DELETE CASCADE,
  petal_count int NOT NULL,
  fragrance text NOT NULL,
  flowering_season text NOT NULL,
  pollination_method text NOT NULL,
  flower_shape text NOT NULL,
  -- Add other fields specific to Flower
);

