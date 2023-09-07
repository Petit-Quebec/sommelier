cargo lambda build --release -p lambda
cargo lambda deploy --binary-name lambda --iam-role $SOMMELIER_LAMBDA_EXECUTION_ROLE sommelier
