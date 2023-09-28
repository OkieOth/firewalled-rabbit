use rocket::http::Status;
use env_logger::Env;
use log::{info, error};

#[macro_use]
extern crate rocket;

#[get("/")]
fn root() -> &'static str {
    "Call `/block` for block 6752 or `/unblock` to free the port again"
}

#[get("/block")]
fn block() -> Status {
    match run_ip_tables("-A", "INPUT", "--dport",  "5672") {
        Ok(()) => {
            match run_ip_tables("-A", "OUTPUT", "--sport",  "5672") {
                Ok(()) => {
                    info!("blocked 5672");
                    return Status::Ok
                },
                Err(s) => {
                    error!("error (2) while try to block 5672: {}", s);
                    return Status::InternalServerError
                }
            }
        },
        Err(s) => {
            error!("error while try to block 5672: {}", s);
            return Status::InternalServerError
        }
    }
}

#[get("/unblock")]
fn unblock() -> Status {
    match run_ip_tables("-D", "INPUT", "--dport", "5672") {
        Ok(()) => {
            match run_ip_tables("-D", "OUTPUT", "--sport", "5672") {
                Ok(()) => {
                    info!("ublocked 5672");
                    return Status::Ok
                },
                Err(s) => {
                    error!("error (2) while try to block 5672: {}", s);
                    return Status::InternalServerError
                }
            }
        },
        Err(s) => {
            error!("error while try to block 5672: {}", s);
            return Status::InternalServerError
        }
    }
}

#[launch]
fn rocket() -> _ {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    rocket::build()
        .mount("/", routes![root])
        .mount("/", routes![block])
        .mount("/", routes![unblock])
}

fn run_ip_tables(cmd: &str, chain: &str, port_type: &str, port: &str) -> Result<(), String> {
    //    iptables -A INPUT -p tcp --dport 15672 -j DROP
    match std::process::Command::new("iptables")
        .arg(cmd)
        .arg(chain)
        .arg("-p")
        .arg("tcp")
        .arg(port_type)
        .arg(port)
        .arg("-j")
        .arg("DROP")
        .output()
    {
        Ok(o) => {
            if o.status.success() {
                Ok(())
            } else {
                let o_vec = o.stderr;
                let s = String::from_utf8_lossy(&o_vec).to_string();
                Err(s)
            }
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}
