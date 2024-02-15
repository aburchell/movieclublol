use askama::Template;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// id SERIAL PRIMARY KEY,
// copy TEXT,
// movie_id INTEGER NOT NULL,
// created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
// score NUMERIC,
// out_of NUMERIC DEFAULT 10,
// unit VARCHAR(255) DEFAULT 'stars',
// CONSTRAINT movie
// 	FOREIGN KEY(movie_id)
// 	REFERENCES movies(id)

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Review {
    pub id: i32,
    pub movie_id: i32,
    pub author: String,
    pub copy: String,
    pub created_at: DateTime<Local>,
    pub score: Option<i32>,
    pub out_of: i32,
    pub unit: String,
}
