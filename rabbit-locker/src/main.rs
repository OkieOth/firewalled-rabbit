
#[macro_use] extern crate rocket;

#[get("/")]
fn root() -> &'static str {
    "Call `/block` for block 6752 or `/unblock` to free the port again"
}

#[get("/block")]
fn block() -> &'static str {
    match std::process::Command::new("iptables")
        .arg("--list")
        .output() {
        Ok(o) => {
            if o.status.success() {
                let o_vec = o.stdout;
                let s = String::from_utf8_lossy(&o_vec).to_string();
                Box::leak(s.into_boxed_str())
    
            } else {
                let o_vec = o.stderr;
                let s = String::from_utf8_lossy(&o_vec).to_string();
                Box::leak(s.into_boxed_str())
            }
        },
        Err(e) => {
            let s = format!("failed to exec iptables: {}", e.to_string());
            Box::leak(s.into_boxed_str())
        }
    }
}

#[get("/unblock")]
fn unblock() -> &'static str {
    "I opened local port 6752"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![root])
    .mount("/", routes![block])
    .mount("/", routes![unblock])
}