# Understand RAG

I wrote this project during the summer and finished it up fall of 2025.
Working on this project was a little bit ironic, since I'd been making fun of people for doing ChatGPT wrappers for so long.
It was a simple, cool concept. 
I build a chat that would 
1. let the user manually set how many messages it could read back 
2. let the user choose how many messages the llm could retrieve with RAG
Behind the scenes, I used langchain just as an API wrapper for text gen and embeddings.
For the uninitiated, RAG can support LLMs by sifting through potentially important information and adding the most relevant pieces to the prompt.
How do they choose which ones? They use embeddings, which are a big vector (direction in high dimensional space) that can be used to find similarities.

![image of app](https://raw.githubusercontent.com/kyteware/understandrag/main/images/sample1.png?token=GHSAT0AAAAAADRUGFTVIIC77H6NPVJEMNQA2KK4ANA)

After sending messages, you can hover over different responses to see which parts the llm used as context.
This was a great opportunity for me to get caught up on using LLMs programatically!
