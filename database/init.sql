-- Create songs table
CREATE TABLE IF NOT EXISTS songs (
    id SERIAL PRIMARY KEY,
    uri TEXT NOT NULL
);

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    spotifytoken JSONB,
    liked_songs INTEGER[],
    disliked_songs INTEGER[]
);

-- Junction tables for many-to-many relationship (if needed)
CREATE TABLE IF NOT EXISTS user_liked_songs (
    user_id INTEGER REFERENCES users(id),
    song_id INTEGER REFERENCES songs(id),
    PRIMARY KEY (user_id, song_id)
);

CREATE TABLE IF NOT EXISTS user_disliked_songs (
    user_id INTEGER REFERENCES users(id),
    song_id INTEGER REFERENCES songs(id),
    PRIMARY KEY (user_id, song_id)
);
