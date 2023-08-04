# Simple Rust Lambda

[![Build](https://github.com/rayyildiz/go-rayyildiz-rs/actions/workflows/build.yml/badge.svg)](https://github.com/rayyildiz/go-rayyildiz-rs/actions/workflows/build.yml)


A simple aws lambda function using rust. It redirects the url that comes via [api gateway](https://docs.aws.amazon.com/apigateway/index.html) or [lambda function url](https://docs.aws.amazon.com/lambda/latest/dg/lambda-urls.html). 
You need to install [cargo-lambda](https://github.com/awslabs/aws-lambda-rust-runtime#getting-started)

Use this command to build binary. If you don't want to deploy to arm, just remove `--arm64` parameter from the command.
```shell
cargo lambda build --release --arm64
```

Then simply compress and upload the `./target/lambda/go-rayyildiz-rs/bootstrop` file.
