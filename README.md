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

## Environment Setup
There are no required environment variables for the (non-web) proejct. If you want control over how gambling hashing is determined, you will need to set the following environment variable:
- `SOMMELIER_GAMBLING_SALT`

This variable is a secret key that will randomize an aspect of gambling in the app.

The way you do this will depend on your terminal. A way of doing this in VS Code can be found [here](https://stackoverflow.com/questions/48595446/is-there-any-way-to-set-environment-variables-in-visual-studio-code).

# If you want to write code...

## Pull Request Requirements
Even though this project is just for fun, we do need to enforce some level of regularity. The following three criteria are three very general acceptance criteria for pull requests (PRs).

### Testing Coverage
The PR must include tests that fully characterize and verify the functionality of the added command. 

### Minimality
This PR is minimal, in the sense that there isn't any way to remove logic to achieve the same level of functionality and test coverage. Of course, this judgment is somewhat subjective, but the point is that there isn't a ton of extra fluff.

### CI Passing
The PR must pass CI tests, which require that the code must:

- build, which you can test locally using `cargo build`
- pass all tests, which you can tests locally using `cargo test`
- be well formatted, which you can do by using `cargo fmt` before committing code

When you open a PR, there will be a notice on the PR that looks like:

![image](https://github.com/Petit-Quebec/sommelier/assets/36433367/ffbe403b-3c5d-4736-890e-daf9f9c93d42)

If everything goes well with the CI tests, then it will change to:

![image](https://github.com/Petit-Quebec/sommelier/assets/36433367/3c70a725-4ae0-4939-b8ee-021157bafcbe)

If something goes wrong, then it will change to:

![image](https://github.com/Petit-Quebec/sommelier/assets/36433367/1cfbe566-cc92-429f-bd06-37d3956368ec)

These checks verify that things are going "the way they should". If you've satisfied all of these criteria (testing coverage, minimality, CI passing) then your PR will be merged in. Example "hello world" PR here: https://github.com/Petit-Quebec/sommelier/pull/1

# Deployment

Sommelier is automatically deployed on push to main. Once your pull request is merged in, the changes to the bot will be propagated within a few minutes. For specification on deployment, see `.github/workflows/deployment.yml`.
