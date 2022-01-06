use std::process::Command;

pub fn notify(message: String) {
    Command::new("notify-send")
        .arg("-t")
        .arg("3000")
        .arg(message)
        .spawn()
        .expect("failed to execute process");

    Command::new("paplay")
        .arg("complete.oga")
        .spawn()
        .expect("failed to execute process");
}
