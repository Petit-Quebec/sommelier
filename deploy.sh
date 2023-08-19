cargo lambda build --release
cargo lambda deploy --iam-role $LAMBDA_EXECUTION_ROLE
