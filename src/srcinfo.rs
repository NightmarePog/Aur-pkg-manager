pub fn parse_srcinfo(srcinfo_path: &str) -> anyhow::Result<()> {
    let srcinfo = srcinfo::Srcinfo::from_path(srcinfo_path)?;

    Ok(())
}
