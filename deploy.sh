cargo lambda build --release
cargo lambda deploy nyoomio --iam-role $LAMBDA_EXECUTION_ROLE
