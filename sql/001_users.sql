CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255),
  password_salt VARCHAR(255),
  handle VARCHAR(255) NOT NULL UNIQUE,
  google_sub VARCHAR(255) UNIQUE,
  "role" VARCHAR(255) NOT NULL DEFAULT 'user',

  gender VARCHAR(255),
  date_of_birth DATE,
  sexuality VARCHAR(255),

  ll_ipaddress VARCHAR(255),
  ll_longitude FLOAT,
  ll_latitude FLOAT,
  ll_continent VARCHAR(255),
  ll_country VARCHAR(255),
  ll_region VARCHAR(255),
  ll_city VARCHAR(255),
  ll_postal_code VARCHAR(255),
  ll_region_code VARCHAR(255),
  ll_timezone VARCHAR(255),
  ll_as_organization VARCHAR(255),
  ll_colocation VARCHAR(255),
  ll_asn VARCHAR(255),

  banned BOOLEAN NOT NULL DEFAULT FALSE,
  ban_reason TEXT,
  banned_at TIMESTAMP,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_handle ON users (handle);

-- Create trigger function to automatically update the updated_at column
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach the trigger to users table
CREATE TRIGGER trigger_update_users_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

CREATE TABLE IF NOT EXISTS users_preferences (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category_id INT NOT NULL,
    value FLOAT NOT NULL
);
