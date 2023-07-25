use std::process;
use std::path::Path;
use chrono;
use notify::{ Config, event, EventKind, RecommendedWatcher, RecursiveMode, Watcher };
//use notify::{ Config, event, EventKind, PollWatcher, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind, };
use std::thread;
use std::time::Duration;

fn fd_monitor_thread() {
    static MONITORPATH: &str = "/mnt/c/Users/amadha013/Desktop/rdk-next";
    println!("Monitoring changes in {}", MONITORPATH);
    let monitor_path = Path::new(MONITORPATH);

    let (tx, rx) = std::sync::mpsc::channel();
    let config = Config::default().with_poll_interval(Duration::from_secs(5));
    let mut watcher: Box<dyn Watcher> = Box::new(RecommendedWatcher::new(tx, config).unwrap());
    //let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
    //    let config = Config::default().with_poll_interval(Duration::from_secs(5));
    //    println!("Monitoring with custom config.");
    //    Box::new(PollWatcher::new(tx, config).unwrap())
    //} else {
    //    println!("Monitoring with default config.");
    //    Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
    //};

    watcher
        .watch(&monitor_path, RecursiveMode::Recursive)
        .unwrap();

    'fd_wait_loop: for data in rx {
        match data {
            Ok (event) => {
                if event.kind == EventKind::Remove(event::RemoveKind::File) && (event.paths.len() > 0) {
                    println!("[{:?}] DATA_RM event: {:?}", chrono::offset::Utc::now(), event);
                    for i in &event.paths {
                        println!("ITR: {:?}", i.as_path().display().to_string());
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
    println!("[{:?}] Main code.", chrono::offset::Utc::now());
    let handler = thread::spawn(|| { fd_monitor_thread(); });

    for i in 1..35 {
        println!("[{:?}] Main code says {}.", chrono::offset::Utc::now(), i);
        thread::sleep(Duration::from_secs(1));
    }

    handler.join().expect("Couldn't join the fd_monitor_thread");
}
