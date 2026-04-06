fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/esphd/v1/daemon.proto",
                "proto/esphd/v1/cluster.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
