# VecDir

![vecdir](public/vecdir-banner.png)

> **Vectorize your directories for better search**

A local-only, very lightweight (**~10MB**), privacy-minded desktop application for semantic/similarity file indexing and retrieval. Unlike traditional search tools that look for exact keyword matches, this tool uses vector embeddings to understand the *meaning* of your queries.

Built for privacy and performance using **Tauri**, **Rust**, **SQLite-Vec** and many other great technologies.

It uses multimodal local LLMs (like Gemma 3, Ministral, Qwen3-VL, etc.) to generate descriptions of each file. Then, it turns these descriptions into embeddings using local embeddings. It stores vectors, index and metadata in SQLite. This tool doesn't touch your files. It just indexes them and stores its index separately.

While you are searching for some file, your search query is processed into an embedding and then, thanks to SQLite-Vec extension, you get the similar files you are looking for.

> **Your files are yours. No data leaves your device**

## What is it?

This is not another Electron-based AI tool that burns your RAM and sends data to the cloud.

**Type and Memory Safety**: The entire backend is written in Rust, compiled to native machine code with zero-cost abstractions and guaranteed memory safety. No garbage collection pauses, no runtime overhead.

**Native Performance**: Unlike Electron apps, VecDir uses the operating system's native WebView. The final binary is around **10MB**. The application consumes minimal memory, leaving maximum resources available for your local LLM inference.

**Complete Privacy**: All processing happens on your machine. No API calls to external services unless you explicitly configure them. Your files, your models, your data. Period.

**Non-Destructive Indexing**: VecDir operates in strict read-only mode on your files. It creates virtual indexes stored separately in SQLite. There is no code path that can modify, move, or delete your original files. **The application physically cannot harm your data**.

**Multi-Space Architecture**: Create isolated spaces with different configurations. Use a vision-heavy model with custom prompts for your photo library, and a code-specialized model for your repositories. Each space maintains its own index, embeddings, and AI config.

**Provider Agnostic**: The system does not lock you into OpenAI, Anthropic, or any specific vendor. Point it to LM Studio, Ollama, vLLM, or your private cloud endpoint. Full control over prompts and model selection.

## Tech Stack
### Core & Backend
* **Rust**: Core logic, file system crawler/indexer, files processor, and database interactions
* **Tauri**: Application framework bridging the Rust backend with the local web frontend rendered in OS' native WebView
* **SQLX** + **SQLite** + **SQLite-Vec**: Local SQL database with a vector search extension connected using SQLX crate
* **async-openai**: Rust crate for communicating with OpenAI-compatable API
* **Specta**: for automated binding generation from Tauri commands/events/structures to TypeScript frontend

### Frontend
* **TypeScript**: The only way I use JavaScript in this project
* **React**: UI
* **TanStack Router**: Type-safe routing
* **Zustand**: State management with my custom hybrid persistence strategy - LocalStorage of WebView for instant access on startup and file-based powered by Rust and Tauri to ensure safety
* **TailwindCSS / Shadcn UI**: Styling and components

## Architecture
The application architecture is designed around modularity and user control. It detaches the application logic from the underlying AI providers, allowing the system to act as a neutral orchestration layer rather than a wrapper for a specific vendor.

### 1. Hierarchical Data Organization: Spaces and Roots
The data model is built on a two-tier hierarchy designed to separate configuration from physical storage:

* **Spaces** (Logical Layer): A Space is a self-contained configuration unit. It defines how data should be processed, not just where it is. Each Space maintains its own isolated SQLite index and specific configuration sets for LLMs and Embeddings. This allows users to create distinct environments; for example, one Space using a vision-heavy model (like Qwen3-VL 235B) with strict prompts for image datasets, and another using a code-specialized model (DeepSeek Coder) for repositories.

* **Roots** (Physical Layer): Roots are the actual filesystem directories linked to a Space. A single Space can aggregate data from multiple disjoint directories (Roots) across the disk. The crawler monitors these paths for changes, ensuring the logical index stays synchronized with the physical file system without moving files.

### 2. Provider & Prompt Agnosticism
The system is architected to be completely agnostic regarding the AI backend. It does not hardcode reliance on OpenAI, Anthropic, or any specific cloud provider.
* **Universal API Compatibility**: The backend uses the async-openai Rust crate but allows the user to override the `api_base_url` and `api_key` for both the LLM and embedding models. This enables the application to connect to:
    * **Local Inference Servers**: LLama.cpp, vLLM, LM Studio, Ollama, etc.
    * **Private Enterprise Cloud APIs**: Custom endpoints for really heavy private models.
    * **Public Cloud Providers**: OpenAI, Groq, or OpenRouter.

* **Granular Prompt Engineering**: Prompts are not embedded in the code. They are stored as mutable configurations within the database. The user has full control over the `system_prompt` and `user_prompt` for different pipelines (Text Processing vs. Image Processing). This exposes the RAG strategy to the user, allowing them to optimize how files are summarized and indexed based on their specific domain requirement.

### 3. Resource Efficiency
All AI Apps are heavy. Usually. Even if you are not inference locally. But why?

The vast majority of Desktop apps are built over Electron. Really great technology, but it makes apps slow and heavy.

Unlike typical desktop AI assistants built on Electron, vecDir leverages the Tauri v2 framework.

* **Binary Size**: By utilizing the operating system's native WebView and compiling the backend to native Rust machine code, the final application distributable is approximately **~10MB**.
* **Memory Footprint**: The application avoids the overhead of bundling a full Chromium instance. This ensures the background indexing processes and vector search operations leave maximum system resources available for the local LLMs running alongside the application.

### 4. Backend and Frontend communication
The communication is powered by Tauri commands - a wrapper functions application exports to the frontend. Specta crate generates bindings for better type-safety. Basically, the Backend is the only Source of Truth. The client is Dumb.

Thanks for Rust, Tauri and Specta, the communication is made in a very elegant way, making helper wrapper functions using Tauri Command macro, and then calling it using generated bindings from the client.

## Data Pipeline
The system operates on an automated ingestion (crawling/indexing & processing) and retrieval pipeline designed to run entirely on consumer hardware

### 1. Crawling/Indexing
The application uses the [ignore](https://crates.io/crates/ignore) crate (the part of [ripgrep](https://github.com/BurntSushi/ripgrep) project) to walk the directory tree, respecting `.ignore` and `.gitignore` files.
* **Differential Indexing**: The scanner compares file modification timestamps (mtime) against the files_metadata SQL table. Only new or modified files are queued for processing.
* **Concurrency**: File walking and database upsert operations are handled asynchronously to prevent UI blocking.

### 2. Processing Layer; Utilizing AI
Once files are queued, they pass through a processing logic that acts as a router based on **MIME** types. This ensures the correct model is applied to the correct data type:
* **Vision Pipeline**: Image files are converted to Base64 and sent to a Vision-Language Model (Gemma, Ministral, Qwen-VL). The model executes a system prompt to describe the visual content specifically for retrieval purposes.
* **Text Pipeline**: Code and text files are read and sent to a text-based LLM (Gemma, Mistral, Qwen) to generate a concise summary of the file's purpose and content.
* **Error Handling**: Failed processings are logged in the database with error messages for debugging, preventing the entire batch from failing.
* **Chunking**: Each response splits into chunks to save it in vector storage.

### 3. Vectorization & Storage
* **Embeddings**: The generated descriptions are batched and sent to an embedding model (EmbeddingGemma, Qwen-Embedding).

* **Matroshka Embeddings**: The system implements Matroshka Representation Learning logic to truncate vectors to 768 dimensions if necessary, optimizing storage size without significant precision loss.

* **Chunking**: Each chunk represents the part - chunk - of the file indexed in database; Vector form of each chunk is being saved in the database.

### 4. Retrieval (Semantic Search)
When a user queries the system:
1. Their text prompt converts into vector using the same embedding model as `space` user searching in.
2. A Cosine Similarity search is performed by `sqlite-vec` against the `vec_chunks` table.
3. Results are joined with metadata and returned to the frontend, sorted by distance score (relevance).
