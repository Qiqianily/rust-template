fn main() -> anyhow::Result<()> {
    // 如果目录不存在则创建
    std::fs::create_dir_all("src/pb")?;
    let build = tonic_prost_build::configure();
    build
        .out_dir("src/pb")
        .compile_protos(&["proto/demo/greeter.proto"], &["proto"])?;
    Ok(())
}
