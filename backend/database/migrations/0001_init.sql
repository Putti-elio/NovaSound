CREATE TABLE IF NOT EXISTS artists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    image_path TEXT
);

CREATE TABLE IF NOT EXISTS albums (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    total_duration INTEGER DEFAULT 0,
    release_date BIGINT,
    artist_id TEXT NOT NULL,
    image_path TEXT,
    album_type TEXT DEFAULT 'ALBUM',
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS songs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    duration INTEGER,
    artist_id TEXT NOT NULL,
    album_id TEXT,
    release_date BIGINT,
    track_number INTEGER,
    image_path TEXT,
    FOREIGN KEY (artist_id) REFERENCES artists(id),
    FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_songs_album_id ON songs(album_id);
CREATE INDEX IF NOT EXISTS idx_songs_artist_id ON songs(artist_id);
