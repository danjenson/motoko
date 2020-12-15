use clap::{load_yaml, App, ArgMatches};
use regex::Regex;
use std::{
    env,
    fs::{create_dir_all, read_dir, remove_file, File},
    io::{BufRead, BufReader, Write},
    os::unix::fs::symlink,
    path::Path,
    process::{exit, Command, ExitStatus, Stdio},
};
use which::which;

fn main() {
    ensure_in_repo("motoko");
    set_dir_to_git_root();
    let yaml_args = load_yaml!("args.yaml");
    let args = App::from(yaml_args).get_matches();
    match args.subcommand() {
        Some(("build", args)) => build(args),
        Some(("deploy", args)) => deploy(args),
        Some(("install", args)) => install(args),
        Some(("run", args)) => run(args),
        _ => quit("invalid subcommand!"),
    }
    eprintln!("\nsuccess!\n");
}

fn ensure_in_repo(name: &str) {
    // `current_repo()` doesn't work inside CodeBuild because the entry point
    // of the build script is in a shallow copy of the repo with no config
    if current_repo() != name && !is_cloudbuild() {
        quit(&format!("must be run from the '{}' git repository", name));
    }
}

fn is_cloudbuild() -> bool {
    env::var("CODEBUILD_BUILD_ARN").is_ok()
}

fn current_repo() -> String {
    let cmd = "git";
    let args = &["config", "--get", "remote.origin.url"];
    if !exit_status_from(".", cmd, args).success() {
        quit("must run from a git repository");
    }
    run_from(".", cmd, args)
        .rsplit('/')
        .next()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .into()
}

fn exit_status_from(from: &str, cmd: &str, args: &[&str]) -> ExitStatus {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(from)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status();
    if status.is_err() {
        quit_cmd_from(from, cmd, args);
    }
    status.unwrap()
}

fn quit_cmd_from(from: &str, cmd: &str, args: &[&str]) {
    quit(&format!(
        "[ {:>15} ] {} {} <===== FAILED",
        from,
        cmd,
        args.join(" "),
    ));
}

fn quit(s: &str) {
    eprintln!("\nstderr:\n\n{}\n\nfailed!", s);
    exit(1);
}

fn run_from(from: &str, cmd: &str, args: &[&str]) -> String {
    ensure_has(cmd);
    eprintln!("[ {:>15} ] {} {}", from, cmd, args.join(" "),);
    let res = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(from)
        .output();
    if res.is_err() {
        quit_cmd_from(from, cmd, args);
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

fn set_dir_to_git_root() {
    env::set_current_dir(run_from(
        ".",
        "git",
        &["rev-parse", "--show-toplevel"],
    ))
    .expect("unable to set directory to git root!");
}

fn build(args: &ArgMatches) {
    match args.subcommand() {
        Some(("android", args)) => build_android(args),
        Some(("build-image", _)) => build_build_image(),
        Some(("graphql", _)) => {
            build_rust_lambda("backend/rs/graphql", "graphql")
        }
        Some(("garbage-collect", _)) => {
            build_rust_lambda("backend/rs/graphql", "garbage-collect")
        }
        Some(("ios", _)) => build_ios(),
        Some(("sam", _)) => build_sam(),
        Some(("web", _)) => build_web(),
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
            "command can only be run from the following branches: {:?}",
            branches
        ));
    }
}

fn current_branch() -> String {
    run_from(".", "git", &["branch", "--show-current"])
}

fn build_android(args: &ArgMatches) {
    match args.subcommand() {
        Some(("apk", _)) => build_android_apks(),
        Some(("bundle", _)) => build_android_bundle(),
        _ => quit("must specify either 'apk' or 'bundle'"),
    }
}

fn ensure_clean(path: &str) {
    // intermediate build files are not ignored by git in cloudbuild
    if !run_from(".", "git", &["status", "--porcelain", path]).is_empty()
        && !is_cloudbuild()
    {
        quit(&format!("directory '{}' is not clean", path));
    }
}

fn build_android_apks() {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean("frontend");
    if is_cloudbuild() {
        setup_android_keystore();
    }
    run_from("frontend", "flutter", &["clean"]);
    run_from(
        "frontend",
        "flutter",
        &[
            "build",
            "apk",
            &build_tier_flag(),
            "--release",
            "--split-per-abi",
        ],
    );
}

fn build_tier_flag() -> String {
    format!("--dart-define=TIER={}", current_branch())
}

fn build_android_bundle() {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean("frontend");
    if is_cloudbuild() {
        setup_android_keystore();
    }
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

fn build_build_image() {
    run_from("build_image", "docker", &["build", "-t", "motoko", "."]);
}

fn build_and_deploy_rust_lambda(path: &str, bin_name: &str) {
    build_rust_lambda(path, bin_name);
    deploy_rust_lambda(path, bin_name);
}

fn build_rust_lambda(path: &str, bin_name: &str) {
    // TODO(danj): should this be restricted?
    // ensure_on_branch(&["dev", "prod"]);
    // ensure_clean(path);
    run_from(
        ".",
        "rustup",
        &["target", "add", "x86_64-unknown-linux-musl"],
    );
    if !is_cloudbuild() {
        // TODO(danj): create test databases for cloudbuild tests
        run_from(path, "cargo", &["test"]);
    }
    env::set_var("RUST_BACKTRACE", "1");
    run_from(
        path,
        "cargo",
        &[
            "build",
            "--release",
            "--target",
            "x86_64-unknown-linux-musl",
            "--bin",
            bin_name,
        ],
    );
}

fn deploy_rust_lambda(path: &str, bin_name: &str) {
    // TODO(danj): should this be restricted?
    // ensure_on_branch(&["dev", "prod"]);
    let branch = if current_branch() == "prod" {
        "prod"
    } else {
        "dev"
    };
    let function_name = &format!("{}-{}-{}", current_repo(), bin_name, branch);
    let build_dir = "target/x86_64-unknown-linux-musl/release";
    let binary_path = &format!("{}/{}", build_dir, bin_name);
    let binary_bootstrap_path = &format!("{}/{}", build_dir, "bootstrap");
    let binary_bootstrap_path_zip =
        &format!("{}/{}", build_dir, "bootstrap.zip");
    let fileb_binary_bootstrap_path_zip =
        &format!("fileb://{}", binary_bootstrap_path_zip);
    run_from(path, "cp", &[binary_path, binary_bootstrap_path]);
    run_from(
        path,
        "zip",
        &["-j", binary_bootstrap_path_zip, binary_bootstrap_path],
    );
    if lambda_exists(function_name) {
        run_from(
            path,
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
            path,
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
                "--timeout",
                "60", // 1 minute
            ],
        );
    }
}

fn lambda_exists(name: &str) -> bool {
    exit_status_from(
        ".",
        "aws",
        &["lambda", "get-function", "--function-name", name],
    )
    .success()
}

fn build_ios() {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean("frontend");
    quit("building iOS is not yet supported!");
}

fn build_sam() {
    run_from("backend", "sam", &["build", "-p"]);
}

fn build_web() {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean("frontend");
    run_from("frontend", "flutter", &["channel", "beta"]);
    run_from("frontend", "flutter", &["upgrade"]);
    run_from("frontend", "flutter", &["config", "--enable-web"]);
    run_from("frontend", "flutter", &["clean"]);
    run_from("frontend", "flutter", &["build", "web", "--release"]);
}

fn deploy(args: &ArgMatches) {
    match args.subcommand() {
        Some(("all", _)) => deploy_all(),
        Some(("android", args)) => deploy_android(args),
        Some(("build-image", _)) => deploy_build_image(),
        Some(("garbage-collect", _)) => {
            deploy_rust_lambda("backend/rs/graphql", "garbage-collect")
        }
        Some(("graphql", _)) => {
            deploy_rust_lambda("backend/rs/graphql", "graphql")
        }
        Some(("ios", _)) => deploy_ios(),
        Some(("infer-datatypes", _)) => deploy_python_lambda("infer-datatypes"),
        Some(("invalidate-cache", _)) => {
            deploy_python_lambda("invalidate-cache")
        }
        Some(("uri-to-sql-db", _)) => deploy_python_lambda("uri-to-sql-db"),
        Some(("last-commit", _)) => deploy_last_commit(),
        Some(("web", _)) => deploy_web(),
        _ => quit("invalid deploy target!"),
    }
}

fn deploy_all() {
    ensure_on_branch(&["dev", "prod"]);
    build_and_deploy_rust_lambda("backend/rs/graphql", "graphql");
    build_and_deploy_rust_lambda("backend/rs/graphql", "garbage-collect");
    deploy_python_lambda("invalidate-cache");
    deploy_python_lambda("infer-datatypes");
    deploy_python_lambda("uri-to-sql-db");
    build_android_apks();
    deploy_android_apks();
    build_web();
    deploy_web();
}

fn deploy_android(args: &ArgMatches) {
    ensure_on_branch(&["dev", "prod"]);
    match args.subcommand() {
        Some(("apk", _)) => deploy_android_apks(),
        Some(("bundle", _)) => deploy_android_bundle(),
        _ => quit("must specify either 'apk' or 'bundle'"),
    }
}

fn deploy_android_apks() {
    ensure_on_branch(&["dev", "prod"]);
    let s3_bucket =
        format!("s3://{}-{}-mobile", current_repo(), current_branch());
    run_from("frontend", "aws", &["s3", "rm", &s3_bucket, "--recursive"]);
    run_from(
        "frontend",
        "aws",
        &[
            "s3",
            "cp",
            "build/app/outputs/apk/release/app-arm64-v8a-release.apk",
            &format!("{}/install/android", s3_bucket),
        ],
    );
    invalidate_cache();
}

fn infer_datatypes(args: &ArgMatches) {
    let output_file = "/tmp/infer-datatypes.json";
    run_from(
        ".",
        "aws",
        &[
            "lambda",
            "invoke",
            "--cli-binary-format",
            "raw-in-base64-out",
            "--function-name",
            "motoko-infer-datatypes",
            "--payload",
            &format!("{{ \"uri\": \"{}\" }}", args.value_of("uri").unwrap()),
            output_file,
        ],
    );
    let output = std::fs::read_to_string(output_file).unwrap();
    let value: serde_json::Value = serde_json::from_str(&output).unwrap();
    println!("{:?}", value);
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

fn deploy_android_bundle() {
    quit("deploying Android bundle is not yet supported!");
}

fn deploy_build_image() {
    let credentials = run_from(
        ".",
        "aws",
        &["ecr", "get-login-password", "--region", "us-west-1"],
    );
    run_from(
        ".",
        "docker",
        &[
            "login",
            "--username",
            "AWS",
            "--password",
            &credentials,
            "902096072945.dkr.ecr.us-west-1.amazonaws.com/motoko:latest",
        ],
    );
    run_from(
        ".",
        "docker",
        &[
            "tag",
            "motoko:latest",
            "902096072945.dkr.ecr.us-west-1.amazonaws.com/motoko:latest",
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

fn deploy_python_lambda(name: &str) {
    let dir = format!("backend/py/{}", name);
    // TODO(danj): should this only be on dev and prod branches?
    // ensure_on_branch(&["dev", "prod"]);
    // ensure_clean(&dir);
    let zip_path = format!("/tmp/{}.zip", name);
    let fileb_zip_path = &format!("fileb://{}", zip_path);
    let reqs = format!("{}/requirements.txt", dir);
    if Path::new(&reqs).exists() {
        let deps = format!("{}/deps", &dir);
        run_from(".", "rm", &["-rf", &deps]);
        run_from(".", "pip", &["install", "-r", &reqs, "--target", &deps]);
        run_from(&deps, "zip", &["-r9", &zip_path, "."]);
        run_from(&dir, "zip", &["-g", &zip_path, "main.py"]);
    } else {
        run_from(&dir, "zip", &["-r9", &zip_path, "main.py"]);
    }
    let function_name = format!("motoko-{}", name);
    if lambda_exists(&function_name) {
        run_from(
            ".",
            "aws",
            &[
                "lambda",
                "update-function-code",
                "--function-name",
                &function_name,
                "--zip-file",
                fileb_zip_path,
                "--cli-connect-timeout",
                "10000",
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
                &function_name,
                "--handler",
                "main.lambda_handler",
                "--zip-file",
                fileb_zip_path,
                "--runtime",
                "python3.8",
                "--role",
                "arn:aws:iam::902096072945:role/motoko-lambda",
                "--timeout",
                "300", // 5 minutes
                "--cli-connect-timeout",
                "10000",
            ],
        );
    }
}

fn deploy_ios() {
    ensure_on_branch(&["dev", "prod"]);
    quit("deploying iOS frontend is not yet supported!");
}

fn deploy_last_commit() {
    ensure_on_branch(&["dev", "prod"]);
    let modified_files =
        &run_from(".", "git", &["diff", "--name-only", "HEAD", "HEAD~1"]);
    let frontend = Regex::new("(^|[[:^alpha:]])frontend").unwrap();
    let graphql = Regex::new("(^|[[:^alpha:]])backend/rs/graphql").unwrap();
    let garbage_collect =
        Regex::new("(^|[[:^alpha:]])backend/rs/garbage-collect").unwrap();
    let invalidate_cache =
        Regex::new("(^|[[:^alpha:]])backend/py/invalidate-cache").unwrap();
    let infer_datatypes =
        Regex::new("(^|[[:^alpha:]])backend/py/infer-datatypes").unwrap();
    // execute in topological order
    if graphql.is_match(modified_files) {
        build_and_deploy_rust_lambda("backend/rs/graphql", "graphql");
    }
    if garbage_collect.is_match(modified_files) {
        build_and_deploy_rust_lambda("backend/rs/graphql", "garbage-collect");
    }
    if invalidate_cache.is_match(modified_files) {
        deploy_python_lambda("invalidate-cache");
    }
    if infer_datatypes.is_match(modified_files) {
        deploy_python_lambda("infer-datatypes");
    }
    if frontend.is_match(modified_files) {
        build_android_apks();
        deploy_android_apks();
        build_web();
        deploy_web();
    }
}

fn deploy_web() {
    ensure_on_branch(&["dev", "prod"]);
    let s3_bucket = format!("s3://{}-{}-www", current_repo(), current_branch());
    run_from("frontend", "aws", &["s3", "rm", &s3_bucket, "--recursive"]);
    run_from(
        "frontend",
        "aws",
        &["s3", "cp", "build/web", &s3_bucket, "--recursive"],
    );
    invalidate_cache();
}

fn install(args: &ArgMatches) {
    match args.subcommand() {
        Some(("android", _)) => install_android(),
        Some(("aws", _)) => install_aws(),
        Some(("flutter", _)) => install_flutter(),
        Some(("git-hooks", _)) => install_git_hooks(),
        Some(("ios", _)) => install_ios(),
        _ => quit("invalid install target!"),
    }
}

fn install_android() {
    run_from("frontend", "flutter", &["install"]);
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
    if which("sam").is_err() {
        // NOTE: also installs docker
        run_from(".", "yay", &["-Sy", "--noconfirm", "aws-sam-cli"]);
    }
}

fn install_flutter() {
    // source: https://flutter.dev/docs/get-started/install/linux
    if which("flutter").is_err() {
        run_from(
            ".",
            "git",
            &[
                "clone",
                "https://github.com/flutter/flutter.git",
                "/data/repos/flutter",
                "-b",
                "stable",
                "--depth",
                "1",
            ],
        );
    }
    if which("android-studio").is_err() {
        run_from(".", "yay", &["-Sy", "--noconfirm", "android-studio"]);
    }
}

fn install_git_hooks() {
    for res in read_dir("hooks").expect("couldn't list files in hooks dir") {
        let entry = res.expect("unable to read directory entry");
        let src = format!(
            "../../{}",
            entry.path().into_os_string().into_string().unwrap()
        );
        let dest =
            format!(".git/hooks/{}", entry.file_name().into_string().unwrap());
        symlink(&src, &dest).unwrap();
    }
}

fn install_ios() {
    quit("installing iOS is not currently supported!");
}

fn run(args: &ArgMatches) {
    match args.subcommand() {
        Some(("connect-to-aws-db", _)) => connect_to_aws_db(),
        Some(("frontend", args)) => run_frontend(args),
        Some(("graphql", args)) => graphql(args),
        Some(("infer-datatypes", args)) => infer_datatypes(args),
        Some(("invalidate-cache", _)) => invalidate_cache(),
        Some(("reset-android-keystore", _)) => {
            reset_android_keystore();
            setup_android_keystore();
        }
        Some(("reset-databases", _)) => reset_databases(),
        Some(("upload-debug-android-keystore", _)) => {
            upload_debug_android_keystore();
        }
        Some(("upload-backend-dotenv", _)) => {
            upload_backend_dotenv();
        }
        Some(("setup-android-keystore", _)) => setup_android_keystore(),
        Some(("setup-backend-dotenv", _)) => setup_backend_dotenv(),
        _ => quit("invalid run target!"),
    }
}

fn connect_to_aws_db() {
    // TODO(danj): continuously pipe io
    dotenv::from_filename("backend/rs/graphql/.env").ok();
    let stdout = Command::new("psql")
        .args(&[
            "--host=motoko-free-tier.cpybpfl4z4kw.us-west-1.rds.amazonaws.com",
            "--port=5432",
            "--username=motoko",
            "--dbname=postgres",
        ])
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("unable to connect to db")
        .stdout
        .expect("unable to collect stdout");
    BufReader::new(stdout)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
}

fn run_frontend(args: &ArgMatches) {
    match args.subcommand() {
        Some(("android", _)) => run_frontend_on_android_emulator(),
        Some(("ios", _)) => run_frontend_on_ios_emulator(),
        Some(("web", _)) => run_frontend_on_web(),
        Some(("device", _)) => run_on_device(),
        _ => quit("invalid emulator!"),
    }
}

fn run_frontend_on_android_emulator() {
    if !exit_status_from(".", "flutter", &["doctor"]).success() {
        quit("run `flutter doctor` and resolve issues!");
    }
    run_from(
        ".",
        "flutter",
        &["emulators", "--create", "--name", "android"],
    );
    run_from(".", "flutter", &["emulators", "--launch", "android"]);
}

fn run_frontend_on_ios_emulator() {
    quit("emulating on iOS is not yet supported!");
}

fn run_frontend_on_web() {
    run_from("frontend", "flutter", &["channel", "beta"]);
    run_from("frontend", "flutter", &["upgrade"]);
    run_from("frontend", "flutter", &["config", "--enable-web"]);
}

fn run_on_device() {
    run_from("frontend", "flutter", &["run"]);
}

fn graphql(args: &ArgMatches) {
    match args.subcommand() {
        Some(("query", args)) => graphql_query(args),
        _ => quit("must specify either 'dev' or 'prod' tier"),
    }
}

fn graphql_query(args: &ArgMatches) {
    match args.subcommand() {
        Some(("lambda", args)) => graphql_query_lambda(args),
        _ => quit("must specify either 'dev' or 'prod' tier"),
    }
}

fn graphql_query_lambda(args: &ArgMatches) {
    match args.subcommand() {
        Some(("dev", args)) => _gql("https://dev.motoko.ai/graphql", args),
        Some(("prod", args)) => _gql("https://motoko.ai/graphql", args),
        _ => quit("must specify either 'dev' or 'prod' tier"),
    }
}

fn _gql(endpoint: &str, args: &ArgMatches) {
    match args.value_of("payload") {
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

fn reset_android_keystore() {
    let path = "/tmp/signing_key.jks";
    remove_file_if_exists(path);
    let pw = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    generate_android_keystore(&pw, path);
    put_binary_secret(path, "android_keystore");
    put_android_keystore_password(&pw);
    remove_file_if_exists(path);
}

fn remove_file_if_exists(path: &str) {
    let path = Path::new(path);
    if path.exists() {
        let message =
            format!("unable to remove file: {}", path.to_str().unwrap());
        remove_file(path).expect(&message);
    }
}

fn upload_debug_android_keystore() {
    let home_dir = dirs::home_dir()
        .expect("unable to get user's home directory!")
        .into_os_string()
        .into_string()
        .expect("unable to convert home directory into string for path!");
    let path = &format!("{}/.android/debug.keystore", home_dir);
    put_binary_secret(path, "debug_android_keystore");
}

fn generate_android_keystore(password: &str, path: &str) {
    run_from(
        ".",
        "keytool",
        &[
            "-genkey",
            "-dname",
            "cn=Daniel Jenson, o=motoko, c=US",
            "-keystore",
            path,
            "-keyalg",
            "RSA",
            "-keysize",
            "2048",
            "-validity",
            "10000",
            "-alias",
            "signing_key",
            "-keypass",
            password,
            "-storepass",
            password,
        ],
    );
}

fn put_binary_secret(path: &str, key: &str) {
    put_secret(key, SecretType::Binary, &format!("fileb://{}", path));
}

#[derive(Debug, Eq, PartialEq)]
enum SecretType {
    Binary,
    String,
}

fn put_secret(name: &str, secret_type: SecretType, value: &str) {
    let secret_flag = if secret_type == SecretType::Binary {
        "--secret-binary"
    } else {
        "--secret-string"
    };
    if secret_exists(name) {
        run_from(
            ".",
            "aws",
            &[
                "secretsmanager",
                "put-secret-value",
                "--secret-id",
                name,
                secret_flag,
                value,
            ],
        );
    } else {
        run_from(
            ".",
            "aws",
            &[
                "secretsmanager",
                "create-secret",
                "--name",
                name,
                secret_flag,
                value,
            ],
        );
    }
}

fn secret_exists(name: &str) -> bool {
    exit_status_from(
        ".",
        "aws",
        &["secretsmanager", "get-secret-value", "--secret-id", name],
    )
    .success()
}

fn put_android_keystore_password(pw: &str) {
    put_secret("android_keystore_password", SecretType::String, pw);
}

fn upload_backend_dotenv() {
    put_binary_secret("backend/.env", "backend_dotenv");
}

fn setup_backend_dotenv() {
    let dotenv = get_secret("backend_dotenv", SecretType::Binary);
    let dotenv_path_str = "backend/.env";
    let dotenv_path = Path::new(dotenv_path_str);
    let mut dotenv_file = File::create(dotenv_path)
        .unwrap_or_else(|_| panic!("unable to open {}", dotenv_path_str));
    dotenv_file.write_all(&dotenv).unwrap_or_else(|_| {
        panic!("unable to write dotenv to {}", dotenv_path_str)
    });
    remove_file_if_exists("backend/rs/graphql/.env");
    symlink("../../.env", "backend/rs/graphql/.env").unwrap();
}

fn setup_android_keystore() {
    let home_dir = dirs::home_dir()
        .expect("unable to get user's home directory!")
        .into_os_string()
        .into_string()
        .expect("unable to convert home directory into string for path!");
    let keystore_path_str =
        &format!("{}/.keys/motoko/android/signing_key.jks", home_dir);
    let debug_keystore_path_str =
        &format!("{}/.android/debug.keystore", home_dir);
    let keystore_path = Path::new(keystore_path_str);
    let debug_keystore_path = Path::new(debug_keystore_path_str);
    let keystore_dir = keystore_path.parent().unwrap();
    let debug_keystore_dir = debug_keystore_path.parent().unwrap();
    let keystore_dir_str = keystore_dir.to_string_lossy();
    let debug_keystore_dir_str = debug_keystore_dir.to_string_lossy();
    let key_properties_path = Path::new("frontend/android/key.properties");
    let key_properties_path_str = key_properties_path.to_string_lossy();
    create_dir_all(keystore_dir).unwrap_or_else(|_| {
        panic!("unable to create key directory: {}", keystore_dir_str)
    });
    create_dir_all(debug_keystore_dir).unwrap_or_else(|_| {
        panic!(
            "unable to create debug key directory: {}",
            debug_keystore_dir_str
        )
    });
    let ks = get_secret("android_keystore", SecretType::Binary);
    let dks = get_secret("debug_android_keystore", SecretType::Binary);
    let mut keystore = File::create(keystore_path)
        .unwrap_or_else(|_| panic!("unable to open {}", keystore_path_str));
    let mut debug_keystore =
        File::create(debug_keystore_path).unwrap_or_else(|_| {
            panic!("unable to open {}", debug_keystore_path_str)
        });
    keystore.write_all(&ks).unwrap_or_else(|_| {
        panic!("unable to write keystore to {}", keystore_path_str)
    });
    debug_keystore.write_all(&dks).unwrap_or_else(|_| {
        panic!(
            "unable to write debug keystore to {}",
            debug_keystore_path_str
        )
    });
    let pw = get_secret("android_keystore_password", SecretType::String);
    let mut key_properties =
        File::create(key_properties_path).unwrap_or_else(|_| {
            panic!("unable to open: {}", key_properties_path_str)
        });
    let key_properties_content = [
        &format!("storePassword={}", std::str::from_utf8(&pw).unwrap()),
        &format!("keyPassword={}", std::str::from_utf8(&pw).unwrap()),
        "keyAlias=signing_key",
        &format!("storeFile={}", keystore_path_str),
    ]
    .join("\n");
    key_properties
        .write_all(&key_properties_content.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "unable to write key properties file to {}",
                key_properties_path_str
            )
        });
}

fn get_secret(name: &str, secret_type: SecretType) -> Vec<u8> {
    let json_str = run_from(
        ".",
        "aws",
        &["secretsmanager", "get-secret-value", "--secret-id", name],
    );
    let v: serde_json::Value =
        serde_json::from_str(&json_str).expect("unable to parse json secret!");
    let value: Vec<u8>;
    if secret_type == SecretType::Binary {
        value = base64::decode(v["SecretBinary"].as_str().unwrap())
            .expect("unable to decode base64 binary secret!");
    } else {
        value = v["SecretString"].as_str().unwrap().as_bytes().to_owned();
    };
    value
}

fn reset_databases() {
    dotenv::from_filename("backend/rs/graphql/.env").ok();
    run_from("backend/rs/graphql", "sqlx", &["database", "reset"]);
    env::set_var("DATABASE_URL", env::var("DATA_DATABASE_URL").unwrap());
    run_from(
        "backend/rs/graphql",
        "sqlx",
        &["database", "reset", "--source", "data_migrations"],
    );
}
