# CodeWhisperer Test Lab - TODO list in Rust and TypeScript
Trying CodeWhisperer LLM coding assistant to build a TODO list app in Rust and TypeScript.

## Server 
- CodeWhisperer (CW) is not very useful when working from a blank slate (it suggests Rocket code even when the comments say use Axum)
- CW likes to suggest modules even they they are not used
- CW does not work in the `Cargo.toml` file


## Client
To give CW a skeleton to infer from, I created the client application with `create-react-app` like this:

    npx create-react-app client --template typescript

