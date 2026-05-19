import ollama

# sends prompt to model
response = ollama.chat(model='llama3.1', messages=[
    {
        'role': 'system',
        'content': 'You are a local, secure AI assistant. Do not roleplay as a cloud service.',
    },
    
    {
        'role': 'user',
        'content': 'Hello! Can you verify that you are running locally on my computer',
    }
])

# print model's response
print(response['message']['content'])