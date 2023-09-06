# Sommelier
A Discord bot written in Rust. The goal of this bot is _not_ to create an amazing piece of software, but to just have fun with friends and have something fun to work on.

## Modules
This is a brief description of the three modules of this project. The first two are named folders in the root of this directory, and the third (main module) is contained in the **src** directory.

### Ante
Startup script required to initialize the [Discord Application Commands](https://discord.com/developers/docs/interactions/application-commands) required by Sommelier. This module requires the use of the **contractor** module to manage commands.

### Contractor
Contractor is a library responsible for managing the set of allowed [Discord Application Commands](https://discord.com/developers/docs/interactions/application-commands) associated with Sommelier.

### Sommelier
Contains the main Sommelier logic of the bot. This module is set up as an [AWS Lambda](https://aws.amazon.com/lambda/), which takes in a user Discord interaction, and returns the appropriate response. See [this documentation](https://discord.com/developers/docs/interactions/receiving-and-responding) for more on Discord interaction structure.

## Environment Setup

To deploy this app, you will need [Cargo](https://github.com/rust-lang/cargo) >= 1.71.0 and [Cargo Lambda](https://www.cargo-lambda.info/) >= 0.20.1. You will also need your [AWS CLI to be properly configured](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html). 

You will also need an application with a bot user in the [Discord Developer Portal](https://discord.com/developers/applications). 

You will also need the following environment variables: 
- `SOMMELIER_PUBLIC_KEY`
- `SOMMELIER_APPLICATION_ID`
- `SOMMELIER_BOT_TOKEN`
- `SOMMELIER_LAMBDA_EXECUTION_ROLE`

The first three of these can be found in the Discord Developer Portal. `SOMMELIER_LAMBDA_EXECUTION_ROLE` should be created on your AWS account, with the **AWSLambdaBasicExecutionRole** policy. 

## Building and Running

This project uses Rust's package manager, [Cargo](https://doc.rust-lang.org/cargo/), to abstract away most of the complications of the build process. To build the project, run `cargo build`. To test the project, run `cargo test`. 

To deploy the project, run `bash deploy.sh`, which builds the project, and then deploys it to a lambda named `sommelier`. 

To run just the **ante** module, which initializes the discord bot, run `cargo run -p ante`. Make sure to deploy your bot before running this script, or else the Discord backend may reject your command initialization.
