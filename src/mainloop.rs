use std::time::Duration;

use configparser::ini::Ini;
use tokio::time::sleep;

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

async fn update_kuma(url: &String) {
    if let Err(e) = reqwest::get(url).await {
        println!("Failed to update, ignoring and continuing: [{:#?}]", e);
    }
}

pub(crate) async fn mainloop() {
    let config_file = get_config_file();
    let url = load_config(config_file);

    loop {
        sleep(Duration::from_secs(60)).await;
        update_kuma(&url).await;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::time::{Duration, SystemTime};

    use more_asserts::{assert_gt, assert_lt};
    use tempfile::NamedTempFile;
    use tokio::select;
    use tokio::time::sleep;

    use crate::mainloop::{get_config_file, load_config, mainloop};

    #[tokio::test]
    #[should_panic]
    async fn test_load_config_bad_format_1() {
        let tempfile = NamedTempFile::new().unwrap();
        let config_path = tempfile.into_temp_path().as_os_str().to_str().unwrap().to_string();
        fs::write(config_path.clone(), "[not_main]\nurl=123\n".as_bytes()).unwrap();

        load_config(config_path);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_load_config_bad_format_2() {
        let tempfile = NamedTempFile::new().unwrap();
        let config_path = tempfile.into_temp_path().as_os_str().to_str().unwrap().to_string();
        fs::write(config_path.clone(), "[main]\nnot_url=123\n".as_bytes()).unwrap();

        load_config(config_path);
    }

    #[tokio::test]
    async fn test_load_config_success() {
        let tempfile = NamedTempFile::new().unwrap();
        let config_path = tempfile.into_temp_path().as_os_str().to_str().unwrap().to_string();
        fs::write(config_path.clone(), "[main]\nurl=123\n".as_bytes()).unwrap();
        let url = load_config(config_path);
        assert_eq!("123", url);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn test_mainloop_stop_3_seconds() {
        let config_path = get_config_file();
        let path = Path::new(&config_path);
        let _ = fs::create_dir(path.parent().unwrap());

        let _ = fs::remove_file(path);
        fs::write(config_path, "[main]\nurl=https://www.google.com\n").unwrap();
        let start = SystemTime::now();

        async fn shutdown_3seconds() {
            println!("Start sleep");
            sleep(Duration::from_secs(3)).await;
            println!("End sleep");
        }

        select! {
            () = mainloop() => {},
            () = shutdown_3seconds() => {}
        }

        let end = SystemTime::now();
        let delta = end.duration_since(start).unwrap();
        assert_gt!(delta.as_secs(), 2);
        assert_lt!(delta.as_secs(), 10);
    }
}
