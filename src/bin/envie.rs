//! Copyright © Marko Vujnovic, GNU Affero General Public License v3

use envie::*;

fn main() -> core::result::Result<(), std::io::Error> { tokio::runtime::Runtime::new().unwrap().block_on(async {
    main_().await?;
    Ok(())
})}

