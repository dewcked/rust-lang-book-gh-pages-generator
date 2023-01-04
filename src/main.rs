pub mod filesystem;

fn main() -> std::io::Result<()> {
    let directories = filesystem::read_dir()?;
    for file in directories {
        println!("{}", file);
        println!("{}", filesystem::read_file(file).unwrap());
    }
    Ok(())
}
