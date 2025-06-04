-- in production, many fields below should have NOT NULL
CREATE TABLE users(
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) UNIQUE NOT NULL,
	password VARCHAR(255) NOT NULL,
	occupation VARCHAR(255),
	email VARCHAR(255) UNIQUE,
	phone VARCHAR(32) UNIQUE,
	priority INT,
	balance Numeric(26, 18),
	updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Insert initial data
INSERT INTO users(name, password, email ) VALUES
('JohnDoe','john','john@x.com'),
('JaneDoe','jane','jane@x.com'),
('JimmDoe','jimm','jimm@x.com');