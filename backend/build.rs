fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = std::fs::read_dir("proto")?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "proto")
        })
        .collect::<Vec<_>>();

    connectrpc_build::Config::new()
        .files(&proto_files)
        .includes(&["proto/"])
        .include_file("_connectrpc.rs")
        .compile()?;

    Ok(())
}
