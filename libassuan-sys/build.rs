use std::{ffi::OsString, process::Command};
mod build_helper;
use self::build_helper::*;

fn main() -> Result<()> {
    let mut config = get_config()?;
    if config.version.is_none() {
        config.try_detect_version("assuan.h", "ASSUAN_VERSION")?;
    }
    config.write_version_macro("assuan");
    config.print();
    Ok(())
}

fn get_config() -> Result<Config> {
    let project = Project::default();
    if let r @ Ok(_) = project.try_env() {
        return r;
    }

    if let Some(path) = get_env(project.prefix.clone() + "_CONFIG") {
        return try_config(&project, path);
    }

    if let r @ Ok(_) = try_registry(&project) {
        return r;
    }

    try_config(&project, "libassuan-config".into())
}

fn try_config(project: &Project, path: OsString) -> Result<Config> {
    let mut cmd = Command::new(path.clone());
    cmd.arg("--version");
    let version = output(cmd)?;
    let mut cmd = Command::new(path);
    cmd.args(["--libs", "--cflags"]);

    project.try_config(cmd)
    .map(|mut cfg| {
        cfg.version = Some(version.trim().into());
        cfg
    })
}
#[cfg(not(windows))]
fn try_registry(_: &Project) -> Result<Config> {
    Err(())
}

#[cfg(windows)]
fn try_registry(project: &Project) -> Result<Config> {
    use std::{fs, path::PathBuf};
    use winreg::{enums::*, RegKey};

    if !project.target.contains("windows") {
        eprintln!("cross compiling. disabling registry detection.");
        return Err(());
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = match hklm.open_subkey("SOFTWARE\\GnuPG") {
        Ok(k) => k,
        Err(_) => {
            //check if we are on 64bit windows but have 32bit GnuPG installed
            match hklm.open_subkey("SOFTWARE\\WOW6432Node\\GnuPG") {
                Ok(k) => {
                    // found 32bit library
                    if !project.target.contains("i586") && !project.target.contains("i686") {
                        eprintln!("Compile using i586/686 target.");
                        return Err(());
                    } else {
                        k
                    }
                }
                Err(_) => {
                    eprintln!("unable to retrieve install location");
                    return Err(());
                }
            }
        }
    };
    let root = PathBuf::from(
        key.get_value::<String, _>("Install Directory")
           .warn_err("unable to retrieve install location")?,
    );
    if root.join("lib/libassuan.imp").exists() {
        fs::copy(
            root.join("lib/libassuan.imp"),
            project.out_dir.join("libassuan.a"),
        )
        .warn_err("unable to rename library")?;
    }

    let mut config = Config::default();
    config.include_dir.insert(root.join("include"));
    config.lib_dir.insert(project.out_dir.clone());
    config.libs.insert(project.links.clone().into());
    config.prefix = Some(root);
    Ok(config)
}