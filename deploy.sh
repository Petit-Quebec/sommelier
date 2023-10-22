cargo lambda build --release
cargo lambda deploy --iam-role $SOMMELIER_LAMBDA_EXECUTION_ROLE sommelier
