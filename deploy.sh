lambda_execution_role=`cat secrets/lambda-execution-role`
cargo lambda deploy --iam-role $lambda_execution_role
