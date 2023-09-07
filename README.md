# Sommelier
A Discord bot written in Rust, to be deployed in a matching "Kitchen" server. The goal of this bot is _not_ to create an amazing piece of software, but to just have fun with friends and have something fun to work on in an interesting language. 

## Rust
[Rust](https://www.rust-lang.org/) is a sleek, new(ish) general purpose programming language that's gotten a lot of [hype](https://dev.to/somedood/rust-reviewed-is-the-hype-justified-1pa1) and some [criticism](https://www.reddit.com/r/rustjerk/). It's also a "harder" language to learn, so do expect some difficulty when you start using it. To start, you should be familiar with the first 6 chapters of [the Rust Book](https://doc.rust-lang.org/book/), which is a well-written guide on the core language elements of Rust.

One of the most immediately noticeable features of Rust is the Rust compiler, which will be invoked whenever you run `cargo build` or `cargo test`. This compiler will provide strong guidance on how to structure your code, in a way that may be frustrating at first. As some pretentious [rustaceans](https://rustaceans.org/) might say, this is just the normal "conversation with the compiler" to teach you about Rust's typing system. Do not be alarmed if the compiler yells at you, very often. 

## Cargo
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is the primary build/project/package manager for Rust. Useful Cargo commands:
- `cargo build`, to build the project
- `cargo test`, to test the project
- `cargo fmt`, to format the project's code to conform to some standard
- `cargo help`, to get more cargo commands.
To develop on this app, you will need Cargo >= 1.71.0.

# If you want to write code...

## Just modify the code in the `app/` folder
Most of the rest of the files in this repository have to do with file organization, project building, or project deployment. So if you just want to add functionality to the bot itself, you can focus your efforts to that folder. Once you've added some functionality to the bot, open a pull request (PR), and it will be deployed for you. 

## PR Requirements
- PR must include tests that summarize the behavior of this addition
- PR must satisfy CI, which runs checks formatting, building, and tests

# If you want to deploy your own Sommelier...

## Environment Setup

To deploy this app, you will need [Cargo Lambda](https://www.cargo-lambda.info/) >= 0.20.1. You will also need your [AWS CLI to be properly configured](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html). 

You will also need an application with a bot user in the [Discord Developer Portal](https://discord.com/developers/applications). 

You will also need the following environment variables: 
- `SOMMELIER_PUBLIC_KEY`
- `SOMMELIER_APPLICATION_ID`
- `SOMMELIER_BOT_TOKEN`
- `SOMMELIER_LAMBDA_EXECUTION_ROLE`

The first three of these can be found in the Discord Developer Portal. `SOMMELIER_LAMBDA_EXECUTION_ROLE` should be created on your AWS account, with the **AWSLambdaBasicExecutionRole** policy. 

## Deploying
To deploy the project, run the `deploy.sh` scripts, which builds the project, and then deploys it to a lambda named `sommelier`. 

To run just the **ante** module, which initializes the discord bot, run `cargo run -p ante`. Make sure to deploy your bot before running this script, or else the Discord backend may reject your command initialization.
