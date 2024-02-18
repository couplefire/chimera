# Chimera

## Inspiration

Examining our difficulties in our daily workflow, we realized that we often could not find files that we knew we had written before. Lots of us had saved countless papers and books, only to be unsure of where we saved them. Not knowing where our old code was led us to rewrite old libraries and codebases. Thus, we all wished for a better file searcher. One which, instead of solely relying on filenames or exact string matching in the file content, also took into account how 

## What it does

Chimera will first ask you to describe the file you are looking for. Then it will go through all the files in your directory and rank them according to which fit the prompt the best.

## How we built it

We used the Tauri framework to build the app and wrote it in Rust. The app first embedded the prompts and the files into a vector space (using OpenAI's embedding models), then compared the similarity between the prompts and each file, then outputted them in order of likelihood. 

## Challenges we ran into

Building it in Rust was quite difficult, as some of us didn't know Rust and had to learn it on the fly, and there were a lot of issues working with some Rust libraries. 

## Accomplishments that we're proud of

The end product, being built with Rust, is highly performant. Our embeddings framework is also easily parallelizable, so it can easily be extended to index every file in a user's computer relatively quickly.

## What we learned

We learned how to work with Rust (especially the Tauri framework and Apache Arrow's data format that LanceDB uses) and how to use a vector DB.

## What's next for Chimera

While this hackathon used OpenAI's embeddings API, the end goal would be to ship a local embeddings model to the user's computer so their files won't have to leave their personal laptop. In addition, we hope to fine-tune the embeddings model to perform better at the specific task of searching for files.