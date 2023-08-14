CREATE TABLE IF NOT EXISTS apods 
(
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL UNIQUE,
    explanation TEXT NOT NULL,
    hdurl TEXT,
    media_type TEXT NOT NULL,
    service_version TEXT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    upvotes INTEGER,
    downvotes INT DEFAULT 0
)
