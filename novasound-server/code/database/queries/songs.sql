--: Song(id, name, duration, artist_id, album_id?, release_date?, track_number?, image_path?)

--! get_all_songs : Song
SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path
FROM songs ORDER BY track_number;

--! get_song_by_id : Song
SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path
FROM songs WHERE id = :id;

--! get_songs_by_artist : Song
SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path
FROM songs WHERE artist_id = :artist_id ORDER BY track_number;

--! get_songs_by_album : Song
SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path
FROM songs WHERE album_id = :album_id ORDER BY track_number;

--! check_song_by_id
SELECT 1 FROM songs WHERE id = :id;

--! get_song_album_id : (album_id?)
SELECT album_id FROM songs WHERE id = :id;

--! insert_song(id, name, duration, artist_id, album_id?, release_date?, track_number?, image_path?)
INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path)
VALUES (:id, :name, :duration, :artist_id, :album_id, :release_date, :track_number, :image_path);

--! update_song_name(name, id)
UPDATE songs SET name = :name WHERE id = :id;

--! update_song_duration(duration, id)
UPDATE songs SET duration = :duration WHERE id = :id;

--! update_song_release_date(release_date, id)
UPDATE songs SET release_date = :release_date WHERE id = :id;

--! update_song_track_number(track_number, id)
UPDATE songs SET track_number = :track_number WHERE id = :id;

--! update_song_partial(id, name?, duration?, artist_id?, album_id?, release_date?, track_number?, image_path?)
UPDATE songs SET
    name = COALESCE(:name, name),
    duration = COALESCE(:duration, duration),
    artist_id = COALESCE(:artist_id, artist_id),
    album_id = COALESCE(:album_id, album_id),
    release_date = COALESCE(:release_date, release_date),
    track_number = COALESCE(:track_number, track_number),
    image_path = COALESCE(:image_path, image_path)
WHERE id = :id;

--! delete_song
DELETE FROM songs WHERE id = :id;
