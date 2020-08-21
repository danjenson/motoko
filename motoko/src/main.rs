use clap::{
    crate_authors, crate_description, crate_name, crate_version, App,
    AppSettings, Arg, ArgMatches, SubCommand,
};
use std::process::{exit, Command, ExitStatus};
use which::which;

fn main() {
    ensure_in_repo("motoko");
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("install")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("aws"))
                .subcommand(SubCommand::with_name("android")),
        )
        .subcommand(
            SubCommand::with_name("test")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("gql")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .subcommand(
                            SubCommand::with_name("dev")
                                .setting(AppSettings::ArgRequiredElseHelp)
                                .arg(
                                    Arg::with_name("json_payload")
                                        .required(true),
                                ),
                        )
                        .subcommand(
                            SubCommand::with_name("prod")
                                .setting(AppSettings::ArgRequiredElseHelp)
                                .arg(
                                    Arg::with_name("json_payload")
                                        .required(true),
                                ),
                        ),
                )
                .subcommand(SubCommand::with_name("invalidate-cache")),
        )
        .subcommand(devops_subcommand("build"))
        .subcommand(devops_subcommand("deploy"))
        .get_matches();
    match args.subcommand() {
        ("test", args) => test(args.expect("missing test arguments!")),
        ("install", args) => install(args.expect("missing install arguments!")),
        ("build", args) => build(args.expect("missing a build target!")),
        ("deploy", args) => deploy(args.expect("missing deploy target!")),
        _ => quit("invalid subcommand!"),
    }
    eprintln!("\n\n\nsuccess!\n");
}

fn ensure_in_repo(name: &str) {
    if current_repo() != name {
        quit(&format!("must be run from the '{}' git repository", name));
    }
}

fn current_repo() -> String {
    let cmd = "git";
    let args = &["config", "--get", "remote.origin.url"];
    if !Command::new(cmd)
        .args(args)
        .status()
        .expect("failed to run git!")
        .success()
    {
        quit("must run from a git repository");
    }
    run_from(".", cmd, args).rsplit("/").next().unwrap().into()
}

fn run_from(from: &str, cmd: &str, args: &[&str]) -> String {
    ensure_has(cmd);
    eprintln!("[ {:>15} ] {} {}", from, cmd, args.join(" "),);
    let res = Command::new(cmd).args(args).current_dir(from).output();
    if res.is_err() {
        quit("failed to execute");
    }
    let output = res.unwrap();
    if !output.status.success() {
        quit(&String::from_utf8(output.stderr).unwrap());
    }
    String::from_utf8(output.stdout).unwrap().trim().into()
}

fn ensure_has(binary: &str) {
    if which(binary).is_err() {
        quit(&format!("missing required binary: {}", binary));
    }
}

fn quit(s: &str) {
    eprintln!("\n{}\n", s);
    exit(1);
}

fn devops_subcommand(name: &str) -> App {
    SubCommand::with_name(name)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("build-image"))
        .subcommand(
            SubCommand::with_name("frontend").arg(
                Arg::with_name("target")
                    .help("i.e. android, ios, www (default: all)")
                    .required(false),
            ),
        )
        .subcommand(
            SubCommand::with_name("backend").arg(
                Arg::with_name("function")
                    .help("i.e. graphql (default: all)")
                    .required(false),
            ),
        )
}

fn test(args: &ArgMatches<'_>) {
    match args.subcommand() {
        ("gql", Some(args)) => test_gql(args),
        ("invalidate-cache", _) => invalidate_cache(),
        _ => quit("invalid test target!"),
    }
}

fn test_gql(args: &ArgMatches<'_>) {
    match args.subcommand() {
        ("dev", Some(args)) => gql("https://dev.motoko.ai/graphql", args),
        ("prod", Some(args)) => gql("https://motoko.ai/graphql", args),
        _ => quit("must specify either 'dev' or 'prod' tier"),
    }
}

fn gql(endpoint: &str, args: &ArgMatches<'_>) {
    // TODO(danj): cleanup
    match args.value_of("json_payload") {
        Some(payload) => {
            let client = reqwest::blocking::Client::new();
            let res = client
                .post(endpoint)
                .body(payload.to_owned())
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap()
                .text()
                .unwrap();
            println!("{}", res);
        }
        _ => quit("must provide a JSON payload"),
    }
}

fn invalidate_cache() {
    ensure_on_branch(&["dev", "prod"]);
    let cloudfront_distribution_id = if current_branch() == "dev" {
        "E1O86QQ54GNZCY"
    } else {
        "E2CR4IH7H1BW7N"
    };

    run_from(
        ".",
        "aws",
        &[
            "lambda",
            "invoke",
            "--cli-binary-format",
            "raw-in-base64-out",
            "--function-name",
            "motoko-invalidate-cache",
            "--payload",
            &format!(
                "{{ \"distribution_id\": \"{}\" }}",
                cloudfront_distribution_id
            ),
            "/tmp/invalidate-cache.json",
        ],
    );
}

fn install(args: &ArgMatches<'_>) {
    match args.subcommand() {
        ("aws", _) => install_aws(),
        ("android", _) => install_android(),
        ("ios", _) => install_ios(),
        _ => quit("invalid install target!"),
    }
}

fn install_aws() {
    if which("aws").is_err() {
        run_from(
            "/tmp",
            "curl",
            &[
                "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip",
                "-o",
                "awscliv2.zip",
            ],
        );
        run_from("/tmp", "unzip", &["awscliv2.zip"]);
        run_from("/tmp", "sudo", &["./aws/install"]);
    }
}

fn install_android() {
    build_frontend_android();
    run_from(
        "frontend",
        "bundletool",
        &[
            "install-apks",
            "--apks",
            "build/app/outputs/bundle/release/app-release.apks",
        ],
    );
}

fn install_ios() {
    build_frontend_ios();
    quit("installing iOS is not currently supported!");
}

fn build(args: &ArgMatches<'_>) {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean();
    match args.subcommand() {
        ("build-image", _) => build_build_image(),
        ("frontend", Some(args)) => build_frontend(args),
        ("backend", Some(args)) => build_backend(args),
        _ => quit("invalid build target!"),
    }
}

fn ensure_on_branch(branches: &[&str]) {
    if !branches
        .iter()
        .map(|b| b.to_string())
        .collect::<String>()
        .contains(&current_branch())
    {
        quit(&format!(
            "command can only be run from the following branches: {:#?}",
            branches
        ));
    }
}

fn current_branch() -> String {
    run_from(".", "git", &["branch", "--show-current"])
}

fn ensure_clean() {
    if !run_from(".", "git", &["status", "--porcelain"]).is_empty() {
        quit("branch is not clean");
    }
}

fn build_build_image() {
    run_from("build_image", "docker", &["build", "-t", "motoko", "."]);
}

fn build_frontend(args: &ArgMatches<'_>) {
    match args.value_of("target") {
        Some("android") => build_frontend_android(),
        Some("ios") => build_frontend_ios(),
        Some("www") => build_frontend_www(),
        Some(&_) | None => {
            // TODO(danj): add iOS
            build_frontend_android();
            build_frontend_www()
        }
    }
}

fn build_frontend_android() {
    run_from("frontend", "flutter", &["clean"]);
    run_from("frontend", "flutter", &["build", "appbundle", "--release"]);
    run_from(
        "frontend",
        "bundletool",
        &[
            "build-apks",
            "--overwrite",
            "--bundle=build/app/outputs/bundle/release/app-release.aab",
            "--output=build/app/outputs/bundle/release/app-release.apks",
        ],
    );
}

fn build_frontend_ios() {
    quit("building iOS frontend is not yet supported!");
}

fn build_frontend_www() {
    run_from("frontend", "flutter", &["channel", "beta"]);
    run_from("frontend", "flutter", &["upgrade"]);
    run_from("frontend", "flutter", &["config", "--enable-web"]);
    run_from("frontend", "flutter", &["build", "web", "--release"]);
}

fn build_backend(args: &ArgMatches<'_>) {
    match args.value_of("function") {
        Some(name) => build_backend_function(name),
        _ => build_all_backend_functions(),
    }
}

fn build_backend_function(name: &str) {
    match name {
        "graphql" => build_backend_function_graphql(),
        "invalidate-cache" => {} // python function doesn't need to be built
        _ => quit(&format!("invalid backend function name: {}", name)),
    }
}

fn build_backend_function_graphql() {
    run_from(
        ".",
        "rustup",
        &["target", "add", "x86_64-unknown-linux-musl"],
    );
    run_from("backend/rs/gql", "cargo", &["test"]);
    run_from(
        "backend/rs/gql",
        "cargo",
        &[
            "build",
            "--release",
            "--target",
            "x86_64-unknown-linux-musl",
        ],
    );
}

fn build_all_backend_functions() {
    build_backend_function_graphql()
}

fn deploy(args: &ArgMatches<'_>) {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean();
    match args.subcommand() {
        ("build-image", _) => deploy_build_image(),
        ("frontend", Some(args)) => deploy_frontend(args),
        ("backend", Some(args)) => deploy_backend(args),
        _ => quit("invalid deploy target!"),
    }
}

fn deploy_build_image() {
    let credentials =
        run_from(".", "aws", &["get-login-password", "--region", "us-west-1"]);
    run_from(
        ".",
        "docker",
        &[
            "tag",
            "motoko:latest",
            "902096072945.dkr.ecr.us-west-1.amazonaws.com/motoko:latest",
            &credentials,
        ],
    );
    run_from(
        ".",
        "docker",
        &[
            "push",
            "902096072945.dkr.ecr.us-west-1.amazonaws.com/motoko:latest",
        ],
    );
}

fn deploy_frontend(args: &ArgMatches<'_>) {
    match args.value_of("target") {
        Some("android") => deploy_frontend_android(),
        Some("ios") => deploy_frontend_ios(),
        Some("www") => deploy_frontend_www(),
        Some(&_) | None => {
            // TODO(danj): deploy iOS
            deploy_frontend_android();
            deploy_frontend_www();
        }
    }
}

fn deploy_frontend_android() {
    ensure_on_branch(&["dev"]);
    let s3_bucket =
        format!("s3://{}-{}-mobile", current_repo(), current_branch());
    run_from("frontend", "aws", &["s3", "rm", &s3_bucket, "--recursive"]);
    run_from(
        "frontend",
        "aws",
        &[
            "s3",
            "cp",
            "build/app/outputs/bundle/release/app-release.apks",
            &format!("{}/install/motoko.apk", s3_bucket),
        ],
    );
    invalidate_cache();
}

fn deploy_frontend_ios() {
    quit("deploying iOS frontend is not yet supported!");
}

fn deploy_frontend_www() {
    let s3_bucket = format!("s3://{}-{}-www", current_repo(), current_branch());
    run_from("frontend", "aws", &["s3", "rm", &s3_bucket, "--recursive"]);
    run_from(
        "frontend",
        "aws",
        &["s3", "cp", "build/web", &s3_bucket, "--recursive"],
    );
    invalidate_cache();
}

fn deploy_backend(args: &ArgMatches<'_>) {
    match args.value_of("function") {
        Some(name) => deploy_backend_function(name),
        _ => deploy_all_backend_functions(),
    }
}

fn deploy_backend_function(name: &str) {
    match name {
        "graphql" => deploy_backend_function_graphql(),
        "invalidate-cache" => deploy_backend_function_invalidate_cache(),
        _ => quit(&format!("invalid backend function name: {}", name)),
    }
}

fn deploy_backend_function_graphql() {
    let function_name =
        &format!("{}-graphql-{}", current_repo(), current_branch());
    let build_dir = "target/x86_64-unknown-linux-musl/release";
    let binary_path = &format!("{}/graphql", build_dir);
    let binary_bootstrap_path = &format!("{}/{}", build_dir, "bootstrap");
    let binary_bootstrap_path_zip =
        &format!("{}/{}", build_dir, "bootstrap.zip");
    let fileb_binary_bootstrap_path_zip =
        &format!("fileb://{}", binary_bootstrap_path_zip);
    run_from("backend/rs", "cp", &[binary_path, binary_bootstrap_path]);
    run_from(
        "backend/rs",
        "zip",
        &["-j", binary_bootstrap_path_zip, binary_bootstrap_path],
    );
    if lambda_exists(function_name) {
        run_from(
            "backend/rs/gql",
            "aws",
            &[
                "lambda",
                "update-function-code",
                "--function-name",
                function_name,
                "--zip-file",
                fileb_binary_bootstrap_path_zip,
            ],
        );
    } else {
        run_from(
            "backend/rs/gql",
            "aws",
            &[
                "lambda",
                "create-function",
                "--function-name",
                function_name,
                "--handler",
                "doesnt.matter",
                "--zip-file",
                fileb_binary_bootstrap_path_zip,
                "--runtime",
                "provided",
                "--role",
                "arn:aws:iam::902096072945:role/motoko-lambda",
                "--environment",
                "Variables={RUST_BACKTRACE=1}",
                "--tracing-config",
                "Mode=Active",
            ],
        );
    }
}

fn exit_status(cmd: &str, args: &[&str]) -> ExitStatus {
    let status = Command::new(cmd).args(args).status();
    if status.is_err() {
        quit(&format!("failed to run: {} {}", cmd, args.join(" ")));
    }
    status.unwrap()
}

fn lambda_exists(name: &str) -> bool {
    exit_status("aws", &["lambda", "get-function", "--function-name", name])
        .success()
}

fn deploy_backend_function_invalidate_cache() {
    let zip_path = "/tmp/invalidate_cache.py.zip";
    let fileb_zip_path = &format!("fileb://{}", zip_path);
    run_from(
        "backend/py",
        "zip",
        &["-j", zip_path, "invalidate_cache.py"],
    );
    let function_name = "motoko-invalidate-cache";
    if lambda_exists(function_name) {
        run_from(
            ".",
            "aws",
            &[
                "lambda",
                "update-function-code",
                "--function-name",
                function_name,
                "--zip-file",
                fileb_zip_path,
            ],
        );
    } else {
        run_from(
            ".",
            "aws",
            &[
                "lambda",
                "create-function",
                "--function-name",
                function_name,
                "--handler",
                "invalidate_cache.lambda_handler",
                "--zip-file",
                fileb_zip_path,
                "--runtime",
                "python3.8",
                "--role",
                "arn:aws:iam::902096072945:role/motoko-lambda",
            ],
        );
    }
}

fn deploy_all_backend_functions() {
    deploy_backend_function_graphql();
    deploy_backend_function_invalidate_cache();
}
