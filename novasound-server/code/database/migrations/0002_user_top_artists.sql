-- No shared/global user. Authentication will supply these NovaSound UUIDs later.
CREATE TABLE top_artists_owners (
    user_id UUID NOT NULL,
    provider TEXT NOT NULL,
    PRIMARY KEY (user_id, provider)
);

CREATE TABLE top_artists_identities (
    user_id UUID NOT NULL,
    provider TEXT NOT NULL,
    id UUID NOT NULL,
    artist JSONB NOT NULL,
    PRIMARY KEY (user_id, provider, id),
    FOREIGN KEY (user_id, provider) REFERENCES top_artists_owners ON DELETE CASCADE
);

CREATE TABLE top_artists_events (
    user_id UUID NOT NULL,
    provider TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    artist_id UUID NOT NULL,
    ended_at TIMESTAMPTZ NOT NULL,
    played_ms BIGINT NOT NULL CHECK (played_ms BETWEEN 0 AND 2147483647),
    PRIMARY KEY (user_id, provider, fingerprint),
    FOREIGN KEY (user_id, provider, artist_id) REFERENCES top_artists_identities ON DELETE CASCADE
);
CREATE INDEX top_artists_events_period ON top_artists_events (user_id, provider, ended_at);

CREATE TABLE top_artists_native_snapshots (
    user_id UUID NOT NULL,
    provider TEXT NOT NULL,
    period TEXT NOT NULL CHECK (period IN ('short_term', 'medium_term', 'long_term')),
    observed_at TIMESTAMPTZ NOT NULL,
    ranking JSONB NOT NULL,
    PRIMARY KEY (user_id, provider, period),
    FOREIGN KEY (user_id, provider) REFERENCES top_artists_owners ON DELETE CASCADE
);
