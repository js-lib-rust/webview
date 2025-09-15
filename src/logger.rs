use chrono::Local;
use env_logger::{Builder, Env, Target};
use std::fs::File;
use std::io::Write;

pub fn init(level: &str, file_path: &str) {
    let mut builder = Builder::from_env(Env::default().default_filter_or(level));
    builder.format(|buffer, record| {
        writeln!(
            buffer,
            "{} [{:?}] {} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
            current_thread_id(),
            buffer.default_styled_level(record.level()),
            record.target(),
            record.args()
        )
    });

    if file_path != "stdout" {
        if let Ok(file) = File::create(file_path) {
            builder.target(Target::Pipe(Box::new(file)));
        }
    }

    builder.init();
}

pub fn current_thread_id() -> u64 {
    use std::cell::Cell;
    use std::sync::Mutex;

    thread_local! {
        static THREAD_ID: Cell<Option<u64>> = Cell::new(None);
    }

    static NEXT_ID: Mutex<u64> = Mutex::new(0);

    THREAD_ID.with(|id| {
        if let Some(existing) = id.get() {
            return existing;
        }

        let mut next_id = NEXT_ID.lock().unwrap();
        let new_id = *next_id;
        *next_id += 1;
        id.set(Some(new_id));
        new_id
    })
}
