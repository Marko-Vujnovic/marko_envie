//! Copyright © Marko Vujnovic, GNU Affero General Public License v3

use envie::*;

#[test]
fn rustenv_t() -> core::result::Result<(), std::io::Error> { tokio::runtime::Runtime::new().unwrap().block_on(async {
    let env_descr = Environment{
        name: "Rust dev env with a very recent rustc version".to_string(),
        installed_packages: vec![
            "bash".to_string(),
            "openssl".to_string(),
            "jetbrains.idea-community".to_string(),
            "nixpkgs_ToUseRustFrom.rustc".to_string(),
            "nixpkgs_ToUseRustFrom.cargo".to_string(),
        ],
        depends_on: vec![],
        inherits_from: vec![],
    };
    let mut env_shell = get_shell(&env_descr).await?;
    { use std::io::Write; env_shell.stdin.as_mut().unwrap().write_all(b"cargo build --release\n")?; }
    let cmd_result = env_shell.wait_with_output()?;
    println!("cmd_result: {:?}", &cmd_result);
    Ok(())
})}