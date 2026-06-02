--: Album(id, name, total_duration, release_date?, artist_id, image_path?, album_type)

--! get_all_albums : Album
SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums;

--! get_album_by_id : Album
SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums WHERE id = :id;

--! get_albums_by_artist : Album
SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums WHERE artist_id = :artist_id;

--! check_album_by_id
SELECT 1 FROM albums WHERE id = :id;

--! check_album_by_name_and_artist
SELECT 1 FROM albums WHERE name = :name AND artist_id = :artist_id;

--! check_album_by_id_simple
SELECT 1 FROM albums WHERE id = :id;

--! get_album_image_path : (image_path?)
SELECT image_path FROM albums WHERE id = :id;

--! get_album_type_by_id : (album_type)
SELECT album_type FROM albums WHERE id = :id;

--! get_standalone_collection_id : (id?)
SELECT id FROM albums WHERE artist_id = :artist_id AND album_type = :album_type;

--! get_album_song_stats : (song_count, total_duration)
SELECT COUNT(*) AS song_count, COALESCE(SUM(duration), 0) AS total_duration FROM songs WHERE album_id = :album_id;

--! insert_album(id, name, release_date?, artist_id, image_path, album_type)
INSERT INTO albums (id, name, total_duration, release_date, artist_id, image_path, album_type)
VALUES (:id, :name, 0, :release_date, :artist_id, :image_path, :album_type);

--! insert_standalone_collection(id, name, artist_id, image_path, album_type)
INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type)
VALUES (:id, :name, 0, :artist_id, :image_path, :album_type);

--! update_album_name(name, id)
UPDATE albums SET name = :name WHERE id = :id;

--! update_album_release_date(release_date, id)
UPDATE albums SET release_date = :release_date WHERE id = :id;

--! update_album_artist_id(artist_id, id)
UPDATE albums SET artist_id = :artist_id WHERE id = :id;

--! update_album_duration(total_duration, id)
UPDATE albums SET total_duration = :total_duration WHERE id = :id;

--! update_album_duration_and_type(total_duration, album_type, id)
UPDATE albums SET total_duration = :total_duration, album_type = :album_type WHERE id = :id;

--! update_album_partial(id, name?, release_date?, artist_id?, total_duration?, image_path?, album_type?)
UPDATE albums SET
    name = COALESCE(:name, name),
    release_date = COALESCE(:release_date, release_date),
    artist_id = COALESCE(:artist_id, artist_id),
    total_duration = COALESCE(:total_duration, total_duration),
    image_path = COALESCE(:image_path, image_path),
    album_type = COALESCE(:album_type, album_type)
WHERE id = :id;

--! calc_album_duration : (total_duration)
SELECT COALESCE(SUM(duration), 0) AS total_duration FROM songs WHERE album_id = :album_id;

--! delete_album
DELETE FROM albums WHERE id = :id;
