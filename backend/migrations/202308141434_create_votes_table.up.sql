CREATE TYPE vote_choice AS ENUM ('upvote', 'downvote');

CREATE TABLE votes (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    apod_id INTEGER NOT NULL REFERENCES apods(id),
    vote_type vote_choice NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, apod_id)
);

