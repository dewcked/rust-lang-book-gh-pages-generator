use std::env;

pub fn read_dir() -> std::io::Result<()> {
    let directories = env::current_dir()?
        .join("book")
        .join("src")
        .read_dir()?
        .map(|x| x.unwrap().path().into_os_string().into_string().unwrap())
        .filter(|x| x.ends_with(".md"));

    for dir_entry in directories {
        println!("{:}", dir_entry);
    }

    Ok(())
}
