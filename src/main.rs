use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == "-o" && args[2] == "youtube" {
        let _ = Command::new("open")
            .arg("https://www.youtube.com/")
            .status()
            .expect("failed to open browser");
        return;
    }

    if args.len() == 4 && args[1] == "-o" && args[2] == "store" {
        let package = &args[3];
        let url = format!("https://play.google.com/store/apps/details?id={}", package);
        let _ = Command::new("open")
            .arg(url)
            .status()
            .expect("failed to open browser");
        return;
    }

    eprintln!("Usage:");
    eprintln!("  khelper -o store <package_name>");
    eprintln!("  khelper -o youtube");
}