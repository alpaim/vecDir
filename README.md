# VecDir

> **Vectorize your directory for better search**

A local-only, privacy-minded desktop application for semantic/similarity file indexing and retrieval. Unlike traditional search tools that look for exact keyword matches, this tool uses vector embeddings to understand the *meaning* of your queries.

Built for privacy and performance using **Tauri**, **Rust**, **LanceDB** and many other great technologies.

It uses multimodal local LLMs (like Gemma 3, Ministral, Qwen3-VL, etc.) to generate descriptions of each file. Then, it turns these descriptions into embeddings using local embeddings. It stores vectors in LanceDB, index and metadata in SQLite. This tool doesn't touch your files. It just indexes them and stores its index separately.

While you are searching for some file, your search query is processed into an embedding and then, thanks to LanceDB search, you get the similar files you are looking for.

> **Your files are yours. No data leaves your device**