cargo lambda build --release
cargo lambda deploy sommelier --iam-role $PROSPECTOR_LAMBDA_EXECUTION_ROLE
