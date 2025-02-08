# AWS Lambda template

## Prerequisites

- Recent Rust toolchain
- [`cargo lambda`](https://www.cargo-lambda.info/guide/installation.html)
- AWS account / IAM role with enough permissions ( at least `"lambda:CreateFunction"`, `"lambda:UpdateFunctionCode"`, `"lambda:GetFunction"` )
- AWS Local Credentials ( e.g. set up by `aws configure` )

In addition, on other than Linux, [`zig`](https://ziglang.org) or [`cross`](https://crates.io/crates/cross) is required for cross compiling ( see **Deploy** ) .

## Setup

```sh
cargo lambda new ＜project dir＞ --template https://github.com/ohkami-rs/ohkami-templates
> Do you enable Function URL?

cd ＜project dir＞
```

Ohkami supports both `Function URLs` and `API Gateway`. The prompt configures if automatically create Function URL of this Lambda ( default: `true` (just Enter) ) by `CargoLambda.toml` ( see https://www.cargo-lambda.info/guide/configuration.html for details ).

## Local dev

```sh
cargo lambda watch
```

See https://www.cargo-lambda.info/commands/watch.html for detailed document.

## Deploy

### Build

`cargo lambda deploy` **does not** build by itself and have no bones about shipping old binary if you forget to build latest source code.

```sh
cargo lambda build --release
```

#### cross compiling

We need to build source code into Linux binary running on Lambda.

* Without flag ( as above ), `cargo lambda` tries using [Zig](https://ziglang.org) for cross compiling.
* With `--compiler cargo` **on Linux**, no additional tool is required and directly built.
* With `--compiler cross`, [Cross](https://crates.io/crates/cross) is used.

See https://www.cargo-lambda.info/guide/cross-compiling.html for more detail.

### Deploy

```sh
cargo lambda deploy
```

If you're forgetful, always

```
cargo lambda build ＜flags＞ && cargo lambda deploy ＜flags＞
```

is recommended.

#### credentials

This refers to your local AWS credentials, typically configured by `aws configure` command.

#### flags

A lot of configurations can be overrided via flags ( for example multiple profiles by `--profile` ) , especially:

* `--role ＜arn-of-a-iam-role＞` may be required after the role is created.
* `--enable-function-url` ( or `--disable-function-url` ) overrides the prompt configuration.

See https://www.cargo-lambda.info/commands/deploy.html for details.
