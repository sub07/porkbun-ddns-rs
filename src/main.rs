use std::thread;

use ::log::{error, info};
use context::Context;
use log::setup_logger;

mod context;
mod log;
mod request;

fn main() {
    setup_logger();
    let context = Context::new();

    let mut last_ip = String::new();

    loop {
        match context.get_ip() {
            Ok(ip) => {
                info!("Ip is {ip}");
                info!("Last ip was {last_ip}");

                if ip != last_ip {
                    info!("Different ip, posting new ip");
                    match context.post_ip(&[""], &ip) {
                        Ok(()) => info!("Ip posted"),
                        Err(err) => error!("Error while posting ip: {err}"),
                    }
                    last_ip = ip;
                } else {
                    info!("Same ip, skipping dns post");
                }
            }
            Err(err) => error!("Error while fetch ip: {err}"),
        }

        thread::sleep(context.wait_interval);
    }
}
