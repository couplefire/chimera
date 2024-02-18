use anyhow::Result;
use async_openai::{types::{CreateEmbeddingRequest, EmbeddingInput}, Client};

use crate::EMBEDDING_DIM; 

pub async fn create_embedding_prompt(prompt: &str) -> Result<Vec<f32>> {
    let client = Client::new(); // looks for OPENAI_API_KEY environment variable
    let result = client.embeddings().create(CreateEmbeddingRequest {
        model: "text-embedding-3-small".to_string(),
        input: EmbeddingInput::String(prompt.to_string()),
        dimensions: Some(EMBEDDING_DIM),
        ..Default::default()
    }).await?;

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
