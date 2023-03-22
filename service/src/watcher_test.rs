use crate::{system_provider::SystemProvider, watcher::Watcher};
use std::thread;
use sysinfo::{Pid, PidExt};

#[cfg(test)]
mod tests {

    use super::*;

    pub struct MockProvider {
        proc: std::process::Child,
    }

    impl MockProvider {
        fn new() -> Self {
            let proc = std::process::Command::new("powershell")
                .arg("sleep 3")
                .spawn()
                .unwrap();
            Self { proc: proc }
        }
    }

    impl SystemProvider for MockProvider {
        fn try_get_game_pid(&self) -> Result<Pid, String> {
            Ok(Pid::from_u32(self.proc.id()))
        }

        fn try_get_product_name(&self, exe_name: String) -> Result<String, String> {
            assert!(exe_name.ends_with("powershell.exe"));
            Ok("PowerShell".to_string())
        }
    }

    #[test]
    fn watch() {
        let (send_closer, closer) = crossbeam::channel::bounded(1);
        let (sender, receiver) = crossbeam::channel::unbounded();
        let mut mp = MockProvider::new();

        let h = thread::spawn(move || {
            let w = Watcher::new();
            w.watch(&mp, sender, closer);

            let m = w.games.lock().unwrap();
            assert_eq!("powershell.exe", m.keys().last().unwrap());

            mp.proc.kill().expect("process should exit");
            mp.proc.wait().expect("process should have exited");
        });

        for rcv in receiver {
            assert_eq!("0h 0m", rcv);
            send_closer.send(()).expect("send should succeed");
            break;
        }

        h.join().expect("join failed");
    }
}
