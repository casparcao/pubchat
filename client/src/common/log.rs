use std::fs::File;
use std::io::Write;

pub fn init() {
    // 创建日志文件
    let home = std::env::home_dir().expect("Failed to get home directory");
    let log_file = File::create(format!("{}/.pubchat.log", home.display())).expect("Failed to create log file");
    
    // 初始化 env_logger，输出到文件
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();
}