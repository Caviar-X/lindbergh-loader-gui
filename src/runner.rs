use std::env;
use anyhow::anyhow;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
const SO_LIST : [&'static str;7] = [
    "libkswapapi.so",
    "libposixtime.so",
    "libposixtime.so.1",
    "libposixtime.so.2.4",
    "libsegaapi.so",
    "lindbergh.so",
    "libCg.so"
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
pub fn copy_files(path: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(path);

    Ok(())
}
pub fn run_game(path: &str, test_mode: bool) -> anyhow::Result<Child> {
    let path = PathBuf::from(path);
    let ld_library_path = env::var("LD_LIBRARY_PATH")?;
    let fname = path.file_name().ok_or(anyhow!("Unable to get the filename of path"))?.to_string_lossy().to_string();
    if test_mode && get_test(&fname).is_some() {
        return run_game(get_test(&fname).unwrap().as_str(), false);
    } 
    let child = Command::new(&path)
        .env(
            "LD_LIBRARY_PATH",
            if !ld_library_path.is_empty() {
                format!("{}:.:lib:../lib", ld_library_path)
            } else {
                ".:lib:../lib".to_string()
            },
        )
        .env("LD_PRELOAD", "lindbergh.so")
        .arg(if test_mode && get_test(fname).is_none() { "-t" } else { "" })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    Ok(child)
}
