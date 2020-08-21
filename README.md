# motoko
![motoko motorcycle](https://i.pinimg.com/originals/56/55/bb/5655bbf38aedf1ff44e926c190859c7b.png)
> I'll have my AI analyze the data.

## TODO
- docker image for CloudBuild
- build script for docker image
- lambda function for invalidating cloudfront distribution: https://medium.com/taptuit/automated-build-deploy-with-aws-codepipeline-f0714d62f61c
- apk hosting - S3?
- motoko.ai/privacy - flutter route?
- google auth submit for review
- backend queries and mutations
- error dialog
- get rights to Motoko font for mobile apps too
- motoko pypi sdk

## Deployment
- run `cargo build`
- build environment is specified by a [Dockerfile] and is pushed to AWS ECR
- push deploy/
- motoko.ai:
  - CodeBuild runs
    [buildspec-prod.yml](https://github.com/danjenson/motoko/blob/master/buildspec-prod.yml)
    for
    [motoko-prod](https://us-west-1.console.aws.amazon.com/codesuite/codebuild/projects/motoko-prod/details):
    - [Service Role](https://console.aws.amazon.com/iam/home?#/roles/codebuild-motoko-prod-service-role) has S3 access

## Setup
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 
- `./install_motoko_command`
- `motoko install aws`
- `aws configure`
  - [Users -> user -> Security Keys](https://console.aws.amazon.com/iam/home#/users)
  - [Regions](https://docs.aws.amazon.com/general/latest/gr/rande.html)
    (default to `us-west-1`, which is Northern California)
  - output format: `json`

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
