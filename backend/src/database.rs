use function_name::named;
use log::{error, info};
use rusqlite::{Connection, Error, Result};

#[named]
pub fn init_database() -> Result<Connection, Error> {
    let database = Connection::open("data/database.db").map_err(|e| {
        error!(
            "Database couldn't be initialized: {}. At {}::{}",
            e,
            file!(),
            function_name!()
        );
        e
    })?;

    let query = "
        CREATE TABLE IF NOT EXISTS artists (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            image_path TEXT
        );

        CREATE TABLE IF NOT EXISTS albums (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            total_duration INTEGER DEFAULT 0,
            release_date INTEGER,
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
            release_date INTEGER,
            track_number INTEGER,
            image_path TEXT,
            FOREIGN KEY (artist_id) REFERENCES artists(id),
            FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE SET NULL
        );

        CREATE INDEX IF NOT EXISTS idx_songs_album_id ON songs(album_id);
        CREATE INDEX IF NOT EXISTS idx_songs_artist_id ON songs(artist_id);
    ";

    database.execute_batch(query).map_err(|err| {
        error!(
            "Failed to initialise the database and to create tables: {}. At {}::{}",
            err,
            file!(),
            function_name!()
        );
        err
    })?;

    info!("Tables created successfully!");
    Ok(database)
}
