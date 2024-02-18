use std::io::Read;
use std::fmt;

use crate::db;

#[derive(Clone)]
pub struct ParsedFile {
    pub name: String, 
    pub extension: String, 
    pub path: String, 
    pub content: Option<String>, 
    pub file_size: u64, 
    pub num_pages: Option<u64>
}

/*
impl fmt::Debug for ParsedFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsedFile {{ name: {:?}, extension: {:?}, path: {:?}, content: {:?}, file_size: {:?}, num_pages: {:?} }}",
            self.name, self.extension, self.path, self.content, self.file_size, self.num_pages)
    }
}
*/

impl std::fmt::Debug for ParsedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParsedFile")
            .field("name", &self.name)
            .field("extension", &self.extension)
            .field("path", &self.path)
            .field("content", &self.content)
            .finish()
    }
}

fn get_pdf_content(path: &str) -> String {
    let document = lopdf::Document::load(path).unwrap();
    let mut content = String::new();
    for i in 0..document.get_pages().len() {
        content.push_str(document.extract_text(&[(i+1) as u32]).unwrap().as_str());
    }
    content
}

fn get_pdf_num_pages(path: &str) -> u64 {
    let document = lopdf::Document::load(path).unwrap();
    document.get_pages().len() as u64
}

pub fn parse(path: &str) -> Option<ParsedFile> {
    let path_obj = std::path::Path::new(path);
    let name = path_obj
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let extension = path_obj
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_string();
   

    let content: Option<String> = match extension.as_str() {
        "pdf" => {
            Some(get_pdf_content(path))
        },
        "txt" => {
            let mut content = String::new();
            let mut file = std::fs::File::open(path).unwrap();
            file.read_to_string(&mut content).unwrap();
            Some(content)
        }, 
        _ => {
            let mut content = String::new();
            let mut file = std::fs::File::open(path).unwrap();
            file.read_to_string(&mut content).unwrap_or_default();
            Some(content)
        },
    };

    Some(ParsedFile {
        name,
        extension: extension.clone(),
        path: path.to_string(),
        content,
        file_size: std::fs::metadata(path).unwrap().len(),
        num_pages: if extension == "pdf" { Some(get_pdf_num_pages(path)) } else { None }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let file_paths = [
            "/Users/couplefire/Downloads/glass.pdf"
        ];

        for path in file_paths.iter() {
            let parsed_file = parse(path);
            println!("{:?}", parsed_file)
        }

        assert_eq!(1, 1);
    }
}