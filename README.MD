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

$ rustc -V
rustc 1.43.0 (4fb7144ed 2020-04-20)

```

## Install & Setting

```bash
#Not required if installed
rustup target add x86_64-unknown-linux-musl
brew install filosottile/musl-cross/musl-cross

#setting your account
aws configure
```

Got the authentication information of the GCP service account.

```json:google_secret.json
{
  "type": "service_account",
  "project_id": "xxxxxxx",
  "private_key_id": "xxxxxxxx",
  "private_key": "xxxxxx",
  "client_email": "xxxxxxxx",
  "client_id": "xxxxxxxxx",
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token",
  "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
  "client_x509_cert_url": "xxxxxxxxxxx"
}
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