use std::fs;

pub fn initialize(){
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_filename = format!("logs/app_{}.log", timestamp);

    fs::create_dir_all("logs").expect("could not create logs dir");

    let raw = fs::read_to_string("logs/config/log4rs.yaml")
        .expect("could not read log4rs config file");
    let substituted = raw.replace("${DYNAMIC_LOG_PATH:-logs/app.log}", &log_filename);

    let generated_config_path = "logs/config/log4rs.generated.yaml";
    fs::write(generated_config_path, substituted)
        .expect("could not write generated log4rs config");

    log4rs::init_file(generated_config_path, Default::default())
        .expect("Failed to initialize log4rs configuration file");
}
