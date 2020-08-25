use clap::{load_yaml, App, ArgMatches};
use regex::Regex;
use std::fs::{create_dir_all, remove_file, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, ExitStatus, Stdio};
use which::which;

fn main() {
    ensure_in_repo("motoko");
    set_dir_to_git_root();
    let args = App::from(load_yaml!("args.yaml")).get_matches();
    match args.subcommand() {
        ("build", Some(args)) => build(args),
        ("deploy", Some(args)) => deploy(args),
        ("install", Some(args)) => install(args),
        ("run", Some(args)) => run(args),
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
    std::env::var("CODEBUILD_BUILD_ARN").is_ok()
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
    std::env::set_current_dir(run_from(
        ".",
        "git",
        &["rev-parse", "--show-toplevel"],
    ))
    .expect("unable to set directory to git root!");
}

fn build(args: &ArgMatches) {
    match args.subcommand() {
        ("android", Some(args)) => build_android(args),
        ("build-image", _) => build_build_image(),
        ("graphql", _) => build_graphql(),
        ("ios", _) => build_ios(),
        ("web", _) => build_web(),
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
        ("apk", _) => build_android_apks(),
        ("bundle", _) => build_android_bundle(),
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
        &["build", "apk", "--release", "--split-per-abi"],
    );
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
    run_from("motoko", "cargo", &["build", "--release"]);
    run_from(".", "cp", &["motoko/target/release/motoko", "build_image"]);
    run_from("build_image", "docker", &["build", "-t", "motoko", "."]);
    run_from(".", "rm", &["build_image/motoko"]);
}

fn build_graphql() {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean("backend/rs/gql");
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

fn build_ios() {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean("frontend");
    quit("building iOS is not yet supported!");
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
        ("all", _) => deploy_all(),
        ("android", Some(args)) => deploy_android(args),
        ("build-image", _) => deploy_build_image(),
        ("graphql", _) => deploy_graphql(),
        ("ios", _) => deploy_ios(),
        ("invalidate-cache", _) => deploy_invalidate_cache(),
        ("last-commit", _) => deploy_last_commit(),
        ("web", _) => deploy_web(),
        _ => quit("invalid deploy target!"),
    }
}

fn deploy_all() {
    ensure_on_branch(&["dev", "prod"]);
    build_graphql();
    deploy_graphql();
    deploy_invalidate_cache();
    build_android_apks();
    deploy_android_apks();
    build_web();
    deploy_web();
}

fn deploy_android(args: &ArgMatches) {
    ensure_on_branch(&["dev", "prod"]);
    match args.subcommand() {
        ("apk", _) => deploy_android_apks(),
        ("bundle", _) => deploy_android_bundle(),
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

fn deploy_graphql() {
    ensure_on_branch(&["dev", "prod"]);
    let dir = "backend/rs/gql";
    let function_name =
        &format!("{}-graphql-{}", current_repo(), current_branch());
    let build_dir = "target/x86_64-unknown-linux-musl/release";
    let binary_path = &format!("{}/graphql", build_dir);
    let binary_bootstrap_path = &format!("{}/{}", build_dir, "bootstrap");
    let binary_bootstrap_path_zip =
        &format!("{}/{}", build_dir, "bootstrap.zip");
    let fileb_binary_bootstrap_path_zip =
        &format!("fileb://{}", binary_bootstrap_path_zip);
    run_from(dir, "cp", &[binary_path, binary_bootstrap_path]);
    run_from(
        dir,
        "zip",
        &["-j", binary_bootstrap_path_zip, binary_bootstrap_path],
    );
    if lambda_exists(function_name) {
        run_from(
            dir,
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
            dir,
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

fn lambda_exists(name: &str) -> bool {
    exit_status_from(
        ".",
        "aws",
        &["lambda", "get-function", "--function-name", name],
    )
    .success()
}

fn deploy_invalidate_cache() {
    ensure_on_branch(&["dev", "prod"]);
    ensure_clean("backend/py/invalidate_cache.py");
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

fn deploy_ios() {
    ensure_on_branch(&["dev", "prod"]);
    quit("deploying iOS frontend is not yet supported!");
}

fn deploy_last_commit() {
    ensure_on_branch(&["dev", "prod"]);
    let modified_files =
        &run_from(".", "git", &["diff", "--name-only", "HEAD", "HEAD~1"]);
    let frontend = Regex::new("(^|[[:^alpha:]])frontend").unwrap();
    let graphql = Regex::new("(^|[[:^alpha:]])backend/rs/gql").unwrap();
    let invalidate_cache =
        Regex::new("(^|[[:^alpha:]])backend/py/invalidate_cache").unwrap();
    // execute in topological order
    if graphql.is_match(modified_files) {
        build_graphql();
        deploy_graphql();
    }
    if invalidate_cache.is_match(modified_files) {
        deploy_invalidate_cache();
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
        ("android", _) => install_android(),
        ("aws", _) => install_aws(),
        ("ios", _) => install_ios(),
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
}

fn install_ios() {
    quit("installing iOS is not currently supported!");
}

fn run(args: &ArgMatches) {
    match args.subcommand() {
        ("gql", Some(args)) => gql(args),
        ("invalidate-cache", _) => invalidate_cache(),
        ("reset-android-keystore", _) => {
            reset_android_keystore();
            setup_android_keystore();
        }
        ("setup-android-keystore", _) => setup_android_keystore(),
        _ => quit("invalid test target!"),
    }
}

fn gql(args: &ArgMatches) {
    match args.subcommand() {
        ("dev", Some(args)) => _gql("https://dev.motoko.ai/graphql", args),
        ("prod", Some(args)) => _gql("https://motoko.ai/graphql", args),
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
    let path = Path::new("/tmp/signing_key.jks");
    if path.exists() {
        remove_file(path).expect("unable to remove old temporary signing key!");
    }
    let pw = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    generate_android_keystore(&pw, &path);
    put_android_keystore(&path);
    put_android_keystore_password(&pw);
    remove_file(path).unwrap_or_else(|_| {
        panic!("unable to remove file: {}", path.to_string_lossy())
    });
}

fn generate_android_keystore(password: &str, path: &Path) {
    run_from(
        ".",
        "keytool",
        &[
            "-genkey",
            "-dname",
            "cn=Daniel Jenson, o=motoko, c=US",
            "-keystore",
            &path.to_string_lossy(),
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

fn put_android_keystore(path: &Path) {
    put_secret(
        "android_keystore",
        SecretType::Binary,
        &format!("fileb://{}", path.to_string_lossy()),
    );
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

fn setup_android_keystore() {
    let home_dir = dirs::home_dir()
        .expect("unable to get user's home directory!")
        .into_os_string()
        .into_string()
        .expect("unable to convert home directory into string for path!");
    let keystore_path_str =
        &format!("{}/.keys/motoko/android/signing_key.jks", home_dir);
    let keystore_path = Path::new(keystore_path_str);
    let keystore_dir = keystore_path.parent().unwrap();
    let keystore_dir_str = keystore_dir.to_string_lossy();
    let key_properties_path = Path::new("frontend/android/key.properties");
    let key_properties_path_str = key_properties_path.to_string_lossy();
    create_dir_all(keystore_dir).unwrap_or_else(|_| {
        panic!("unable to create key directory: {}", keystore_dir_str)
    });
    let ks = get_secret("android_keystore", SecretType::Binary);
    let mut keystore = OpenOptions::new()
        .create(true)
        .write(true)
        .open(keystore_path)
        .unwrap_or_else(|_| panic!("unable to open {}", keystore_path_str));
    keystore.write_all(&ks).unwrap_or_else(|_| {
        panic!("unable to write keystore to {}", keystore_path_str)
    });
    let pw = get_secret("android_keystore_password", SecretType::String);
    let mut key_properties = OpenOptions::new()
        .create(true)
        .write(true)
        .open(key_properties_path)
        .unwrap_or_else(|_| {
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
