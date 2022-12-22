use std::env;

fn main() -> std::io::Result<()> {
    let dir_entries = env::current_dir()?.join("book").join("src").read_dir()?;
    for dir_entry in dir_entries {
        println!("{:?}", dir_entry?.file_name());
    }
    Ok(())
}
