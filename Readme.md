### How to Start the Project

This project is based on Rust. 

#### Prerequisites:
- Install [Rust 2024 Edition](https://www.rust-lang.org/tools/install).
  - Command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install [Dora-cli](https://dora-rs.ai/zh-CN/).
  - Command: `cargo install dora-cli`
  - If you encounter issues, try using the `--locked` flag.

#### Verify Successful Setup:
- Ensure you have Rust 2024 Edition:
  - Check Rust version: `rustc --version`
    - Example: rustc 1.85.1 (4eb161250 2025-03-15)
  - Check Cargo version: `cargo --version`
    - Example: cargo 1.85.1 (d73d2caf9 2024-12-31)
- Check Dora-cli version:
  - Command: `dora --version`
    - Example: dora-cli 0.3.10

#### Clone the Project:
- Clone the repository:
  - Command: `git clone https://github.com/Jackylee2233/mofa_search_agent_contest.git`

#### Run the Project: (Ubuntu 24.10 tested)
1. Navigate to the project directory:
   - Command: `cd mofa_search_agent_contest`
2. Start Dora services:
   - Command: `dora up`
   - Output should display:
     - "Started dora coordinator"
     - "Started dora daemon"
   - If errors occur:
     - Run `dora destroy` before retrying `dora up`.
     - Alternatively, use `dora check` to inspect the Dora status.
3. Run the project:
   - Set the environment variable: `DORA=dora`
   - Then Run Command: `cargo run`
   
   - Full Command: `DORA=dora cargo run`
     - This command will act as a wrapper to automatically execute:
       - `dora build dataflow.yml`
       - `dora start dataflow.yml`

### How to Stop the Project

- The project runs in an infinite loop to demonstrate the Dora framework. To stop it, press `Ctrl+C` in your terminal.

*Note: Updates will be added here as the project evolves.*

### How to Debug the Project

- Debug information can be found in the `out` folder located in the root directory of `mofa_search_agent`.

### Project diagram

This is auto generated project diagram, will auto update after every commit push.

```mermaid
graph TB
    %% External Users/Systems
    Timer((Timer Service))

    subgraph "Mofa Search Agent System"
        subgraph "Communication Services"
            DoraRuntime["Dora Runtime<br>(Rust/Dora)"]
        end

        subgraph "Publisher Services"
            Talker1["Talker 1<br>(Rust)"]
            Talker2["Talker 2<br>(Rust)"]

            subgraph "Talker Components"
                T1Init["Node Initializer<br>(DoraNode)"]
                T1EventHandler["Event Handler<br>(Rust)"]
                T1Publisher["Message Publisher<br>(Dora API)"]
            end
        end

        subgraph "Subscriber Services"
            Listener1["Listener 1<br>(Rust)"]

            subgraph "Listener Components"
                L1Init["Node Initializer<br>(DoraNode)"]
                L1EventHandler["Event Handler<br>(Rust)"]
                L1MessageProcessor["Message Processor<br>(Dora API)"]
            end
        end
    end

    %% Relationships
    Timer -->|"Triggers (100ms)"| Talker1
    Timer -->|"Triggers (2s)"| Talker2
    Timer -->|"Triggers (1s)"| Listener1

    %% Talker 1 internal relationships
    Talker1 -->|"Initializes"| T1Init
    T1Init -->|"Configures"| T1EventHandler
    T1EventHandler -->|"Uses"| T1Publisher
    T1Publisher -->|"Sends 'speech' events"| DoraRuntime

    %% Talker 2 internal relationships
    Talker2 -->|"Initializes"| T1Init
    T1Init -->|"Configures"| T1EventHandler
    T1EventHandler -->|"Uses"| T1Publisher
    T1Publisher -->|"Sends 'speech' events"| DoraRuntime

    %% Listener 1 internal relationships
    Listener1 -->|"Initializes"| L1Init
    L1Init -->|"Configures"| L1EventHandler
    L1EventHandler -->|"Uses"| L1MessageProcessor

    %% Runtime relationships
    DoraRuntime -->|"Routes messages"| L1MessageProcessor
    DoraRuntime -->|"Manages communication"| T1Publisher

```

**Map the communication interfaces between talker and listensr components**
```mermaid
%%{init: {'theme': 'default', 'themeVariables': { 
    'background': '#ffffff',
    'primaryColor': '#f4f4f4',
    'primaryTextColor': '#333333',
    'primaryBorderColor': '#cccccc',
    'lineColor': '#666666',
    'secondaryColor': '#f9f9f9',
    'tertiaryColor': '#fff'
}}}%%
graph TD
    %% Timer Components
    Timer100ms[Timer 100ms]
    Timer1s[Timer 1s]
    Timer2s[Timer 2s]

    %% Main Components
    subgraph Talker1
        T1Init[Initialize Node]
        T1Event[Event Handler]
        T1Send[Send Output]
        T1Error[Error Handler]
    end

    subgraph Talker2
        T2Init[Initialize Node]
        T2Event[Event Handler]
        T2Send[Send Output]
        T2Error[Error Handler]
    end

    subgraph Listener1
        L1Init[Initialize Node]
        L1Event[Event Handler]
        L1Process[Process Message]
        L1Error[Error Handler]
    end

    %% Data Serialization
    ArrowSer[Arrow Serialization]
    ArrowDeser[Arrow Deserialization]

    %% Timer Triggers
    Timer100ms -->|tick| T1Event
    Timer2s -->|tick| T2Event
    Timer1s -->|tick| L1Event

    %% Talker1 Flow
    T1Init --> T1Event
    T1Event -->|on tick| T1Send
    T1Send -->|speech| ArrowSer
    T1Error -.->|handle| T1Event

    %% Talker2 Flow
    T2Init --> T2Event
    T2Event -->|on tick| T2Send
    T2Send -->|speech| ArrowSer
    T2Error -.->|handle| T2Event

    %% Message Flow
    ArrowSer -->|serialized data| ArrowDeser
    ArrowDeser -->|speech-1| L1Process
    ArrowDeser -->|speech-2| L1Process

    %% Listener Flow
    L1Init --> L1Event
    L1Event -->|on input| L1Process
    L1Error -.->|handle| L1Event

```

### Relating to rust-analyzer
Since the entire Dora project (`mofa_search_agent`) is placed within a single Rust project (`mofa_search_agent_contest`), the Dora project essentially functions as a Rust workspace. However, having a workspace nested within another Rust project is currently not supported by rust-analyzer for syntax hints, suggestions, and similar features.

The solution is to open the Dora project separately in another IDE window. This way, you can fully utilize the support provided by rust-analyzer.

### License

This project uses the MIT license. For more information, see the [LICENSE](LICENSE) file.
