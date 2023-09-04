cargo lambda build --release
cargo lambda deploy prospector --iam-role $LAMBDA_EXECUTION_ROLE
