use tokio::{
    fs::OpenOptions, io::AsyncReadExt
};
use std::{
    error::Error, path::PathBuf
};

pub async fn read_file(file_name: PathBuf) -> Result<String, Box<dyn Error>> {
    let file_name = PathBuf::from(file_name);

    let mut file = OpenOptions::new()
        .read(true)
        .open(file_name)
        .await?;
                
    let mut content = String::new();
    file.read_to_string(&mut content).await?;

    Ok(content)
}