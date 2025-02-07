use std::fs::File;
use std::process::Command;

fn main() {
    let mut x = 0;
    let rsync_return = rsync();

    match &rsync_return as &str {
        "exit status: 0" => println!("Backup successful"),
        "exit status: 30" => println!("Sync Error, exit status: 30 "),
        "exit status: 35" => println!("Time Out Sync Error, exit status: 35"),
        "exit status: 255" => println!("Connection Timed Out Sync Error, exit status: 255"),
        _ => {
            x += 1;
            println!("Retry {} {}", x, rsync());

            x += 1;
            println!("Retry {} {}", x, rsync());

            x += 1;
            println!("Retry {} {}", x, rsync());

            println!("Backup failed after several Restarts");
        }
    }
}

fn rsync() -> String {
    let rsync_file_create = "rsync.log";
    let error_file_create = "error.log";

    let rsync_log = File::create(rsync_file_create).expect("Unable to create file");
    let error_log = File::create(error_file_create).expect("Unable to create file");

    let data = Command::new("bash")
        .arg("backup.sh")
        .stdout(rsync_log)
        .stderr(error_log)
        .output()
        .expect("failed to execute process");

    let value = data.status.to_string();

    return value;
}
