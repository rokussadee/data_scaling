-- Your SQL goes here
CREATE TABLE herbs (
  flora_id serial PRIMARY KEY REFERENCES floras(flora_id) ON DELETE CASCADE,
  lifespan text NOT NULL,
  growth_habit text NOT NULL,
  culinary_or_medicinal_use text NOT NULL,
  aroma_and_flavor text NOT NULL,
  -- Add other fields specific to Herb
);

