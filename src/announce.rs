use std::{collections::HashMap, sync::LazyLock};

use reqwest::{blocking::{Client, ClientBuilder}, header::HeaderValue};
use tracing::{event, Level};

use crate::cfg::{get_announce_masters, get_interval, Server};

pub static SESSION: LazyLock<Client> = LazyLock::new(|| {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("VCMP/0.4"));
    headers.insert("VCMP-Version", HeaderValue::from_static("67710"));
    ClientBuilder::new().default_headers(headers).build().unwrap()
});

pub fn post(
    id: &String,
    url: &String,
    form: &HashMap<&str, u16>,
) {
    let session = SESSION.clone();
    event!(Level::DEBUG, "Announce {id} to {url}");
    match session.post(url).form(form).send() {
        Ok(resp) => {
            match resp.error_for_status() {
                Ok(_) => {
                    event!(Level::DEBUG, "Announce {id} to {url} succeeded");
                }
                Err(e) => {
                    event!(Level::ERROR, "Announce {id} to {url} failed: {e}");
                }
            }
        }
        Err(e) => {
            event!(Level::ERROR, "Announce {id} to {url} failed: {e}");
        }
    }
}

pub fn announce(
    id: String,
    port: u16
) {
    let masters = get_announce_masters();
    let mut tasks = Vec::new();
    let form = HashMap::from([("port", port)]);
    for master in masters {
        let id_clone = id.clone();
        let form_clone = form.clone();
        tasks.push(std::thread::spawn(move || {
            post(&id_clone, &master, &form_clone);
        }));
    }
    for task in tasks {
        task.join().unwrap();
    }
}

pub fn start_announce(
    servers: HashMap<String, Server>
) {
    let mut threads = Vec::new();
    for (id, server) in servers {
        let id_clone = id.clone();
        let port = server.port;
        threads.push(std::thread::spawn(move || {
            loop {
                announce(id_clone.clone(), port);
                std::thread::sleep(std::time::Duration::from_secs(get_interval()));
            }
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}