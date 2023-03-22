use crate::system_provider::SystemProvider;
use std::thread;
use std::time::Duration;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

#[cfg(test)]
mod tests {

    use crate::system_provider::Win32Provider;

    use super::*;
    #[test]
    fn win32() {
        let mut proc = std::process::Command::new("powershell")
            .arg("start chrome --start-fullscreen")
            .spawn()
            .unwrap();

        // TODO: get rid of sleep
        thread::sleep(Duration::from_secs(3));

        let provider = Win32Provider::new();
        let chrome_id = provider.try_get_game_pid().expect("Failed to get id");
        let system = System::new_all();
        let chrome = system.process(chrome_id).unwrap();

        assert_eq!(proc.id(), chrome.parent().unwrap().as_u32());

        let name = provider
            .try_get_product_name(chrome.exe().display().to_string())
            .expect("Failed to get product name");

        assert_eq!("Google Chrome", name);

        chrome.kill();
        proc.wait().expect("Chrome failed to exit");
    }
}
