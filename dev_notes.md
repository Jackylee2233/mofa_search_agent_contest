# Mofa Development Notes

## Setup Server

- [x] **Base Server Initialization**
  - ✅ Base server setup using `rust-axum-async-graphql-postgres-redis-starter`.
    - **Description**: Utilized the `rust-axum` template from GitHub.
  - ✅ Added a general WebSocket router powered by Cursor.
  - ✅ Integrated a WebSocket client example via Cursor.
  - ✅ Successfully tested client-server connection.

- [x] **One-Click Dora Setup**
  - ✅ Created a new Rust project named `dora_auto_run`.
    - **Description**: Automates execution of the Mofa Dora AI agent project. Command sequence: `cargo run <dora_auto_run>` → `dora up` → `dora build` → `dora start`.
  - ✅ Tested basic commands (`dora --version`, `dora --help`) successfully.
  - ✅ Attempted adding Dora build-in tracing log to `dora_auto_run_test` with AI assistance (Cursor performance suboptimal).
  - ✅ Manually added the Dora tracing crate. Experimented with grouping `dora_auto_run_test` into a new Rust workspace. Discontinued and deleted workspace.
  - ✅ Successfully tested `dora-node-api` with the `["tracing"]` feature:
    1. ✅ Copied Dora Rust example `rust-dataflow` into the `working` workspace.
    2. ✅ Verified the standalone `dora-tracing` crate functionality. Note: Works only with `dora-cli new` or `dora-node-api` with `tracing` feature.
  - ✅ Created a new Dora project called `mofa_search_agent` using `dora new`.
  - ✅ Tested one-click execution of `mofa_search_agent` via another Rust project, `mofa_search_agent_contest` (03-26-2025, 16:15).
    - **Notes**:
      - One-click execution requires wrapping the Dora project in a new Rust project. However, this may cause issues with `rust-analyzer` inside the Dora project, while the wrapper project functions correctly.
      - A potential feature request for the `rust-analyzer` team could address this limitation.

- [ ] **New CLI Tool Development**
  1. ✅ Establish a new layer on top of `dora-cli` to enhance functionality, such as environment checks before running Dora projects.
  2. ✅ Before executing the Dora CLI wrapper, include a `dora check` step to verify the status of `Dora Coordinator` and `Dora Daemon`. If not running, execute `dora up`. Integrated this check into the wrapper (03-26-2025, 18:00).
  3. ✅ Wrapper inspired by Cursor Editor's Agent function, which facilitates project creation in the working directory. Plan to utilize the `Clap` crate for this wrapper.
  4. More expected features:
     - Wrapper to act as a super node external to the Dora dataflow.
     - Functionality to serve custom compute needs beyond Dora’s published capabilities.
     - Additional development planned (03-26-2025, 20:38).

## Setup Moly as Frontend

- ✅ **Updates**
  Received confirmation from Alex regarding new Moly functionality to handle network connectivity errors. Awaiting repository merge and sync.

- ✅ **Basic Concept**  
Moly's frontend enables seamless interaction with remote LLM services without requiring a server and connects to local LLM services through `moly-server`. Local services manage simpler queries, while more complex tasks are delegated to remote LLMs, ensuring efficient token usage and cost optimization.

- [ ] **To-Do**
  1. [ ] Upgrade `moly-server` to support WebSocket connections.
  2. [ ] Add `WebSocket` client functionality to the Moly frontend and a control node in the `Dora` project.
  3. [ ] Establish real-time communication between `moly-server`, `moly`, and `dora` via WebSocket.

- [ ] **Dataflow Design**
  1. [ ] Initial inquiry processed in Moly, response passed to Dora for iterative processing in its nodes, and results sent back to Moly for display.
  2. [ ] Enable Moly to intercept messages from Dora and display them alongside the original chat messages for comparison.
  3. [ ] Introduce a new Moly message window with buttons for firing additional LLM requests. This window will support multiple messages.
  4. [ ] Add a Redis node to the Dora dataflow for buffering and managing duplicate information.
  5. [ ] Further enhancements planned @03-26-2025, 21:30

- **Project Vision**
  - ✅ Designed as a human assistant leveraging LLMs to manage daily workloads.
  - [ ] Features include `firing`, `mixing`, `modifying` prompts, `reviewing` responses, and `comparing` outputs from multiple LLMs before finalizing replies.
  - [ ] All tools and options will be integrated into the `Moly UI`

## Setup moly-server
- [ ] TODO !...
---
  *Ready for upload and create new github repo* @03-27-2025:15:23