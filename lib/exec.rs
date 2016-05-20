use error::CSDError;

use std::process::Command;

pub fn call_ceph(cmd: &str) -> Result<String, CSDError> {
    let ceph = try!(Command::new("/usr/bin/env")
                        .arg("sh")
                        .arg("-c")
                        .arg(format!("ceph {} -f json", cmd))
                        .output());
    if ceph.status.success() {
        Ok(String::from_utf8_lossy(&ceph.stdout).into_owned())
    } else {
        Err(CSDError::CephExecError(String::from_utf8_lossy(&ceph.stderr).into_owned()))
    }
}
