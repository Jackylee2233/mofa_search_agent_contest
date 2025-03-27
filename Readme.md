### How to start project

This project is Rust based.
- prerequirement:
    * install [Rust 2024 Edition ](https://www.rust-lang.org/tools/install)
        * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    * install [Dora-cli](https://dora-rs.ai/zh-CN/)
        * `cargo install dora-cli`
        * In case of issues, try: `--locked`
        
- success run setup ref:
not necessary the same version but need Rust 2024 Edition
    * `rustc --version`
        * rustc 1.85.1 (4eb161250 2025-03-15)
    * `cargo --version`
        * cargo 1.85.1 (d73d2caf9 2024-12-31)

    * `dora --version`
        * dora-cli 0.3.10

- clone project:
    * `git clone https://github.com/mofa***.git`

- enter project and run command:
    * `cd mofa_search_agent_contest` 
    * `dora up`
        * show: started dora coordinator
        * show: started dora daemon
        * if show error: run `dora destroy` before `dora up` or run `dora check` to get dora status.
    * `DORA=dora cargo run` 
        * must set env `DORA=dora` before `cargo run`
        * this `cargo run` will run a dora wapper to auto run `dora build dataflow.yml` and `doar start dataflow.yml`for you.

### How to stop project.

- you are running a scaffolding projcet, so far, it will run infinite loop to demonstrate a base dora porjcet, `Ctrl+C` in your terminal will stop the projcet.

...TODO!
- as project going will update here.

### How to debug project.

- building debug in the `out` folder under the root of `mofa_search_agent`