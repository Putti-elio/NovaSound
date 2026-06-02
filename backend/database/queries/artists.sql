--: Artist(id, name, image_path)

--! get_all_artists : Artist
SELECT id, name, image_path FROM artists;

--! get_artist_by_id : Artist
SELECT id, name, image_path FROM artists WHERE id = :id;

--! check_artist_by_name
SELECT 1 FROM artists WHERE name = :name;

--! check_artist_by_id
SELECT 1 FROM artists WHERE id = :id;

--! get_artist_name_by_id
SELECT name FROM artists WHERE id = :id;

--! insert_artist(id, name, image_path)
INSERT INTO artists (id, name, image_path) VALUES (:id, :name, :image_path);

--! update_artist(name, image_path, id)
UPDATE artists SET name = :name, image_path = :image_path WHERE id = :id;

--! update_artist_partial(id, name?, image_path?)
UPDATE artists SET
    name = COALESCE(:name, name),
    image_path = COALESCE(:image_path, image_path)
WHERE id = :id;

--! delete_artist
DELETE FROM artists WHERE id = :id;
