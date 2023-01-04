use std::env;

pub fn read_dir() -> std::io::Result<Vec<String>> {
    let directories: Vec<String> = env::current_dir()?
        .join("book")
        .join("src")
        .read_dir()?
        .map(|x| x.unwrap().path().into_os_string().into_string().unwrap())
        .filter(|x| x.ends_with(".md"))
        .collect();
    /* Way to invoke error */
    // Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
    Ok(directories)
}

pub fn read_file(file: String) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let content: String = String::from_utf8_lossy(&std::fs::read(file)?).parse()?;
    Ok(content)
}
