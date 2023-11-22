-- Your SQL goes here
CREATE TABLE flora (
  id serial PRIMARY KEY,
  common_name varchar NOT NULL,
  latin_name varchar NOT NULL,
  blossom_season varchar NOT NULL,
  planting_date varchar NOT NULL,
  date_of_discovery date NULL,
  discoverer_id bigserial NULL,
  flora_type text NOT NULL
);
