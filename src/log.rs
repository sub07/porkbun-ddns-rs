pub fn setup_logger() {
    if let Err(_e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    #[cfg(debug_assertions)]
    const LOG4RS_PATH: &str = "log4rs.yaml";
    #[cfg(not(debug_assertions))]
    const LOG4RS_PATH: &str = "/etc/porkbun-ddns/log4rs.yaml";

    log4rs::init_file(LOG4RS_PATH, Default::default()).unwrap();
}
