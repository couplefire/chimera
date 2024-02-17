use std::io::Read;
pub struct ParsedFile {
    name: String, 
    extension: String, 
    path: String, 
    content: Option<String>
}

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
    let file = std::fs::read(path).unwrap();
    let out = pdf_extract::extract_text_from_mem(&file).unwrap();
    out
}

pub fn parse(path: &str) -> ParsedFile {
    let path_obj = std::path::Path::new(path);
    let name = path_obj
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let extension = path_obj
        .extension()
        .unwrap()
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
        _ => None
    };

    ParsedFile {
        name,
        extension,
        path: path.to_string(),
        content,
    }
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