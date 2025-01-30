use anyhow::anyhow;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::Duration;
use std::{env, thread};

use crate::games::GameTitle;
const SO_LIST: [&str; 7] = [
    "libkswapapi.so",
    "libposixtime.so",
    "libposixtime.so.1",
    "libposixtime.so.2.4",
    "libsegaapi.so",
    "lindbergh.so",
    "libCg.so",
];
fn get_test(name: impl ToString) -> Option<String> {
    match name.to_string().as_str() {
        "./hod4M.elf" => Some("./hod4testM.elf".into()),
        "./hodexRI.elf" => Some("./hodextestR.elf".into()),
        "./Jennifer" => Some("../JenTest/JenTest".into()),
        "./apacheM.elf" => Some("./apacheTestM.elf".into()),
        "./vt3_Lindbergh" => Some("./vt3_testmode".into()),
        _ => None,
    }
}
fn copy_files(path: &str, current_game: &GameTitle) -> anyhow::Result<()> {
    let mut path = PathBuf::from(path);
    path.pop();
    for i in SO_LIST {
        std::fs::copy(
            format!("./dynlibs/{}", i),
            format!("{}/{}", path.display(), i),
        )?;
    }
    std::fs::copy(
        format!("./config/{:?}.conf", current_game),
        format!("{}/lindbergh.conf", path.display()),
    )?;
    Ok(())
}
fn delete_files(path: &str) -> anyhow::Result<()> {
    let mut path = PathBuf::from(path);
    path.pop();
    for i in SO_LIST {
        std::fs::remove_file(format!("{}/{}", path.display(), i))?;
    }
    std::fs::remove_file(format!("{}/lindbergh.conf", path.display()))?;
    if std::fs::exists(format!("{}/eeprom.bin", path.display()))? {
        std::fs::remove_file(format!("{}/eeprom.bin", path.display()))?;
    }
    if std::fs::exists(format!("{}/sram.bin", path.display()))? {
        std::fs::remove_file(format!("{}/sram.bin", path.display()))?;
    }
    Ok(())
}
pub fn monitor_game(path: &str, child: &mut Child) -> anyhow::Result<Option<ExitStatus>> {
    thread::sleep(Duration::from_secs(1));
    let mut s = String::new();
    if let Some(a) = child.try_wait()? {
        child.stderr.as_mut().unwrap().read_to_string(&mut s)?;
        eprintln!("\x1b[94m[INFO]\x1b[0m STDERR OUTPUT:\n {}", s);
        delete_files(path)?;
        Ok(Some(a))
    } else {
        Ok(None)
    }
}
pub fn run_game(path: &str, test_mode: bool, current_game: &GameTitle) -> anyhow::Result<Child> {
    copy_files(path, current_game)?;
    let mut path = PathBuf::from(path);
    let ld_library_path = env::var("LD_LIBRARY_PATH").unwrap_or_default();
    let fname = path
        .file_name()
        .ok_or(anyhow!("Unable to get the filename of path"))?
        .to_string_lossy()
        .to_string();
    path.pop();
    if test_mode && get_test(&fname).is_some() {
        return run_game(get_test(&fname).unwrap().as_str(), false, current_game);
    }
    let child = Command::new(format!("{}/{}", path.display(), fname))
        .env(
            "LD_LIBRARY_PATH",
            if !ld_library_path.is_empty() {
                format!("{}:.:lib:../lib", ld_library_path)
            } else {
                ".:lib:../lib".to_string()
            },
        )
        .env("LD_PRELOAD", "lindbergh.so")
        .arg(if test_mode && get_test(fname).is_none() {
            "-t"
        } else {
            ""
        })
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    eprintln!("\x1b[94m[INFO]\x1b[0m Game process id: {:?}", child.id());
    Ok(child)
}
