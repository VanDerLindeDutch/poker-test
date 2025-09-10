use chrono::{TimeZone, Utc};
use log::{error, info, set_logger};
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::AtomicI32;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

mod repo;
mod domain;


//here we can use condvar or channel
struct SharedState {
    time: AtomicI32,
}
fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "info"); }
    }
    env_logger::init();
    let (host, port, user, password, database) = ("localhost", "5432", "postgres", "postgres", "poker_test");
    let shared_state = Arc::new(SharedState { time: AtomicI32::new(0) });
    let mut pg_client = repo::Client::new(&format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, database)).expect("error while creating pg client");
    pg_client.migration().expect("pg_client.migration: ");
    let repo = Arc::new(Mutex::new(pg_client));
    let repo_cloned = repo.clone();
    let shared_state_cloned = shared_state.clone();
    let mut threads = vec![];
    threads.push(std::thread::spawn(move || {
        loop {
            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("system time panic").as_secs() as i32;
            shared_state_cloned.time.store(time, SeqCst);
            sleep(Duration::from_secs(1));
        }
    }));
    let shared_state_cloned = shared_state.clone();
    threads.push(std::thread::spawn(move || {
        loop {
            if let Err(err) = repo_cloned.lock().unwrap().insert(shared_state_cloned.time.load(SeqCst)) {
                error! {"repo.insert: {:?}", err}
            };
            sleep(Duration::from_millis(500));
        }
    }));
    let repo_cloned = repo.clone();
    threads.push(std::thread::spawn(move || {
        loop {
            match repo_cloned.lock().unwrap().get_last() {
                Ok(v) => {
                    let time =  Utc.timestamp_opt(v as i64, 0).unwrap();
                    info!("{}", time.format("%Y-%m-%d %H:%M:%S"));
                }
                Err(err) => { error!("repo.get_last: {:?}", err) }
            }
            sleep(Duration::from_secs(1));
        }
    }));
    threads.into_iter().for_each(|x| { x.join().expect("thread panicked:"); });
}
