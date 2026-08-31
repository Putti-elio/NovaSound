fn main() -> Result<(), Box<dyn std::error::Error>> {
    connectrpc_build::Config::new()
        .files(&[
            "proto/artist.proto",
            "proto/album.proto",
            "proto/song.proto",
        ])
        .includes(&["proto/"])
        .include_file("_connectrpc.rs")
        .compile()?;

    Ok(())
}
