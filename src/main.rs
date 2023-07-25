use std::path::Path;
use chrono;
use notify::{ Config, event, EventKind, PollWatcher, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind, };
use std::time::Duration;

static MONITORPATH: &str = "/mnt/c/Users/amadha013/Desktop/rdk-next";

fn main() {
    println!("Monitoring changes in {}", MONITORPATH);

    let monitor_path = Path::new(MONITORPATH);

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        let config = Config::default().with_poll_interval(Duration::from_millis(200));
        Box::new(PollWatcher::new(tx, config).unwrap())
    } else {
        Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
    };

    watcher
        .watch(&monitor_path, RecursiveMode::Recursive)
        .unwrap();

    for data in rx {
        println!("[{:?}] New... {:?}", chrono::offset::Utc::now(), data);
        match data {
            Ok (mut event) => {
                println!("DATA_OK event: {:?}", event);
                if event.kind == EventKind::Remove(event::RemoveKind::File)
                    && (event.paths.len() > 0) {
                    let current_path: Option<PathBuf> = event.paths.pop().map(Into::into);
                    if current_path == monitor_path {

                    }
                }
            },
            Err(error) => println!("DATA_ER error: {:?}", error),
        }
    }
    let _ = watcher.unwatch(&monitor_path);
}