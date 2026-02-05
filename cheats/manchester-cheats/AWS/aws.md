# AWS

## aws - set-up a profile
```
aws configure (--profile foo)

# For a local bucket, you can fill the prompt like so
aws configure set aws_access_key_id YOUR_ACCESS_KEY
aws configure set aws_secret_access_key YOUR_SECRET_KEY
aws configure set region us-east-1  # Any region works for local
aws configure set default.s3.endpoint_url http://localhost:54321 OR http://TARGET_IP
aws configure set default.s3.use_path_style true
```

## aws - list buckets
```
aws s3 ls --endpoint-url http://<IP>:54321
```

## aws - copy file from bucket to local
```
# cp for specific files
# sync for dir/continuous update
# Whole bucket
aws s3 sync s3://your-bucket ./local-folder --endpoint-url http://<IP>:54321

# Specific folder
aws s3 sync s3://your-bucket/folder/ ./local-folder --endpoint-url http://localhost:54321
```

## aws tips and tricks
```
Todo
```

