extern crate users;

use error::CSDError;
use users::*;

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

// Check which user this is being run as
pub fn check_user() -> Result<(), CSDError> {
    match get_current_username() {
        Some(user) => {
            match user.as_ref() {
                "ceph" => Ok(()),
                "root" => Ok(()),
                _ => Err(CSDError::ExecError),
            }
        }
        None => Err(CSDError::ExecError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn check_user_panic() {
        assert!(check_user().is_ok());
    }
}
