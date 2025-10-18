use actix::prelude::System;

use qatrade_rs::config::CONFIG;
use qatrade_rs::log4::init_log4;
use qatrade_rs::scheduler::Scheduler;
use actix::Actor;


fn main() {
    let sys = System::new();
    init_log4("log/qatrader.log", &CONFIG.common.log_level);
    let scheduler = Scheduler::new();
    scheduler.start();
    sys.run().expect("Failed to run actix system");
}