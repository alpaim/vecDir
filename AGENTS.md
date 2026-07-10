# AGENTS.md

## Project Mission

VecDir is a Tauri 2 desktop application built with React, TypeScript, and Rust. It is intended for local-first semantic and multimodal-embedding file search: users attach filesystem directories to a logical Space, VecDir indexes file metadata and content into SQLite, and search compares query embeddings with stored text and image embeddings through `sqlite-vec`.

Preserve these product guarantees:

- Treat user files as read-only. Indexing may read files and may delete stale records from VecDir's database, but it must not modify, move, rename, or delete source files.
- Keep indexes and metadata separate from source files in the Tauri application-data directory.
- A Space is the unit of embedding configuration and search compatibility; Roots are physical directories attached to that Space.
- The app is local-first, not necessarily offline-only. AI endpoints may be local or remote according to user configuration. Never silently introduce a new network destination.
- The Rust backend and SQLite database are authoritative. Zustand is a persisted UI cache, not the source of truth for Spaces, Roots, files, or embeddings.

## Repository Map

### Frontend

- `src/main.tsx` — React entry point and manually assembled TanStack Router tree.
- `src/routes/` — `/`, `/createSpace`, `/settings`, and the persistent root layout.
- `src/components/` — application UI; reusable shadcn/Radix primitives live in `src/components/ui/`.
- `src/components/settings/createSpace.tsx` and `settings.tsx` — TanStack Form configuration for Spaces and embedding backends.
- `src/lib/vecdir/bindings.ts` — generated Tauri/Specta commands, events, and Rust-derived TypeScript types.
- `src/lib/vecdir/{spaces,roots,indexer,search,files}/` — small frontend wrappers around generated commands and Tauri plugins.
- `src/store/store.ts` — Zustand runtime state and persisted UI cache.
- `src/store/storage.ts` — hybrid persistence using browser `localStorage` first and Tauri Store's `app-state.json` as fallback.
- `src/index.css` — Tailwind CSS v4 imports and theme tokens; there is no `tailwind.config.*`.

### Rust/Tauri backend

- `src-tauri/src/lib.rs` — Tauri startup, plugins, command/event registration, async database initialization, and debug binding export.
- `src-tauri/src/state.rs` — managed `AppState`, currently containing the SQLx SQLite pool.
- `src-tauri/src/database/` — schema models and database access for config, Spaces, Roots, files, chunks, and vectors.
- `src-tauri/migrations/` — embedded SQLx migrations.
- `src-tauri/src/indexer/crawler.rs` — filesystem crawl and metadata reconciliation.
- `src-tauri/src/indexer/chunker.rs` — text splitting into 1,024-character chunks with 150-character overlap.
- `src-tauri/src/indexer/processor.rs` — pending-file processing and embedding-backend dispatch.
- `src-tauri/src/ai.rs` — OpenAI-compatible client plus shared image/base64 and vector preparation logic.
- `src-tauri/src/ai/vecbox.rs` — VecBox multimodal embedding client.
- `src-tauri/src/ai/llamacpp.rs` — llama.cpp multimodal embedding client and its mock-HTTP tests.
- `src-tauri/src/search/embedding.rs` — query embedding, vector retrieval, per-file deduplication, and ranking.
- `src-tauri/src/status/events.rs` — backend-ready, progress, notification, and error event types.
- `src-tauri/capabilities/default.json` and `src-tauri/tauri.conf.json` — desktop permissions, asset protocol, packaging, and frontend build integration.

## Runtime and Data Flow

### Startup

1. `vecdir_lib::run` registers Tauri plugins and the Specta command/event surface.
2. Database initialization runs asynchronously during Tauri setup.
3. `initialize_database` creates `<app-data>/vecDir.db`, registers `sqlite-vec`, opens a WAL-mode SQLx pool, runs migrations, and verifies `vec_version()`.
4. Only after initialization completes is `AppState` managed and `BackendReadyEvent` emitted.
5. Frontend code must remain gated by `useBackendReady`; commands that require `State<AppState>` are not safe before readiness.

### Indexing is intentionally two-stage

Do not collapse or reorder these stages accidentally:

1. `index_space` crawls every Root in a Space. The `ignore` walker includes hidden files while honoring `.gitignore` and `.ignore`, upserts filesystem metadata, marks changed files as `pending` based on mtime, and removes database records for paths no longer found.
2. `process_space` loads pending files, dispatches by MIME type and `EmbeddingBackendType`, chunks text, creates embeddings, inserts chunks/vectors, and marks successful files `indexed`.

The frontend currently sequences both calls in `src/components/index/main.tsx`. Crawling alone does not make content searchable.

### Search

1. Load the selected Space's `EmbeddingConfig`.
2. Embed the text query with the same backend/model family used for indexing.
3. Search `vec_chunks` by cosine distance, filtered through Root membership to the selected Space.
4. Oversample candidates, retain the best chunk per file, sort by ascending distance, and return the requested number of files.

Search results contain the winning chunk, file identity/path, and cosine distance. A lower distance is better.

## Embedding Backends

`EmbeddingBackendType` in `src-tauri/src/database/models.rs` is the central dispatch enum. A backend addition or change must be reflected in the model, processor, query embedding path, settings/create forms, generated bindings, and tests.

| Backend | Indexing behavior | Endpoint/config notes |
| --- | --- | --- |
| `openai_compat` | Text is chunked and embedded directly. Images are described by the configured multimodal LLM, then the description is embedded. | Uses `async-openai`; LLM and embedding URLs/keys are separate. |
| `vecbox` | Text and images are embedded directly by the multimodal embedding model. | `VecboxClient` appends `/v1/embeddings`; configure the server root, not a URL already ending in `/v1`. `api_key` is currently unused. |
| `llamacpp` | Text and images are embedded directly. Images use `prompt_string` plus raw base64 `multimodal_data`. | Appends `/v1/embeddings`; the media marker defaults to `<__media__>` or is fetched from `/props`. Only the image prompt's user text is currently consumed. `api_key` is unused. |

All backends must preserve these invariants:

- Every vector stored in `vec_chunks` is exactly 768 `f32` values because the virtual table is `float[768] distance_metric=cosine`.
- Current clients truncate longer vectors to 768 and L2-normalize them. Reject short or zero-norm vectors rather than storing malformed data.
- Prefix truncation is valid only for embedding models that support Matryoshka-style representations; do not assume every model is compatible.
- Indexing and query embeddings for a Space must remain mutually compatible.
- Changing a Space's backend, model, dimensions, normalization, or embedding-relevant prompt invalidates existing vectors. Preserve the UI warning and implement explicit reindex/invalidation behavior when changing this flow; updating Space JSON alone does not rebuild vectors.

## Database Invariants

- Use SQLx with `anyhow::Result` and add context at subsystem boundaries.
- Add schema changes as new timestamped files under `src-tauri/migrations/`. Never rewrite an applied migration.
- `file_chunks.id` and `vec_chunks.rowid` are the same logical ID. Chunk and vector inserts must remain paired in one transaction.
- Deleting a `file_chunks` row triggers deletion of the corresponding vector. Preserve this relationship in deletion/reindex work.
- Keep Space filtering in vector search. Cross-Space comparisons are invalid and may mix incompatible models.
- Roots and absolute file paths are globally unique in the current schema. Be careful with overlapping Roots, path canonicalization, Windows case differences, and any uniqueness migration.
- Space AI configuration, including API keys, is stored as JSON in SQLite and may also appear in the persisted frontend cache. Do not log credentials, add real keys to tests, or describe this storage as secure secret storage.
- Database code declares cascading foreign keys, but initialization does not currently enable `PRAGMA foreign_keys` explicitly. Verify actual cascade behavior before relying on it in new destructive operations.

## Tauri and TypeScript Contract

- Rust commands and event types are the contract source of truth.
- New commands require `#[tauri::command]`, `#[specta::specta]`, and registration in `collect_commands!` in `src-tauri/src/lib.rs`.
- New events require the Specta/Tauri event derive and registration in `collect_events!`.
- Do not edit `src/lib/vecdir/bindings.ts` manually. It is generated and intentionally excluded from ESLint.
- Bindings are exported when a debug build executes `run()`, normally via `bun run tauri dev`; a plain `cargo build` or `bun run build` does not execute the exporter.
- After changing Rust IPC types/commands/events: run the debug app to regenerate bindings, inspect the generated diff, then update wrappers and call sites.
- Preserve serialized snake_case Rust field names such as `api_base_url`, `embedding_config`, and `fetch_marker_from_server` in frontend values.
- `search_by_emdedding` / `searchByEmdedding` is an existing misspelled IPC name. Do not rename it as incidental cleanup; a correction must update registration, generated bindings, wrappers, and callers together.
- Tauri plugin operations may require both Rust plugin registration and capability entries. Review `src-tauri/capabilities/default.json` when adding filesystem, opener, dialog, clipboard, or asset-protocol behavior.

## Frontend Conventions

- Use strict TypeScript and the `@/` import alias.
- Prefer existing shadcn/Radix components in `src/components/ui/`, `lucide-react` icons, and `cn` from `src/lib/utils.ts`.
- Follow existing TanStack Form patterns: controlled `form.Field` values, `handleBlur`, `handleChange`, guarded native submit handlers, and `form.Subscribe` for backend-dependent controls.
- Keep all backend modes representable in both create and edit forms. Although some fields are unused by direct multimodal backends, the Rust `Space` model currently requires complete `LLMConfig` and `EmbeddingConfig` values.
- TanStack Router routes are assembled manually in `src/main.tsx`; add new route objects to the route tree explicitly.
- Zustand `spaces` is a `Map`. Replace it with a new `Map` when mutating so subscribers are notified, and preserve Map-to-array serialization in persistence.
- Event listeners must clean up the promised unlisten callback. React runs under `StrictMode`, so leaked subscriptions are easy to duplicate during development.
- Use wrappers under `src/lib/vecdir/` instead of scattering direct command/plugin calls through components. Do not treat zero-byte placeholder wrappers as implemented behavior.
- Browser-only `bun run dev` cannot exercise Tauri commands or plugins. Use `bun run tauri dev` for integration behavior.

## Rust Conventions

- Keep modules small and aligned with the existing `database`, `indexer`, `ai`, `search`, and `status` boundaries.
- Use `anyhow` for internal errors, `Context` for actionable failure messages, Serde for wire/storage models, SQLx for database access, and `reqwest` for direct HTTP clients.
- Do not copy existing `unwrap()`, `todo!()`, or error-erasing `Result<_, ()>` patterns into new fallible code. Propagate useful command errors and regenerate the frontend contract when changing error types.
- Do not block the async runtime with new heavy filesystem, CPU, or inference work. Use async APIs or isolate blocking work appropriately.
- No new code comments unless explicitly requested. Prefer clear names and small functions; retain comments only when they explain a non-obvious safety invariant or protocol requirement.

## Safe Change Workflows

### Changing commands or shared models

1. Update Rust models and command implementation.
2. Register the command/event in `src-tauri/src/lib.rs`.
3. Run `bun run tauri dev` to regenerate `bindings.ts`.
4. Update frontend wrappers, Zustand shapes if relevant, forms, and callers.
5. Validate both Rust and frontend layers.

### Changing embedding behavior

1. Update both file processing and query embedding dispatch.
2. Preserve 768-dimensional normalized vectors or add a coordinated schema migration and compatibility strategy.
3. Decide how existing files become pending and how old chunks/vectors are removed before reprocessing.
4. Test request serialization, response ordering, HTTP errors, short vectors, and zero norms with mock HTTP; never require a live model server for unit tests.
5. Verify text and image paths separately.

### Changing indexing or deletion behavior

- Distinguish deletion from the index from deletion of a source file. Source-file deletion is outside the current product contract.
- Preserve chunk/vector transactional pairing and Space isolation.
- Account for partial crawl failures before interpreting unseen paths as deleted.
- Reprocessing must replace stale chunks/vectors rather than append duplicates.

### Changing versions or desktop permissions

- Keep the application version synchronized in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`.
- Treat CSP, asset protocol scope, opener scope, and filesystem capabilities as security-sensitive changes.

## Build, Run, and Validation

Use Bun; `bun.lock` is the frontend lockfile.

### From the repository root

- Install dependencies: `bun install`
- Frontend dev server only: `bun run dev`
- Full Tauri development app: `bun run tauri dev`
- Frontend lint: `bun run lint`
- Frontend lint fix: `bun run lint:fix`
- Strict typecheck and frontend production build: `bun run build`
- Full desktop bundle: `bun run tauri build`
- Preview built frontend: `bun run preview`

There is currently no frontend test script or configured frontend test runner.

### From `src-tauri/`

- Format check: `cargo fmt --check`
- Format fix: `cargo fmt`
- Build: `cargo build`
- Test: `cargo test`
- Lint: `cargo clippy --all-targets --all-features`

Rust test coverage currently focuses on `src-tauri/src/ai/llamacpp.rs`. Add focused tests near new Rust behavior, especially for protocol clients and vector invariants.

### Minimum validation by change type

- Markdown/docs only: inspect the diff and run a whitespace check.
- Frontend only: `bun run lint` and `bun run build`.
- Rust only: `cargo fmt --check`, relevant `cargo test` targets, and `cargo clippy --all-targets --all-features` when practical.
- Shared IPC contract: Rust checks, regenerate bindings via `bun run tauri dev`, then `bun run lint` and `bun run build`.
- Migration/index/vector changes: run Rust tests and exercise a fresh database plus an upgrade/reindex path when feasible.

Do not run broad auto-fix commands over unrelated user work without inspecting the resulting diff.

## Current Constraints to Keep in Mind

- Processing is sequential; `AppConfig.indexer_parallelism` is not wired into the processor yet.
- `process_space` handles at most 1,000 pending files per invocation.
- MIME handling is based on file extension, and unsupported file types are not consistently handled across backends.
- Existing reprocessing does not first remove old chunks, so idempotent replacement is required when working in that area.
- Backend command wrappers often collapse failures into empty/false values, and several Rust commands still panic on internal errors. Do not assume an empty result means success.
- Placeholder modules/wrappers exist for unimplemented operations such as stop-indexing and delete-Space. Confirm an end-to-end implementation before exposing related UI.
- Search has no distance threshold and may return fewer than the requested limit after per-file deduplication.
