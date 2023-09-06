# Sommelier
A Discord bot written in Rust, to be deployed in a matching "Kitchen" server. The goal of this bot is _not_ to create an amazing piece of software, but to just have fun with friends and have something fun to work on in an interesting language. 

## Rust
Rust is a sleek, new(ish) general purpose programming language that's gotten a lot of [hype](https://dev.to/somedood/rust-reviewed-is-the-hype-justified-1pa1) and some [criticism](https://www.reddit.com/r/rustjerk/). It's also a "harder" language to learn, so do expect some difficulty when you start using it. To start, you should be familiar with the first 6 chapters of [the Rust Book](https://doc.rust-lang.org/book/), which is a well-written guide on the core language elements of Rust.

One of the most immediately noticeable features of Rust is the Rust compiler, which will be invoked whenever you 

## Contributing to Sommelier
Sommelier is intended to be an open space to experiment with Rust in a Discord bot setting. To contribute, you should open a pull request with passing tests that illustrate the functionality of your contribution. 

## Code Modules
A brief description of the three modules of this project. The first two are named folders in the root of this directory, and the third (main module) is contained in the **src** directory.

### Ante
Startup script required to initialize the [Discord Application Commands](https://discord.com/developers/docs/interactions/application-commands) required by Sommelier. This module requires the use of the **contractor** module to manage commands.

### Contractor
Contractor is a library responsible for managing the set of allowed [Discord Application Commands](https://discord.com/developers/docs/interactions/application-commands) associated with Sommelier.

### Sommelier
Contains the main Sommelier logic of the bot. This module is set up as an [AWS Lambda](https://aws.amazon.com/lambda/), which takes in a user Discord interaction, and returns the appropriate response. See [this documentation](https://discord.com/developers/docs/interactions/receiving-and-responding) for more on Discord interaction structure.

## Environment Setup

To develop on this app, you will need [Cargo](https://github.com/rust-lang/cargo) >= 1.71.0 and [Cargo Lambda](https://www.cargo-lambda.info/) >= 0.20.1. You will also need your [AWS CLI to be properly configured](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html). 

You will also need an application with a bot user in the [Discord Developer Portal](https://discord.com/developers/applications). 

You will also need the following environment variables: 
- `SOMMELIER_PUBLIC_KEY`
- `SOMMELIER_APPLICATION_ID`
- `SOMMELIER_BOT_TOKEN`
- `SOMMELIER_LAMBDA_EXECUTION_ROLE`

The first three of these can be found in the Discord Developer Portal. `SOMMELIER_LAMBDA_EXECUTION_ROLE` should be created on your AWS account, with the **AWSLambdaBasicExecutionRole** policy. 

## Building and Testing Locally

This project uses Rust's package manager, [Cargo](https://doc.rust-lang.org/cargo/), to abstract away most of the complications of the build process. To build the project, run `cargo build`. To test the project, run `cargo test`. 

## DIY
If you are only looking to write code on this project, don't read this section. If you want to deploy your own version of the bot, continue on.

To deploy the project, run `bash deploy.sh`, which builds the project, and then deploys it to a lambda named `sommelier`. 

To run just the **ante** module, which initializes the discord bot, run `cargo run -p ante`. Make sure to deploy your bot before running this script, or else the Discord backend may reject your command initialization.
