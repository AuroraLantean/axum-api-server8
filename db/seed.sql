-- in production, many fields below should have NOT NULL
CREATE TABLE users(
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) UNIQUE NOT NULL,
	password VARCHAR(255) NOT NULL,
	email VARCHAR(255) UNIQUE NOT NULL,
	occupation VARCHAR(255),
	phone VARCHAR(32),
	level INT NOT NULL CHECK (level >= 0) DEFAULT 0,
	balance Numeric(26, 9) NOT NULL CHECK (balance >= 0) DEFAULT 0.0,
	updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Insert initial data
INSERT INTO users(name, password, email ) VALUES
('John Doe','john','john@x.com'),
('Jane Doe','jane','jane@x.com');