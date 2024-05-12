use quickxml_to_serde::{xml_string_to_json, Config, NullValue};
use std::io::{Write, BufRead};

pub fn get_n3fjp_settings_dir() -> String {
    let conf = Config::new_with_defaults();
    println!("Created Default XML Config");
    // open a tcp stream to the n3fjp server
    let mut stream = std::net::TcpStream::connect("localhost:1100").expect("Expected TCP Stream");
    println!("Connected to N3FJP Server");
    // send "<CMD><SETTINGSPATHSHARED></CMD>"
    stream
        .write_all(b"<CMD><SETTINGSPATHSHARED></CMD>")
        .expect("Expected Write");
    println!("Sent Request to N3FJP Server");
    // read the response
    let mut reader = std::io::BufReader::new(stream);
    println!("Reading Response from N3FJP Server");
    let mut buffer = String::new();
    // read the response into the buffer for 10 seconds to allow the server to respond before proceeding
    reader
        .read_line(&mut buffer)
        .expect("Expected Read");
    println!("Response: {}", buffer);
    // parse the response
    let json = xml_string_to_json(buffer, &conf).expect("Expected JSON");
    println!("Parsed JSON: {:?}", json);
    // get the settings path
    let settings_path = json["CMD"]["SETTINGSPATHRESPONSE"]["VALUE"].as_str().expect("Expected String");
    println!("Settings Path: {}", settings_path);
    settings_path.to_string()
}