# envie
License: GNU Affero General Public License v3

Envie is a Rust library, written by Marko Vujnovic, that makes it easy for your program to specify what runtime dependencies, such as libraries and CLI programs, it requires and envie will obtain them at the first run of your program. It accomplishes this in a distro-agnostic manner without requiring elevated privileges by using nix+proot/bwrap under the hood.

#### Usage:
```
let env_with_the_newest_rustc = Environment{ /*...*/ };
let mut env_shell = get_shell(&env_with_the_newest_rustc).await?;
env_shell.stdin.as_mut().unwrap().write_all(b"cargo build --release . \n")?;
```
