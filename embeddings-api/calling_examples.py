## calling the functions from files 
# from file_embedding import get_file_embedding_from_name, get_file_embedding_from_content

# file_name_embedding = get_file_embedding_from_name('name') ## dummy variable 
# print(file_name_embedding)

# file_content_embedding = get_file_embedding_from_content('content') ## dummy variable 
# print(file_content_embedding)

# ## calling the functions from prompts 

# from prompt_embedding import get_prompt_embedding 

# prompt_embedding = get_prompt_embedding('prompt')
# print(prompt_embedding)

import json
import importlib.util

def print_json_config(file_path):
    try:
        with open(file_path, 'r') as file:
            config = json.load(file)
            print(json.dumps(config, indent=4))
            
            # Dynamically import functions from file_embedding.py and prompt_embedding.py
            spec = importlib.util.spec_from_file_location("file_embedding", "file_embedding.py")
            file_embedding = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(file_embedding)

            spec = importlib.util.spec_from_file_location("prompt_embedding", "prompt_embedding.py")
            prompt_embedding = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(prompt_embedding)

            # Call the API functions from the imported modules
            file_embedding.get_file_embedding_from_name('name')
            prompt_embedding.get_prompt_embedding('prompt')
    except FileNotFoundError:
        print("File not found or path is incorrect.")

# Specify the file path here
file_path = 'path/to/your/config.json'
print_json_config(file_path)