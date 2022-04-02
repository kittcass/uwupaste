CREATE TYPE paste_type AS ENUM ('text', 'file');

CREATE TABLE paste (
	id          UUID PRIMARY KEY,
	type        paste_type NOT NULL,
	value       TEXT NOT NULL,
	timestamp   TIMESTAMP WITH TIME ZONE,
	expiration  TIMESTAMP WITH TIME ZONE
);
