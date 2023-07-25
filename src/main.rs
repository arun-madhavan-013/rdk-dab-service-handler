use std::process;
use std::path::Path;
use chrono;
use notify::{ Config, event, EventKind, RecommendedWatcher, RecursiveMode, Watcher };
use std::thread;
use std::time::Duration;

fn fd_monitor_thread() {
    static MONITORPATH: &str = "/mnt/c/Users/amadha013/Desktop/rdk-next";
    println!("Monitoring changes of {}/dab-enable", MONITORPATH);
    let monitor_path = Path::new(MONITORPATH);

    let (tx, rx) = std::sync::mpsc::channel();
    let config = Config::default().with_poll_interval(Duration::from_secs(5));
    let mut watcher: Box<dyn Watcher> = Box::new(RecommendedWatcher::new(tx, config).unwrap());

    watcher
        .watch(&monitor_path, RecursiveMode::Recursive)
        .unwrap();

    'fd_wait_loop: for data in rx {
        match data {
            Ok (event) => {
                if event.kind == EventKind::Remove(event::RemoveKind::File) && (event.paths.len() > 0) {
                    for i in &event.paths {
                        let rm_file = i.as_path().display().to_string();
                        let monitor_file = monitor_path.display().to_string()+"/dab-enable";
                        if monitor_file.eq(&rm_file) {
                            println!("MATCH: {:?} {:?}", rm_file, monitor_file);
                            break 'fd_wait_loop;
                        }
                    }
                }
            },
            Err(error) => println!("DATA_ER error: {:?}", error),
        }
    }
    println!("[{:?}] Clean-Up triggered.", chrono::offset::Utc::now());
    let _ = watcher.unwatch(&monitor_path);
    process::exit(0x00);
}

fn main() {
    let handler = thread::spawn(|| { fd_monitor_thread(); });

    for i in 1..35 {
        println!("[{:?}] Main code says {}.", chrono::offset::Utc::now(), i);
        thread::sleep(Duration::from_secs(1));
    }

    handler.join().expect("Couldn't join the fd_monitor_thread");
}
