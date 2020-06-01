use std::fs;
use std::io::Write;
use std::path;
use std::time;

use terminal::grid;

pub fn to_file(blueprint: grid::Segment) -> crate::Result<String> {
    let file_name = {
        let time = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH)?;
        format!("shketch-{}.txt", time.as_millis())
    };
    to_file_as(blueprint, &file_name)?;
    Ok(file_name)
}

pub fn to_file_as(blueprint: grid::Segment, file_name: &str) -> crate::Result {
    let mut file = fs::File::create(path::Path::new(&file_name))?;
    let content: String = blueprint.into();
    file.write_all(content.as_bytes())?;
    Ok(())
}
