struct OpenAI; 

impl OpenAI {
    fn create(&self, input: &str, model: &str) -> Vec<f64> {
        vec![0.1, 0.2, 0.3]
    }
}

fn get_file_embeeding_from_name(file_name: &str) -> Vec<f64> {
    let client = OpenAI; 
    client.create(file_name, "text-embedding-3-large")
}

fn get_file_embedding_from_content(file_content: &str) -> Vec<f64> {
    let client = OpenAI; 
    client.create(file_content, "text-embedding-3-large")
}

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

