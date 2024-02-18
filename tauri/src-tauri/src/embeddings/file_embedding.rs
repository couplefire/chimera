use openai_api_rs::v1::api::Client; 
use openai_api_rs::v1::common::TEXT_EMBEDDING_3_SMALL; 
use openai_api_rs::v1::embedding::EmbeddingRequest; 
use anyhow::Result;

use crate::parser::ParsedFile;
use crate::EMBEDDING_DIM; 
use std::env;

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-x))
}

fn calculate_name_weight(name: &str) -> f32{
    let count = name.chars().filter(|c| c.is_alphabetic()).count();
    let frac = count as f32 / name.len() as f32;
    sigmoid((frac - 0.5)* 12.0) * 0.6  // shouldn't exceed 1
}

fn clip_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        s.chars().take(max_len).collect()
    } else {
        s.to_string()
    }
}

pub fn create_embedding_file(parsed_file: ParsedFile) -> Result<Vec<f32>> {
    let openai_key: String = env::var("OPENAI_API_KEY").unwrap().to_string();
    let client = Client::new(openai_key); 

    let filename = parsed_file.name;
    let mut content = parsed_file.content.unwrap();
    let mut name_req = EmbeddingRequest::new(TEXT_EMBEDDING_3_SMALL.to_string(), filename.clone()); 
    name_req.dimensions = Some(EMBEDDING_DIM);

    content = clip_string(content.as_str(), 2000);
    let mut content_req = EmbeddingRequest::new(TEXT_EMBEDDING_3_SMALL.to_string(), content); 
    content_req.dimensions = Some(EMBEDDING_DIM);

    let filename_embd = (client.embedding(name_req)?).data[0].embedding.clone(); 
    let content_embd = (client.embedding(content_req)?).data[0].embedding.clone(); 

    let name_weighting = calculate_name_weight(&filename);
    let mut filename_embd = filename_embd.iter().map(|x| x * name_weighting).collect::<Vec<f32>>();
    let content_embd = content_embd.iter().map(|x| x * (1.0 - name_weighting)).collect::<Vec<f32>>();
    filename_embd.extend(content_embd);

    Ok(filename_embd)
}


// fn create_embedding_file_name(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string()); 

//     let mut req = EmbeddingRequest::new(TEXT_EMBEDDING_3_SMALL.to_string(), file_name.to_string()); 
//     req.dimensions = Some(10); 

//     let result = client.embedding(req)?; 
//     println!("{:?}", result.data); 

//     Ok(())
// }

// fn create_embedding_file_content(file_content: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string()); 

//     let mut req = EmbeddingRequest::new(TEXT_EMBEDDING_3_SMALL.to_string(), file_content.to_string()); 
//     req.dimensions = Some(10); 

//     let result = client.embedding(req)?; 
//     println!("{:?}", result.data); 

//     Ok(())
// }



// struct OpenAI; 

// impl OpenAI {
//     fn create(&self, input: &str, model: &str) -> Vec<f64> {
//         vec![0.1, 0.2, 0.3]
//     }
// }

// fn get_file_embeeding_from_name(file_name: &str) -> Vec<f64> {
//     let client = OpenAI; 
//     client.create(file_name, "text-embedding-3-large")
// }

// fn get_file_embedding_from_content(file_content: &str) -> Vec<f64> {
//     let client = OpenAI; 
//     client.create(file_content, "text-embedding-3-large")
// }

// from openai import OpenAI 

// client = OpenAI()

// def get_file_embedding_from_name(file_name):
//     response = client.embeddings.create(
//         input = file_name, 
//         model = "text-embedding-3-large"
//     )

//     return response.data[0].embedding 

// def get_file_embedding_from_content(file_content):
//     response = client.embeddings.create(
//         input = file_content, 
//         model = "text-embedding-3-large"
//     )

//     return response.data[0].embedding

