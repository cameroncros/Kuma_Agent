use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;
use configparser::ini::Ini;


#[cfg(all(not(test), target_os = "windows"))]
fn get_config_file() -> String {
    String::from("C:/Program Files/kuma_agent/kuma_agent.conf")
}

#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
fn get_config_file() -> String {
    String::from("/etc/kuma_agent.conf")
}

#[cfg(test)]
fn get_config_file() -> String {
    String::from("/tmp/kuma_agent.conf")
}

fn load_config(config_file: String) -> String {
    let mut config = Ini::new();

    let error_load = format!("Failed loading config [{config_file}]");
    config.load(config_file).expect(&error_load);

    let error_get = ". Contents should be:\n\
    \n\
    [main]\n\
    url = <kuma_url>\n\
    \n\
    \n";
    config.get("main", "url").expect(error_get)
}

fn update_kuma(url: &String) {
    let res = reqwest::blocking::get(url);
    if res.is_err() {
        println!("Failed to update, ignoring and continuing: [{}]", res.err().unwrap());
    }
}

pub fn mainloop(running: Arc<AtomicBool>) {
    let config_file = get_config_file();
    let url = load_config(config_file);

    loop {
        for _ in 0..60 {
            sleep(Duration::from_secs(1));

            if running.clone().load(Ordering::SeqCst) == false {
                return;
            }
        }

        update_kuma(&url);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::{fs, thread};
    use std::path::Path;
    use std::time::{Duration, SystemTime};
    use more_asserts::{assert_gt, assert_lt};
    use tempfile::NamedTempFile;
    use crate::mainloop::{get_config_file, load_config, mainloop};

    #[test]
    #[should_panic]
    fn test_load_config_bad_format_1() {
        let tempfile = NamedTempFile::new().unwrap();
        let config_path = tempfile.into_temp_path().as_os_str().to_str().unwrap().to_string();
        fs::write(config_path.clone(), "[not_main]\nurl=123\n".as_bytes()).unwrap();

        load_config(config_path);
    }

    #[test]
    #[should_panic]
    fn test_load_config_bad_format_2() {
        let tempfile = NamedTempFile::new().unwrap();
        let config_path = tempfile.into_temp_path().as_os_str().to_str().unwrap().to_string();
        fs::write(config_path.clone(), "[main]\nnot_url=123\n".as_bytes()).unwrap();

        load_config(config_path);
    }

    #[test]
    fn test_load_config_success() {
        let tempfile = NamedTempFile::new().unwrap();
        let config_path = tempfile.into_temp_path().as_os_str().to_str().unwrap().to_string();
        fs::write(config_path.clone(), "[main]\nurl=123\n".as_bytes()).unwrap();
        let url = load_config(config_path);
        assert_eq!("123", url);
    }

    #[test]
    fn test_mainloop_already_stopped() {
        let config_path = get_config_file();
        let path = Path::new(&config_path);
        let _ = fs::create_dir(path.parent().unwrap());

        let _ = fs::remove_file(&path);
        fs::write(config_path, "[main]\nurl=https://www.google.com\n").unwrap();

        let running = Arc::new(AtomicBool::new(false));
        mainloop(running);
    }

    #[test]
    fn test_mainloop_stop_3_seconds() {
        let config_path = get_config_file();
        let path = Path::new(&config_path);
        let _ = fs::create_dir(path.parent().unwrap());

        let _ = fs::remove_file(&path);
        fs::write(config_path, "[main]\nurl=https://www.google.com\n").unwrap();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        let start = SystemTime::now();
        let trd = thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            r.clone().store(false, Ordering::SeqCst);
        });

        mainloop(running);
        let end = SystemTime::now();
        let delta = end.duration_since(start).unwrap();
        assert_gt!(delta.as_secs(), 2);
        assert_lt!(delta.as_secs(), 10);
        trd.join().unwrap();
    }
}
