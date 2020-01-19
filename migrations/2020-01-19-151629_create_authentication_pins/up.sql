-- Your SQL goes here
CREATE TABLE authentication_pins (
	id SERIAL PRIMARY KEY,
	user_id INT NOT NULL REFERENCES users(id),
	pin TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL
)
