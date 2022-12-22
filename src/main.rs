use std::env;

fn main() -> std::io::Result<()> {
    let dir_entries = env::current_dir()?.join("book").join("src").read_dir()?;
    for dir_entry in dir_entries {
        // match dir_entry?.file_type()
        match dir_entry?.file_name().to_str() {
            Some(dir_name) => {
                if dir_name.ends_with(".md") {
                    println!("{}", dir_name)
                }
            }
            None => {}
        }
        // println!("{}", dir_entry?.file_name().to_str().un);
    }
    Ok(())
}
