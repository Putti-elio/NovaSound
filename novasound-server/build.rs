fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=code/contracts/proto");
    let proto_files = std::fs::read_dir("code/contracts/proto")?
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
        .includes(&["code/contracts/proto/"])
        .include_file("_connectrpc.rs")
        .compile()?;

    Ok(())
}
