# motoko
![motoko motorcycle](https://i.pinimg.com/originals/56/55/bb/5655bbf38aedf1ff44e926c190859c7b.png)
> I'll have my AI analyze the data.

## TODO
- lambdas:
  - infer data types
  - upload dataset with types
  - calculate statistic
  - plot
  - transform data
  - model
- todo:
  - allow cloning projects
  - accounting to eliminate orphaned dataviews vs garbage collection?
  - copy plots/stats/models when dataview updated
  - type args instead of using Null/serde_json::Value
- clean up lambda:
  - clear stale refresh tokens
  - clear orphaned dataviews
  - clear orphaned datasets (even possible?)
- errors:
  - separate error messages for dev vs prod:
    - https://doc.rust-lang.org/reference/conditional-compilation.html
    - https://doc.rust-lang.org/beta/rustc/command-line-arguments.html
- fix sliding up panel: https://github.com/akshathjain/sliding_up_panel/issues/193
- setup [RDS Proxy](https://aws.amazon.com/blogs/compute/using-amazon-rds-proxy-with-aws-lambda/)
  to manage connection pooling for lambdas
- google auth submit for review
- get rights to Motoko font for mobile apps too
- motoko pypi sdk
- [truncated text on mobile web](https://github.com/flutter/flutter/issues/63467)

## Local Setup

#### General
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 
- `./install_motoko_command`
- `motoko install aws`
- `aws configure`
  - [Users -> user -> Security Keys](https://console.aws.amazon.com/iam/home#/users)
  - [Regions](https://docs.aws.amazon.com/general/latest/gr/rande.html)
    (default to `us-west-1`, which is Northern California)
  - output format: `json`
- `motoko run setup-android-keystore`
- `motoko -h`

#### Database
- add sqlx cli:
  - `cargo install --version=0.1.0-beta.1 sqlx-cli --no-default-features --features postgres`
- setup database:
  - `CREATE DATABASE motoko`
  - `CREATE USER motoko WITH PASSWORD '<password>'  # check .env file`
  - `GRANT ALL PRIVILEGES ON DATABASE motoko TO motoko;`
- setup environment to connect:
  - `vim motoko/backend/rs/gql/.env`:
    - `ADDRESS=http://127.0.0.1:3000`
    - `DATABASE_URL=postgres://motoko:<password>@localhost/motoko  # app client`
    - `GOOGLE_OAUTH_CLIENT_ID=<ID>`
    - `JWT_KEY=<key>`
- run migrations:
  - `sqlx migrate run`

#### Simulators
##### Android
- `motoko run emulator {android,ios,web}`
- `frontend $ flutter run`
- if you get `PlatformException...ApiException: 10`, see App Signing section
- if you get an out of date Google Play Services:
  - sign in on device
  - go to Google Play
  - Settings
  - click Play Store version to update

## Infrastructure
- Frontend uses [flutter](https://flutter.dev/), which is written in dart
- Backend uses API Gateway and Lambda Functions; most code is written in rust
  or python

## Data Models
![model graph](https://github.com/danjenson/motoko/blob/prod/backend/rs/gql/models.pdf)

#### Creating a Migration
- `sqlx migrate add <name> # fill out in migrations/<name>.sql`
- `cargo sqlx prepare -- --lib # recompile static type checking`

#### Mappings
- postgres types are mapped to internal diesel types, which are, in turn,
  mapped to native rust types:
    - running `diesel migration run` outputs `src/schema.rs`, which represents
      a generated mapping between postgres types and internal diesel types
    - the `diesel-derive-enum` crate creates internal diesel enums for postgres
      enums, since they are not supported natively by diesel cli:
      - rust enums, i.e. MyEnum::MyVariant, are assumed to be `my_enum` and
        `my_variant` in postgres
      - using the `#[PgType = "MY_ENUM"]` on a rust enum will map it to
        `MY_ENUM` inside postgres, instead of `my_enum`
      - using the `#[DieselType = "My_enum"]` on a rust enum will map it to the
        diesel type `My_enum`, which is often necessary, since diesel cli using
        `print_schema` (specified in
        [diesel.toml](https://github.com/danjenson/motoko/blob/prod/backend/rs/gql/diesel.toml))
        assumes that a postgres enum like `MY_ENUM` or `my_enum` will be mapped
        to title-case, i.e. `My_enum`, which is not the default
        created by this crate (by default it would create `MyEnumMapping`)
      - more details about renaming can be found
        [here](https://github.com/adwhit/diesel-derive-enum)
      - using `import_types` 
        [diesel.toml](https://github.com/danjenson/motoko/blob/prod/backend/rs/gql/diesel.toml)
        allows importing special mappings, like 
        [enums](https://github.com/danjenson/motoko/blob/prod/backend/rs/gql/src/enums.rs)
        that are used in the `src/schema.rs`
      - using `patch_file` in 
        [diesel.toml](https://github.com/danjenson/motoko/blob/prod/backend/rs/gql/diesel.toml)
        allows adding a patch to the `src/schema.rs` after generation; to make
        one, commit `src/schema.rs`, then make a change, then run
        `git diff -U6 > src/schema.patch` and delete the first two lines; this
        will make whatever changes you just made to the `src/schema.rs` after
        every auto-generation

## Deployment
- automatic deployment for the `dev` and `prod` branches is setup for every
  push using [AWS CodeBuild](https://docs.aws.amazon.com/codebuild/latest/userguide/sample-ecr.html)
  - the [custom build image](https://github.com/danjenson/motoko/blob/prod/build_image/Dockerfile)
    is hosted on [AWS ECR](https://us-west-1.console.aws.amazon.com/ecr/repositories/motoko/permissions?region=us-west-1)
  - CodeBuild [dev](https://console.aws.amazon.com/iam/home?#/roles/codebuild-motoko-dev-service-role)
    and [prod](https://console.aws.amazon.com/iam/home?#/roles/codebuild-motoko-prod-service-role)
    roles have ECR, Lambda, S3, and Secret Manager permissions
  - [buildspec-dev.yaml](https://github.com/danjenson/motoko/blob/prod/buildspec-dev.yml) and
    [buildspec-prod.yaml](https://github.com/danjenson/motoko/blob/prod/buildspec-prod.yml)
    contain the respective build steps
  - the CodeBuild [dev](https://us-west-1.console.aws.amazon.com/codesuite/codebuild/902096072945/projects/motoko-dev/history?region=us-west-1)
    and [prod](https://us-west-1.console.aws.amazon.com/codesuite/codebuild/902096072945/projects/motoko-prod/history?region=us-west-1)
    pipelines provide the progress and logs for builds

## Authentication and Authorization
- [Google login](https://console.cloud.google.com/apis/credentials?folder=&organizationId=&project=motoko-286819)

## App Signing
- android requires a keystore to sign the release app:
  - to setup building locally using the release keys, run
    `motoko run setup-android-keystore`, which does the following:
      - downloads the android keystore to
        `~/.keys/motoko/android/signing_key.jks`
      - creates the file `motoko/android/key.properties`, which contains the
        password to unlock the keystore (also from AWS Secrets Manager) and is
        used when building by gradle; do not add either of these files to the
        code repo
  - to reset the keystore in AWS Secrets Manager, run `motoko run
    reset-android-keystore`, which does the following:
      - generates a new keystore and uploads it to AWS Secrets Manager with the
        key `android_keystore` along with the password under the key
        `android_keystore_password`
      - runs the same commands as `motoko run setup-android-keystore` to setup
        the local environment to use the new keys
      - __NOTE__: after a reset, you will need to run `./gradlew signingReport`
        from the `motoko/android` directory and copy the debug and release SHA1
        hashes into the OAuth2 clients configs:
        [motoko-android-debug](https://console.cloud.google.com/apis/credentials/oauthclient/714421651437-d95mopk70t0o0d9gphomcncu3961ge9s.apps.googleusercontent.com?project=motoko-286819)
        and
        [motoko-android-release](https://console.cloud.google.com/apis/credentials/oauthclient/714421651437-nk7lev14vc27gpa6o30c2o0mc25btmge.apps.googleusercontent.com?project=motoko-286819);
        this lets google login know that builds using these signatures are
        legitimate; if the hashes are incorrect, google will reject attempted
        logins and return a `PlatformException` with error code `10`

## Topography
- [Route 53](https://console.aws.amazon.com/route53/v2/hostedzones#ListRecordSets/Z05536462C01YTPKRNSZ7):
  - NS Records:
    - mapped Nameservers from [Namecheap](https://ap.www.namecheap.com/Domains/DomainControlPanel/motoko.ai/domain/) to Route 53 Nameservers above
    - when validating ownership with AWS, remove name suffix `motoko.ai` for
      CNAME records because Namecheap automatically appends it
  - A Records:
    - motoko.ai:
      - mapped to this [CloudFront
        distribution](https://console.aws.amazon.com/cloudfront/home#distribution-settings:E2CR4IH7H1BW7N)
        - re-routes traffic from motoko.ai/graphql to api.motoko.ai/graphql
        - re-routes traffic from motoko.ai/install/* to
          [S3 bucket](https://console.aws.amazon.com/s3/buckets/motoko-prod-mobile/?region=us-west-1)
        - re-routes traffic from motoko.ai/* to S3 bucket
          [motoko-prod-www](https://console.aws.amazon.com/s3/buckets/motoko-prod-www/?region=us-west-1&tab=overview):
          - allows access by OAI (Origin Access Identity) to CloudFront
            distribution in [bucket policy](https://console.aws.amazon.com/s3/buckets/motoko-prod-www/?region=us-west-1&tab=permissions)
    - api.motoko.ai:
      - mapped to API Gateway
        [api.motoko.ai](https://us-west-1.console.aws.amazon.com/apigateway/home?region=us-west-1#/apis/plot4b3ymh/resources/pmgogvsld8):
        - edge-optimized gateway has an AWS managed CloudFront distribution
          that is mapped to the custom domain api.motoko.ai
          [here](https://us-west-1.console.aws.amazon.com/apigateway/main/publish/domain-names?domain=api.motoko.ai&region=us-west-1),
          which is mapped to the `prod` stage of the gateway:
          - api.motoko.ai/graphql is mapped to the Lambda function
            [motoko-graphql-prod](https://us-west-1.console.aws.amazon.com/lambda/home?region=us-west-1#/functions/motoko-graphql-prod?tab=configuration)
    - dev.motoko.ai:
      - mapped to this [CloudFront
        distribution](https://console.aws.amazon.com/cloudfront/home#distribution-settings:E1O86QQ54GNZCY)
        - re-routes traffic from dev.motoko.ai/graphql to
          api.dev.motoko.ai/graphql
        - re-routes traffic from motoko.ai/install/* to
          [S3 bucket](https://console.aws.amazon.com/s3/buckets/motoko-dev-mobile/?region=us-west-1)
        - re-routes traffic from dev.motoko.ai/* to S3 bucket
          [motoko-dev-www](https://console.aws.amazon.com/s3/buckets/motoko-dev-www/?region=us-west-1&tab=overview):
          - allows access by OAI (Origin Access Identity) to CloudFront
            distribution in [bucket
            policy](https://console.aws.amazon.com/s3/buckets/motoko-dev-www/?region=us-west-1&tab=permissions)
    - dev.api.motoko.ai:
      - mapped to API Gateway
        [dev.api.motoko.ai](https://us-west-1.console.aws.amazon.com/apigateway/home?region=us-west-1#/apis/cxcbzd3q0d/resources/gomvi9ciy9):
        - edge-optimized gateway has an AWS managed CloudFront distribution
          that is mapped to the custom domain api.dev.motoko.ai
          [here](https://us-west-1.console.aws.amazon.com/apigateway/main/publish/domain-names?domain=api.dev.motoko.ai&region=us-west-1),
          which is mapped to the `dev` stage of the gateway:
          - dev.api.motoko.ai/graphql is mapped to the Lambda function
            [motoko-graphql-dev](https://us-west-1.console.aws.amazon.com/lambda/home?region=us-west-1#/functions/motoko-graphql-dev?tab=configuration)
