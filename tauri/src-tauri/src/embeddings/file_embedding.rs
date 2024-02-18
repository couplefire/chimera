use anyhow::Result;
use async_openai::types::{CreateEmbeddingRequest, EmbeddingInput};
use async_openai::Client;

use crate::parser::ParsedFile;
use crate::EMBEDDING_DIM; 

pub async fn create_embedding_file(parsed_file: ParsedFile) -> Result<Vec<f32>> {
    let client = Client::new(); // looks for OPENAI_API_KEY environment variable

    let mut combined_str = parsed_file.name;
    combined_str.push_str("\n");
    combined_str.push_str(&parsed_file.content.unwrap());

    let result = client.embeddings().create(CreateEmbeddingRequest {
        model: "text-embedding-3-small".to_string(),
        input: EmbeddingInput::StringArray(vec![combined_str]),
        dimensions: Some(EMBEDDING_DIM),
        ..Default::default()
    }).await?;

    let mut mag = 0.0;
    for i in 0..result.data[0].embedding.len() {
        mag += result.data[0].embedding[i] * result.data[0].embedding[i];
    }
    println!("Magnitude of data {}", mag);
    Ok(result.data[0].embedding.clone())
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

