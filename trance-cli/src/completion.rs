// SPDX-License-Identifier: MIT

//! Shell autocompletion script generator module.

use anyhow::{Result, anyhow, bail};

pub fn handle_completion(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("usage: trance completion bash | zsh | fish | nu");
    }

    match args[0].as_str() {
        "bash" => {
            let script = include_str!("completions/trance.bash");
            println!("{script}");
            Ok(())
        }
        "zsh" => {
            let script = include_str!("completions/trance.zsh");
            println!("{script}");
            Ok(())
        }
        "fish" => {
            let script = include_str!("completions/trance.fish");
            println!("{script}");
            Ok(())
        }
        "nu" | "nushell" => {
            let script = include_str!("completions/trance.nu");
            println!("{script}");
            Ok(())
        }
        _ => Err(anyhow!(
            "unsupported shell '{}'; please specify 'bash', 'zsh', 'fish', or 'nu'",
            args[0]
        )),
    }
}
