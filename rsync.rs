use std::process::Command;
fn main() {
    let data = Command::new("bash")
        .arg("backup.sh")
        .output()
        .expect("failed to execute process");
    let value = data.status.to_string();
    //let value: &'static str = "exit status: 35";
    //println!("{}", value2);
    match &value as &str {
        "exit status: 0" => println!("Backup successful, exit status: 0"),
        "exit status: 30" => println!("Sync Error, exit status: 30 "),
        "exit status: 35" => println!("Time Out Sync Error, exit status: 35"),
        _ => println!("Backup failed"),
    }
    println!("{}", String::from_utf8_lossy(&data.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&data.stderr));
}
