struct OpenAI; 

impl OpenAI {
    fn create(&self, input: &str, model: &str) -> Vec<f64> {
        vec![0.1, 0.2, 0.3]
    }
}

fn get_prompt_embedding(prompt: &str) -> Vec<f64> {
    let client = OpenAI; 
    client.create(prompt, "text-embedding-3-large")
}

// # from openai import OpenAI 

// # client = OpenAI()

// # def get_prompt_embedding(prompt):
// #     response = client.embeddings.create(
// #         input = prompt, 
// #         model = "text-embedding-3-large"
// #     )

// #     return response.data[0].embedding 
