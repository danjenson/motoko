use clap::{
    crate_authors, crate_description, crate_name, crate_version, App,
    AppSettings, Arg, ArgMatches, SubCommand,
};
use std::fs;
use std::process::{exit, Command, Stdio};
use which::which;

// TODO: danj BRANCH???

fn main() {
    if !run_from(".", "git", &["config", "--get", "remote.origin.url"])
        .ends_with("motoko.git")
    {
        quit("must be run from the 'motoko' git repository!");
    }
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(devops_subcommand("build"))
        .subcommand(devops_subcommand("deploy"))
        .subcommand(test_subcommand())
        .get_matches();
    match args.subcommand() {
        ("build", args) => build(args.expect("missing a build target!")),
        ("deploy", args) => deploy(args.expect("missing deploy target!")),
        ("test", args) => test(args.expect("missing test arguments!")),
        _ => quit("invalid subcommand!"),
    }
}

fn run_from(from: &str, cmd: &str, args: &[&str]) -> String {
    ensure_has(cmd);
    return String::from_utf8(
        Command::new(cmd)
            .args(args)
            .current_dir(from)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .output()
            .unwrap()
            .stdout,
    )
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
    eprintln!("{}", s);
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

fn test_subcommand<'a>() -> App<'a, 'a> {
    return SubCommand::with_name("test")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("gql")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("dev")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(Arg::with_name("json_payload").required(true)),
                )
                .subcommand(
                    SubCommand::with_name("prod")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(Arg::with_name("json_payload").required(true)),
                ),
        );
}

fn build(args: &ArgMatches<'_>) {
    ensure_on_clean_dev_or_prod_branch();
    match args.subcommand() {
        ("build-image", _) => build_build_image(),
        ("frontend", Some(args)) => build_frontend(args),
        ("backend", Some(args)) => build_backend(args),
        _ => quit("invalid build target!"),
    }
}

fn ensure_on_clean_dev_or_prod_branch() {
    if !["dev".to_owned(), "prod".to_owned()].contains(&run_from(
        ".",
        "git",
        &["branch", "--show-current"],
    )) {
        quit("command can only be run from the 'dev' or 'prod' branches");
    }
}

fn build_build_image() {
    run_from(".", "docker", &["build", "-t", "motoko", "."]);
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
            "--bundle",
            "build/app/outputs/bundle/release/app-release.apks",
        ],
    );
}

fn build_frontend_ios() {
    quit("build frontend iOS is not yet implemented!");
}

fn build_frontend_web() {
    run_from("frontend", "flutter", &["channel", "beta"]);
    run_from("frontend", "flutter", &["upgrade"]);
    run_from("frontend", "flutter", &["config", "--enable-web"]);
    run_from("frontend", "flutter", &["build", "web"]);
}

fn build_backend(args: &ArgMatches<'_>) {
    match args.value_of("function") {
        Some(name) => build_backend_function(name),
        _ => build_all_backend_functions(),
    }
}

fn build_backend_function(name: &str) {
    run_from("backend", "cargo", &["build", "--bin", name]);
}

fn build_all_backend_functions() {
    run_from("backend/rs", "cargo", &["build", "--bins"]);
}

fn deploy(args: &ArgMatches<'_>) {
    ensure_on_clean_dev_or_prod_branch();
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
    match args.value_of("function") {
        Some("android") => deploy_frontend_android(),
        Some("ios") => deploy_frontend_ios(),
        Some("web") => deploy_frontend_web(),
        Some(&_) | None => {
            deploy_frontend_android();
            deploy_frontend_web()
        }
    }
}

fn deploy_frontend_android() {
    //
    // - aws s3 rm s3://${S3_BUCKET} --recursive
    // - aws s3 cp frontend/build/web s3://${S3_BUCKET} --recursive
    // TODO
}

fn deploy_frontend_ios() {
    quit("deploy frontend ios is not yet implemented!");
}

fn deploy_frontend_web() {
    // TODO
}

fn deploy_backend(args: &ArgMatches<'_>) {
    match args.value_of("function") {
        Some(name) => deploy_backend_function(name),
        _ => deploy_all_backend_functions(),
    }
}

fn deploy_backend_function(name: &str) {
    // TODO
}

fn deploy_all_backend_functions() {
    // TODO
}

fn test(args: &ArgMatches<'_>) {
    // TODO
}
