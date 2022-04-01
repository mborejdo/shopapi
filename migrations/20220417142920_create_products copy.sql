CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  price bigint NOT NULL,
  origin TEXT NOT NULL,
  cultivar TEXT NOT NULL,
  images TEXT NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);