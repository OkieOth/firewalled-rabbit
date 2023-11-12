use rocket::http::{Status, RawStr};
use env_logger::Env;
use log::{info, error};
use std::process::{Command, Stdio};
use std::str;

#[macro_use]
extern crate rocket;

#[get("/")]
fn root() -> &'static str {
    "Call `/block` for block 6752 or `/unblock` to free the port again"
}

// returns either 'blocked' or 'unblocked'
#[get("/block_state")]
fn block_state() -> &'static str {
    // https://stackoverflow.com/questions/73469520/how-to-pipe-commands-in-rust
    // iptables -L | grep DROP | wc -l
    let iptables_child = Command::new("iptables")
        .arg("-L")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child = Command::new("grep")
        .arg("DROP")
        .stdin(Stdio::from(iptables_child.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let wc_child = Command::new("wc")
        .arg("-l")
        .stdin(Stdio::from(grep_child.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = wc_child.wait_with_output().unwrap();
    let result_lines = str::from_utf8(&output.stdout).unwrap();
    if result_lines == "2\n" {
        "blocked"
    } else {
        "unblocked"
    }
}

#[get("/rabbit_cons")]
fn get_cons() -> Result<String, String> {
    match std::process::Command::new("rabbitmqadmin")
        .arg("--user")
        .arg("guest")
        .arg("--password")
        .arg("guest")
        .arg("list")
        .arg("connections")
        .arg("-f")
        .arg("pretty_json")
        .output()
    {
        Ok(o) => {
            if o.status.success() {
                let o_vec = o.stdout;
                let s = String::from_utf8_lossy(&o_vec).to_string();
                Ok(s)
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

#[get("/rabbit_channels")]
fn get_channels() -> Result<String, String> {
    match std::process::Command::new("rabbitmqadmin")
        .arg("--user")
        .arg("guest")
        .arg("--password")
        .arg("guest")
        .arg("list")
        .arg("channels")
        .arg("-f")
        .arg("pretty_json")
        .output()
    {
        Ok(o) => {
            if o.status.success() {
                let o_vec = o.stdout;
                let s = String::from_utf8_lossy(&o_vec).to_string();
                Ok(s)
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

#[get("/rabbit_queues")]
fn get_queues() -> Result<String, String> {
    match std::process::Command::new("rabbitmqadmin")
        .arg("--user")
        .arg("guest")
        .arg("--password")
        .arg("guest")
        .arg("list")
        .arg("queues")
        .arg("-f")
        .arg("pretty_json")
        .output()
    {
        Ok(o) => {
            if o.status.success() {
                let o_vec = o.stdout;
                let s = String::from_utf8_lossy(&o_vec).to_string();
                Ok(s)
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

#[get("/close_rabbit_con/<con_name>")]
fn close_con(con_name: &str) -> String {
    "test".to_string()
    // match std::process::Command::new("iptables")
    //     .arg(cmd)
    //     .arg(chain)
    //     .arg("-p")
    //     .arg("tcp")
    //     .arg(port_type)
    //     .arg(port)
    //     .arg("-j")
    //     .arg("DROP")
    //     .output()
    // {
    //     Ok(o) => {
    //         if o.status.success() {
    //             Ok(())
    //         } else {
    //             let o_vec = o.stderr;
    //             let s = String::from_utf8_lossy(&o_vec).to_string();
    //             Err(s)
    //         }
    //     }
    //     Err(e) => {
    //         Err(e.to_string())
    //     }
    // }
}

#[get("/close_rabbit_queue/<queue_name>")]
fn close_queue(queue_name: &str) -> String {
    "test".to_string()
    // match std::process::Command::new("iptables")
    //     .arg(cmd)
    //     .arg(chain)
    //     .arg("-p")
    //     .arg("tcp")
    //     .arg(port_type)
    //     .arg(port)
    //     .arg("-j")
    //     .arg("DROP")
    //     .output()
    // {
    //     Ok(o) => {
    //         if o.status.success() {
    //             Ok(())
    //         } else {
    //             let o_vec = o.stderr;
    //             let s = String::from_utf8_lossy(&o_vec).to_string();
    //             Err(s)
    //         }
    //     }
    //     Err(e) => {
    //         Err(e.to_string())
    //     }
    // }
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
        .mount("/", routes![block_state])
        .mount("/", routes![close_con])
        .mount("/", routes![close_con])
        .mount("/", routes![get_cons])
        .mount("/", routes![get_channels])
        .mount("/", routes![get_queues])
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
