CREATE TABLE IF NOT EXISTS artists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    image_path TEXT
);

CREATE TABLE IF NOT EXISTS albums (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    -- Stored as PostgreSQL INTEGER for Clorinde; Rust treats durations as u32.
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
    -- Stored as PostgreSQL INTEGER for Clorinde; Rust treats durations as u32.
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

CREATE TABLE IF NOT EXISTS top_artists_owners (
    user_id UUID NOT NULL,
    provider TEXT NOT NULL,
    PRIMARY KEY (user_id, provider)
);
CREATE TABLE IF NOT EXISTS top_artists_identities (
    user_id UUID NOT NULL,
    provider TEXT NOT NULL,
    id UUID NOT NULL,
    artist JSONB NOT NULL,
    PRIMARY KEY (user_id, provider, id),
    FOREIGN KEY (user_id, provider) REFERENCES top_artists_owners ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS top_artists_events (
    user_id UUID NOT NULL,
    provider TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    artist_id UUID NOT NULL,
    ended_at TIMESTAMPTZ NOT NULL,
    played_ms BIGINT NOT NULL CHECK (played_ms BETWEEN 0 AND 2147483647),
    PRIMARY KEY (user_id, provider, fingerprint),
    FOREIGN KEY (user_id, provider, artist_id) REFERENCES top_artists_identities ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS top_artists_events_period ON top_artists_events (user_id, provider, ended_at);
CREATE INDEX IF NOT EXISTS top_artists_events_artist_history ON top_artists_events (user_id, provider, artist_id, ended_at);
CREATE TABLE IF NOT EXISTS top_artists_native_snapshots (
    user_id UUID NOT NULL,
    provider TEXT NOT NULL,
    period TEXT NOT NULL CHECK (period IN ('short_term', 'medium_term', 'long_term')),
    observed_at TIMESTAMPTZ NOT NULL,
    ranking JSONB NOT NULL,
    PRIMARY KEY (user_id, provider, period),
    FOREIGN KEY (user_id, provider) REFERENCES top_artists_owners ON DELETE CASCADE
);
