pub mod common_dirs; pub use common_dirs::*;

pub fn get_the_script_projects_folder() -> std::path::PathBuf { app_cache_folder().join("ScriptProjects") }
pub fn get_the_buildfolders_folder() -> std::path::PathBuf { app_cache_folder().join("BuildFolders") }
pub fn nix_portable_exe() -> std::path::PathBuf { app_cache_folder().join("nix-portable") }

#[derive(PartialEq, Eq)]
pub enum SandboxRuntime { Proot, Bwrap }
static SANDBOX_TO_USE: SandboxRuntime = SandboxRuntime::Bwrap;

pub async fn get_shell(env: &Environment) -> core::result::Result<std::process::Child, std::io::Error> {
    let nix_script = gen_a_nix_script(&env);
    let envs_folder = app_cache_folder().join("Environments");
    std::fs::create_dir_all(&envs_folder)?;
    let n = envs_folder.join("default.nix");
    if n.exists() { std::fs::remove_file(&n)?; }
    use std::io::Write; writeln!(std::fs::File::create(&n)?, "{}", &nix_script)?;

    let client = reqwest::Client::new();
    let nix_portable_exe_ = nix_portable_exe();
    download_file(&client, "https://github.com/DavHau/nix-portable/releases/latest/download/nix-portable", nix_portable_exe_.to_str().unwrap()).await.unwrap();
    let mut perms = std::fs::metadata(nix_portable_exe_.to_str().unwrap())?.permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o775);
    std::fs::set_permissions(nix_portable_exe_.to_str().unwrap(), perms)?;
    let startup_command =format!("bash");
    let sandbox_to_use = if SANDBOX_TO_USE == SandboxRuntime::Bwrap { "bwrap" } else { "proot" };
    let cmd = format!(r#"NP_RUNTIME={} HOME=/home/{}/nixHome NIXPKGS_ALLOW_UNFREE=1 {} nix-shell /home/marko/envie/Environments/default.nix --verbose --pure --keep HOME --option sandbox false --command "echo 'Entered the Nix env'; export SSL_CERT_FILE=/nix/store/2ymr3vj3sxgcpvwnrfwpz8d2zar030gq-nss-cacert-3.74/etc/ssl/certs/ca-bundle.crt; export HOME=/home/$USER; {}""#, &sandbox_to_use, &os_username(), nix_portable_exe().to_str().unwrap(), startup_command);
    println_!(&cmd);
    let mut child = std::process::Command::new("bash") .args(["-i", "-c", &cmd])
        // .stdin(std::process::Stdio::inherit())
        .stdin(std::process::Stdio::piped())
        // .stdout(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    // child_stdin.write_all(b"whoami\n")?;
    drop(child_stdin);
Ok(child) }

pub async fn main_() -> core::result::Result<(), std::io::Error> {

    Ok(())
}

pub fn os_username() -> String { uid_to_username(geteuid_()).unwrap() }
pub fn uid_to_username(uid: u32) -> core::option::Option<String> { unsafe {
    let mut result = std::ptr::null_mut();
    let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
        n if n < 0 => 512 as usize,
        n => n as usize,
    };
    let mut buf = Vec::with_capacity(amt);
    let mut passwd: libc::passwd = std::mem::zeroed();

    match libc::getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(), buf.capacity() as libc::size_t, &mut result) {
        0 if !result.is_null() => {
            let ptr = passwd.pw_name as *const _;
            let username = std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_owned();
            Some(username)
        },
        _ => None
    }
}}

pub struct ProgramInfo { name: &'static str }
static PROGRAM_INFO: ProgramInfo = ProgramInfo {
    name: "envie",
};

#[link(name = "c")]
extern "C" {
    pub fn geteuid() -> u32;
    pub fn getegid() -> u32;
}

fn geteuid_() -> u32 { unsafe { geteuid() } }

#[macro_export]
macro_rules! println_ {
    ($($arg:expr),*) => {
        $(print!("{}", $arg);)*
        println!();
    };
}

#[macro_export]
macro_rules! print_ {
    ($($arg:expr),*) => {
        $(print!("{}", $arg);)*
    };
}

pub struct IndexPointer<T> { pub element_index:u32, _phantom: core::marker::PhantomData<T>, }
pub struct Environment { pub name: String, pub installed_packages: Vec<String>, pub depends_on: Vec<String>, pub inherits_from: Vec<IndexPointer<Environment>> }

fn concatenate(strs: &[&str; 3]) -> String { strs.join("\n") }

pub fn gen_a_nix_script(env: &Environment) -> String {
    concatenate(&[r#"{ pkgs ? import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/a7ecde854aee5c4c7cd6177f54a99d2c1ff28a31.tar.gz")) {config = { allowUnfree = true; };}}:

with pkgs;
let
  stdenv = pkgs.stdenv; # Exist: pkgs.stdenv, pkgs.stdenvNoCC, pkgs.multiStdenv, pkgs.llvmPackages_12.stdenv. These don't exist: pkgs.multiStdenvNoCC
  nixpkgs_ToUseRustFrom = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/d45e9975c1bb7fa6273e7842a499eb58bbe27cc4.tar.gz")) {}; # rustc-1.58.1, cargo-1.58.1
in
# pkgs.llvmPackages_git.stdenv.mkDerivation { # has clang-13.0.0. 1st in which -a clang++ is now: clang-wrapper-13.0.0/bin/clang++, no gcc ("which: no gcc")
pkgs.llvmPackages_13.stdenv.mkDerivation {
# pkgs.llvmPackages_14.stdenv.mkDerivation {
  name = "marko-envie-env-1";
  src = ./.;
  # impureEnvVars = stdenv.lib.fetchers.proxyImpureEnvVars;

  passthru = with pkgs; {
    CURL_CA_BUNDLE = "${cacert}/etc/ssl/certs/ca-bundle.crt";
    GIT_SSL_CAINFO = "${cacert}/etc/ssl/certs/ca-bundle.crt";
    SSL_CERT_FILE = "${cacert}/etc/ssl/certs/ca-bundle.crt";

    LIBCLANG_PATH = "${libclang}/lib";

    OPENSSL_DIR = "${openssl.dev}";
    OPENSSL_LIB_DIR = "${openssl.out}/lib";
  };

  shellHook = ''
    export EXTRA_CLANG_ARGS="$(< ${stdenv.cc}/nix-support/libc-crt1-cflags) \
          $(< ${stdenv.cc}/nix-support/libc-cflags) \
          $(< ${stdenv.cc}/nix-support/cc-cflags) \
          $(< ${stdenv.cc}/nix-support/libcxx-cxxflags) \
          ${
            lib.optionalString stdenv.cc.isClang
            "-idirater ${stdenv.cc.cc}/lib/clang/${
              lib.getVersion stdenv.cc.cc
            }/include"
          } \
          ${
            lib.optionalString stdenv.cc.isGNU
            "-isystem ${stdenv.cc.cc}/include/c++/${
              lib.getVersion stdenv.cc.cc
            } -isystem ${stdenv.cc.cc}/include/c++/${
              lib.getVersion stdenv.cc.cc
            }/${stdenv.hostPlatform.config} -idirafter ${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${
              lib.getVersion stdenv.cc.cc
            }/include"
          } \
        "
        '';
  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [
  cacert # so that you can download from https urls in the nix env. /nix/store/2ymr3vj3sxgcpvwnrfwpz8d2zar030gq-nss-cacert-3.74/etc/ssl/certs/ca-bundle.crt
"#,
        &env.installed_packages.join(" "),
// envs[&env.inheritsFrom[0]].installedPackages.join(" "),

        r#"];

  propagatedBuildInputs = [

  ];

  buildPhase = "ls -la ./; ls -la ../; pwd; vcpkg --version; cd ..";
  # buildPhase = "ls -la ./; ls -la ../; pwd; vcpkg --version; cd ..; ./build.sh";

  installPhase = ''
    # mkdir -p $out/bin
    # cp main $out/bin/
  '';
}"#])
}

pub async fn download_file(client: &reqwest::Client, url: &str, path: &str) -> core::result::Result<(), String> {
    use std::io::Write;
    use futures_util::StreamExt; // stream.next()

    let res = client.get(url).send().await.or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res.content_length().ok_or(format!("Failed to get content length from '{}'", &url))?;

    let mut file = std::fs::File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk).or(Err(format!("Error while writing to file")))?;
        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
    }

    return Ok(());
}
