pub mod filesystem;

fn main() -> std::io::Result<()> {
    filesystem::read_dir()?;
    Ok(())
}
