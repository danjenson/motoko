use clap::{
    crate_authors, crate_description, crate_name, crate_version, App,
    AppSettings, Arg, ArgMatches, SubCommand,
};
use std::fs;
use std::process::{exit, Command, Stdio};
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
                ),
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
}

fn ensure_in_repo(name: &str) {
    if current_repo() != name {
        quit(&format!("must be run from the '{}' git repository", name));
    }
}

fn current_repo() -> String {
    return run_from(".", "git", &["config", "--get", "remote.origin.url"])
        .rsplit("/")
        .next()
        .unwrap()
        .split(".")
        .next()
        .unwrap()
        .into();
}

fn run_from(from: &str, cmd: &str, args: &[&str]) -> String {
    ensure_has(cmd);
    eprintln!("[ {:>15} ] {} {}", from, cmd, args.join(" "),);
    let output = Command::new(cmd)
        .args(args)
        .current_dir(from)
        .stdout(Stdio::piped())
        .output();
    if output.is_err() {
        quit(&format!(
            "{} {:?} failed from directory {}",
            cmd,
            args.join(" "),
            from
        ));
    }
    return String::from_utf8(output.unwrap().stdout)
        .unwrap()
        .trim()
        .into();
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
    return SubCommand::with_name(name)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("build-image"))
        .subcommand(
            SubCommand::with_name("frontend").arg(
                Arg::with_name("target")
                    .help("i.e. android, ios, web (default: all)")
                    .required(false),
            ),
        )
        .subcommand(
            SubCommand::with_name("backend").arg(
                Arg::with_name("function_name")
                    .help("i.e. graphql (default: all)")
                    .required(false),
            ),
        );
}

fn test(args: &ArgMatches<'_>) {
    match args.subcommand() {
        ("gql", Some(args)) => test_gql(args),
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
    must_be_on(&["dev", "prod"]);
    ensure_clean();
    match args.subcommand() {
        ("build-image", _) => build_build_image(),
        ("frontend", Some(args)) => build_frontend(args),
        ("backend", Some(args)) => build_backend(args),
        _ => quit("invalid build target!"),
    }
}

fn must_be_on(branches: &[&str]) {
    if !branches
        .iter()
        .map(|b| b.to_string())
        .collect::<String>()
        .contains(&current_branch())
    {
        quit("command can only be run from the 'dev' or 'prod' branches");
    }
}

fn current_branch() -> String {
    return run_from(".", "git", &["branch", "--show-current"]);
}

fn ensure_clean() {
    if !run_from(".", "git", &["--porcelain"]).is_empty() {
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
        Some("web") => build_frontend_web(),
        Some(&_) | None => {
            build_frontend_android();
            // TODO(danj): add iOS
            build_frontend_web()
        }
    }
}

fn build_frontend_android() {
    run_from(
        "frontend",
        "flutter",
        &[
            "build",
            "appbundle",
            "--release",
            "--target-platform",
            "android-arm,android-arm64,android-x86",
        ],
    );
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

fn build_frontend_web() {
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
    run_from(
        ".",
        "rustup",
        &["target", "add", "x86_64-unknown-linux-musl"],
    );
    run_from("backend/rs", "cargo", &["test"]);
    run_from(
        "backend/rs",
        "cargo",
        &[
            "build",
            "--release",
            "--target",
            "x86_64-unknown-linux-musl",
            "--bin",
            name,
        ],
    );
}

fn build_all_backend_functions() {
    run_from(
        ".",
        "rustup",
        &["target", "add", "x86_64-unknown-linux-musl"],
    );
    run_from("backend/rs", "cargo", &["test"]);
    run_from(
        "backend/rs",
        "cargo",
        &[
            "build",
            "--release",
            "--target",
            "x86_64-unknown-linux-musl",
        ],
    );
}

fn deploy(args: &ArgMatches<'_>) {
    must_be_on(&["dev", "prod"]);
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
        Some("web") => deploy_frontend_web(),
        Some(&_) | None => {
            // TODO(danj): deploy Android/iOS
            deploy_frontend_web()
        }
    }
}

fn deploy_frontend_android() {
    quit("deploying Android frontend is not yet supported!");
}

fn deploy_frontend_ios() {
    quit("deploying iOS frontend is not yet supported!");
}

fn deploy_frontend_web() {
    let s3_bucket = format!("s3://{}-{}-www", current_repo(), current_branch());
    run_from("frontend", "aws", &["s3", "rm", &s3_bucket, "--recursive"]);
    run_from(
        "frontend",
        "aws",
        &["s3", "cp", "build/web", &s3_bucket, "--recursive"],
    );
    // TODO(danj): invalidate CloudFront distribution cache
}

fn deploy_backend(args: &ArgMatches<'_>) {
    match args.value_of("function") {
        Some(name) => deploy_backend_function(name),
        _ => deploy_all_backend_functions(),
    }
}

fn deploy_backend_function(name: &str) {
    build_backend_function(name);
    run_from(
        ".",
        "rustup",
        &["target", "add", "x86_64-unknown-linux-musl"],
    );
    run_from("backend/rs", "cargo", &["test"]);
    run_from(
        "backend/rs",
        "cargo",
        &[
            "build",
            "--release",
            "--target",
            "x86_64-unknown-linux-musl",
            "--bin",
            name,
        ],
    );
    // TODO deploy
}

fn deploy_all_backend_functions() {
    build_all_backend_functions();
    // TODO deploy
}
