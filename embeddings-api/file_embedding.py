from openai import OpenAI 

client = OpenAI()

def get_file_embedding_from_name(file_name):
    response = client.embeddings.create(
        input = file_name, 
        model = "text-embedding-3-large"
    )

    return response.data[0].embedding 

def get_file_embedding_from_content(file_content):
    response = client.embeddings.create(
        input = file_content, 
        model = "text-embedding-3-large"
    )

    return response.data[0].embedding

