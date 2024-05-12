use reqwest::*;
mod xml;

async fn fetch() {
    let res = get("https://httpbin.org/ip")
        .await
        .expect("Expected Completed Response, Please Ensure Network Connectivity");
    println!(
        "Body: {:?}",
        res.text().await.expect("Expected Textual Response")
    );
}

fn get_conf_dir() -> String {
    match get_os().as_str() {
        "windows" => std::env::var("APPDATA").expect("Expected APPDATA Environment Variable"),
        "linux" => std::env::var("HOME").expect("Expected HOME Environment Variable"),
        "macos" => std::env::var("HOME").expect("Expected HOME Environment Variable"),
        _ => {
            panic!("Unsupported Operating System")
        }
    }
}

fn check_dir_perms() {
    let conf_dir = get_conf_dir();
    let base_path = std::path::Path::new(&conf_dir);
    // add field-day-map to the path
    let path = base_path.join("field-day-map");
    // check if the path exists
    if !path.exists() {
        //check the permissions to ensure the path is writable
        if !base_path.metadata().expect("Expected Metadata").permissions().readonly() {
            // create the directory
            std::fs::create_dir(&path).expect("Expected Directory Creation");
        } else {
            panic!("Path is Read-Only");
        }
    }
}

fn get_os() -> String {
    std::env::consts::OS.to_string()
}

#[tokio::main]
async fn main() {
    check_dir_perms();
    println!("OS: {}", get_os());
    xml::get_n3fjp_settings_dir();
}
