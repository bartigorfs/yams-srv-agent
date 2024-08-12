use tokio::process::Command;

pub async  fn get_iops() -> String {
    let output = Command::new("iostat")
        .output()
        .await.expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    return output.to_string();
}