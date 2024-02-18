use openai_api_rs::v1::api::Client; 
use openai_api_rs::v1::common::TEXT_EMBEDDING_3_SMALL; 
use openai_api_rs::v1::embedding::EmbeddingRequest; 
use anyhow::Result;

use crate::EMBEDDING_DIM; 
use std::env;

pub fn create_embedding_prompt(prompt: &str) -> Result<Vec<f32>> {
    let openai_key: String = env::var("OPENAI_API_KEY").unwrap().to_string();
    let client = Client::new(openai_key); 

    let mut req = EmbeddingRequest::new(TEXT_EMBEDDING_3_SMALL.to_string(), prompt.to_string()); 
    req.dimensions = Some(EMBEDDING_DIM); 

    let result = client.embedding(req)?; 

    Ok(result.data[0].embedding.clone())
}

// fn create_embedding_file_content(file_content: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string()); 

//     let mut req = EmbeddingRequest::new(TEXT_EMBEDDING_3_SMALL.to_string(), file_content.to_string()); 
//     req.dimensions = Some(10); 

//     let result = client.embedding(req)?; 
//     println!("{:?}", result.data); 

//     Ok(())
// }

// # from openai import OpenAI 

// # client = OpenAI()

// # def get_prompt_embedding(prompt):
// #     response = client.embeddings.create(
// #         input = prompt, 
// #         model = "text-embedding-3-large"
// #     )

// #     return response.data[0].embedding 
