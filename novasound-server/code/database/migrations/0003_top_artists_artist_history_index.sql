-- Fetch all-history first/last play for ranked artists without grouping lifetime events.
CREATE INDEX top_artists_events_artist_history
ON top_artists_events (user_id, provider, artist_id, ended_at);
