use anyhow::Result;
use async_openai::types::{CreateEmbeddingRequest, EmbeddingInput};
use async_openai::Client;

use crate::parser::ParsedFile;
use crate::EMBEDDING_DIM; 

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-x))
}


fn calculate_name_weight(filename: &str, content: &str) -> f32{
    let file_extension_len = filename.split('.').last().unwrap_or("").len() as f32;
    let name_letter_cnt = filename.chars().filter(|c| c.is_alphabetic()).count() as f32 - file_extension_len;
    let name_letter_frac = name_letter_cnt / (filename.len() as f32 - file_extension_len);

    let content_space_cnt = content.chars().filter(|c| c.is_whitespace()).count() as f32;
    let content_space_frac = content_space_cnt / content.len() as f32;   // usually 1 space every 6 chars

    let name_legitness = name_letter_frac.powf(1.5) * (name_letter_cnt / 30.0 + 1e-5).powf(0.5);
    let content_legitness = content_space_frac * 6.0 * (content_space_cnt / 2000.0 + 1e-5).powf(0.5);

    let logit = name_legitness - content_legitness;
    println!("logit: {}", logit);
    sigmoid(logit * 1.0)  // shouldn't exceed 1
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
        let content = clip_string(&x.content.clone().unwrap(), 2000);
        if content.len() == 0 {
            vec![x.name.clone(), "_".to_string()]
        } else {
            vec![x.name.clone(), content]
        }
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
        let name_weighting = calculate_name_weight(&parsed_file.name, &parsed_file.content.clone().unwrap());
        let mut filename_embd = name_embed.iter().map(|x| x * (name_weighting + 1e-5).sqrt()).collect::<Vec<f32>>();
        let content_embd = content_embed.iter().map(|x| x * (1.0 - name_weighting + 1e-5).sqrt()).collect::<Vec<f32>>();
        filename_embd.extend(content_embd);
        let mag = filename_embd.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        println!("mag: {}", mag);
        filename_embd
    }).collect();

    Ok(result)
}

