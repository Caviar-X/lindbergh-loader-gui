use crate::games::GameTitle;
use anyhow::anyhow;
use chrono::Local;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::Duration;
use std::{env, thread};

const SO_LIST: [&str; 10] = [
    "libkswapapi.so",
    "libposixtime.so",
    "libposixtime.so.1",
    "libposixtime.so.2.4",
    "libcrypto.so.0.9.7",
    "libopenal.so.0",
    "libssl.so.0.9.7",
    "libsegaapi.so",
    "lindbergh.so",
    "libCg.so",
];

fn get_test(name: impl ToString) -> Option<String> {
    match name.to_string().as_str() {
        "hod4M.elf" => Some("hod4testM.elf".into()),
        "hodexRI.elf" => Some("hodextestR.elf".into()),
        "Jennifer" => Some("../JenTest/JenTest".into()),
        "apacheM.elf" => Some("apacheTestM.elf".into()),
        "vt3_Lindbergh" => Some("vt3_testmode".into()),
        _ => None,
    }
}
fn check_files() -> anyhow::Result<()> {
    for i in SO_LIST {
        if !fs::exists(format!("./dynlibs/{}", i))? {
            return Err(anyhow!("{} does not exists in dynlibs directory", i));
        }
    }
    Ok(())
}
fn copy_files(path: &str) -> anyhow::Result<()> {
    let mut path = PathBuf::from(path);
    path.pop();
    for i in SO_LIST {
        std::fs::copy(
            format!("./dynlibs/{}", i),
            format!("{}/{}", path.display(), i),
        )?;
    }
    Ok(())
}
fn delete_files(path: &str) -> anyhow::Result<()> {
    let mut path = PathBuf::from(path);
    path.pop();
    for i in SO_LIST {
        std::fs::remove_file(format!("{}/{}", path.display(), i))?;
    }
    std::fs::remove_file(format!("{}/eeprom.bin", path.display()))?;
    std::fs::remove_file(format!("{}/sram.bin", path.display()))?;
    Ok(())
}
pub fn monitor_game(path: &str, child: &mut Child) -> anyhow::Result<Option<ExitStatus>> {
    thread::sleep(Duration::from_secs(1));
    let mut f = File::create(format!(
        "./log/{}.log",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    ))?;
    let mut s = String::new();
    writeln!(f, "Game Process ID: {}", child.id())?;
    if let Some(a) = child.try_wait()? {
        child.stderr.as_mut().unwrap().read_to_string(&mut s)?;
        writeln!(f, "STDOUT\n{}", str::repeat("-", 64))?;
        writeln!(f, "{}\n\n\n\n", s)?;
        child.stdout.as_mut().unwrap().read_to_string(&mut s)?;
        writeln!(f, "STDERR\n{}", str::repeat("-", 64))?;
        writeln!(f, "{}", s)?;
        delete_files(path)?;
        Ok(Some(a))
    } else {
        Ok(None)
    }
}
pub fn run_game(path: &str, test_mode: bool, current_game: &GameTitle) -> anyhow::Result<Child> {
    check_files()?;
    copy_files(path)?;
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
        .env(
            "LINDBERGH_CONFIG_PATH",
            format!(
                "{}/config/{:?}.conf",
                env::current_dir()?.display(),
                current_game
            ),
        )
        .arg(if test_mode && get_test(fname).is_none() {
            "-t"
        } else {
            ""
        })
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    Ok(child)
}
