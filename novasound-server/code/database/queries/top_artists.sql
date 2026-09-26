--! ensure_owner
INSERT INTO top_artists_owners (user_id, provider) VALUES (:user_id, :provider)
ON CONFLICT DO NOTHING;

--! lock_owner
SELECT user_id FROM top_artists_owners
WHERE user_id = :user_id AND provider = :provider FOR UPDATE;

--! upsert_identities
INSERT INTO top_artists_identities (user_id, provider, id, artist)
SELECT :user_id, :provider, x.id, x.artist
FROM jsonb_to_recordset(:artists::jsonb) AS x(id UUID, artist JSONB)
ON CONFLICT (user_id, provider, id) DO UPDATE SET artist = EXCLUDED.artist;

--! insert_events
WITH inserted AS (
    INSERT INTO top_artists_events (user_id, provider, fingerprint, artist_id, ended_at, played_ms)
    SELECT :user_id, :provider, x.fingerprint, x.artist_id, x.ended_at, x.played_ms
    FROM jsonb_to_recordset(:events::jsonb) AS x(fingerprint TEXT, artist_id UUID, ended_at TIMESTAMPTZ, played_ms BIGINT)
    ON CONFLICT DO NOTHING
    RETURNING fingerprint
)
SELECT COUNT(*) AS inserted FROM inserted;

--! get_snapshot
SELECT ranking FROM top_artists_native_snapshots
WHERE user_id = :user_id AND provider = :provider AND period = :period;

--! save_snapshot
INSERT INTO top_artists_native_snapshots (user_id, provider, period, observed_at, ranking)
VALUES (:user_id, :provider, :period, :observed_at, :ranking)
ON CONFLICT (user_id, provider, period)
DO UPDATE SET observed_at = EXCLUDED.observed_at, ranking = EXCLUDED.ranking;

--! history_coverage : (first_event?, last_event?)
SELECT MIN(ended_at) AS first_event, MAX(ended_at) AS last_event
FROM top_artists_events WHERE user_id = :user_id AND provider = :provider;

--! custom_ranking : (rank, artist, plays, played_ms, streams, first_play, last_play, rank_change?)
WITH totals AS (
    SELECT artist_id,
        COUNT(*) FILTER (WHERE ended_at >= :start_at AND ended_at < :end_at) AS plays,
        COALESCE(SUM(played_ms) FILTER (WHERE ended_at >= :start_at AND ended_at < :end_at), 0)::bigint AS played_ms,
        COUNT(*) FILTER (WHERE ended_at >= :start_at AND ended_at < :end_at AND played_ms >= 30000) AS streams,
        COUNT(*) FILTER (WHERE ended_at >= :previous_start AND ended_at < :start_at) AS previous_plays
    FROM top_artists_events
    WHERE user_id = :user_id AND provider = :provider
        AND ended_at >= :previous_start AND ended_at < :end_at
    GROUP BY artist_id
), current_ranks AS (
    SELECT *, ROW_NUMBER() OVER (ORDER BY plays DESC, artist_id) AS rank FROM totals WHERE plays > 0
), previous_ranks AS (
    SELECT artist_id, ROW_NUMBER() OVER (ORDER BY previous_plays DESC, artist_id) AS rank FROM totals WHERE previous_plays > 0
), current_page AS MATERIALIZED (
    SELECT * FROM current_ranks ORDER BY rank LIMIT :result_limit
)
SELECT c.rank, i.artist, c.plays, c.played_ms, c.streams,
    first_event.ended_at AS first_play, last_event.ended_at AS last_play,
    p.rank - c.rank AS rank_change
FROM current_page c
JOIN top_artists_identities i ON i.user_id = :user_id AND i.provider = :provider AND i.id = c.artist_id
LEFT JOIN previous_ranks p ON p.artist_id = c.artist_id
JOIN LATERAL (
    SELECT ended_at FROM top_artists_events
    WHERE user_id = :user_id AND provider = :provider AND artist_id = c.artist_id
    ORDER BY ended_at ASC LIMIT 1
) first_event ON TRUE
JOIN LATERAL (
    SELECT ended_at FROM top_artists_events
    WHERE user_id = :user_id AND provider = :provider AND artist_id = c.artist_id
    ORDER BY ended_at DESC LIMIT 1
) last_event ON TRUE
ORDER BY c.rank;
