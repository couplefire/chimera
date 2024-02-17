from openai import OpenAI 

client = OpenAI()

def get_prompt_embedding(prompt):
    response = client.embeddings.create(
        input = prompt, 
        model = "text-embedding-3-large"
    )

    return response.data[0].embedding 
