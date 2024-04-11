use std::{env, io::prelude::*};
use std::fs::File;
use std::fs;
fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
        //std::io::copy(&mut file, &mut std::io::stdout())?;
    }

    Ok(())
}
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let folder_name :&String = &args[1];
    let paths = fs::read_dir("./".to_string()+folder_name).unwrap();

    for path in paths {
        //println!("Name: {}", path.unwrap().path().display())
        let file = File::open(path.unwrap().path());
        list_zip_contents(file.unwrap())?;
    }
    
    //println!("Hello, world!");
    Ok(())
}
