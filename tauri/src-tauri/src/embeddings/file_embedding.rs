use anyhow::Result;
use async_openai::types::{CreateEmbeddingRequest, EmbeddingInput};
use async_openai::Client;

use crate::parser::ParsedFile;
use crate::EMBEDDING_DIM; 

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

pub async fn create_embedding_files(parsed_files: Vec<ParsedFile>) -> Result<Vec<Vec<f32>>> {
    let client = Client::new(); // looks for OPENAI_API_KEY environment variable

    let parsed_file_strings: Vec<String> = parsed_files.iter().flat_map(|x| {
        vec![x.name.clone(), clip_string(&x.content.clone().unwrap(), 2000)]
    }).collect();

    let result = client.embeddings().create(CreateEmbeddingRequest {
        model: "text-embedding-3-small".to_string(),
        input: EmbeddingInput::StringArray(parsed_file_strings),
        dimensions: Some(EMBEDDING_DIM),
        ..Default::default()
    }).await?;

    let result: Vec<Vec<f32>> = result.data.chunks(2).zip(parsed_files.iter()).map(|x| {
        let ([name_res, content_res], parsed_file) = x else {
            panic!("Mismatched lengths");
        };
        let name_embed = name_res.embedding.clone();
        let content_embed = content_res.embedding.clone();
        let name_weighting = calculate_name_weight(&parsed_file.name);
        let mut filename_embd = name_embed.iter().map(|x| x * name_weighting).collect::<Vec<f32>>();
        let content_embd = content_embed.iter().map(|x| x * (1.0 - name_weighting)).collect::<Vec<f32>>();
        filename_embd.extend(content_embd);
        filename_embd
    }).collect();

    Ok(result)
}

