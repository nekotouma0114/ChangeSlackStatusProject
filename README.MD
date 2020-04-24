# What's this project

Googleカレンダーより今日の予定を取得し、
それに応じてSlackのステータスを変更するする(予定)。

# Environment

## Exmanple

```bash
$ aws --version
aws-cli/1.18.45 Python/3.8.2 Darwin/19.0.0 botocore/1.15.45

$ sam --version
SAM CLI, version 0.47.0

$ cargo --version
cargo 1.39.0 (1c6ec66d5 2019-09-30)

```

## Install & setting

```bash
#Not required if installed
rustup target add x86_64-unknown-linux-musl
brew install filosottile/musl-cross/musl-cross

#setting your account
aws configure
```

# Build

```bash
bash ./build.sh
```

# Run

```bash
sam local invoke -e event.json
```

# Deploy

```bash

#if exist samconfig.toml
sam deploy

# if not exist samconfig.toml
# this command can create "samconfig.toml"
# Select "Save arguments to samconfig.toml" Y
sam deploy --guided
```