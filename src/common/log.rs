
pub type LogResult<T> = Result<T, LogType>;

//TODO: Make this procedural to handle both @str and String.
#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        LogType::Info($msg.to_string()).write_to_log()
    };
    ($fmt:expr, $($args:expr),+) => {
        LogType::Info(format!($fmt, $($args),+)).write_to_log()
    };
}
pub enum LogType {
    Info(String),
    Warning(String),
    Error(String),
}

impl LogType {
    pub fn write_to_log(&self) {
        match self {
            LogType::Info(message) => println!("Info: {}", message),
            LogType::Warning(message) => println!("Warning: {}", message),
            LogType::Error(message) => println!("Error: {}", message),
        }
    }
    pub fn diplay_to_user(&self) {
        println!("Not displayed to user!");
        self.write_to_log();
        //match self {
        //    LogType::Info(message) => println!("Info: {}", message),
        //    LogType::Warning(message) => println!("Warning: {}", message),
        //    LogType::Error(message) => println!("Error: {}", message),
        //}
    }
}
