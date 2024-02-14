CREATE TABLE reviews (
	id SERIAL PRIMARY KEY,
	copy TEXT,
	movie_id INTEGER NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	score NUMERIC,
	out_of NUMERIC DEFAULT 10,
	unit VARCHAR(255) DEFAULT 'stars',
	CONSTRAINT movie
		FOREIGN KEY(movie_id)
		REFERENCES movies(id)
)