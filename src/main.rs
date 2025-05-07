use std::env;
use std::process::Command;

fn main() {
    const VERSION: &str = "1.0.4";
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "-v" || args[1] == "--version") {
        println!("khelper version {}", VERSION);
        return;
    }

    if args.len() == 3 && (args[1] == "-o" || args[1] == "open") {
        match args[2].as_str() {
            "youtube" => {
                let _ = Command::new("open")
                    .arg("https://www.youtube.com/")
                    .status()
                    .expect("failed to open browser");
            }
            "as" => {
                let _ = Command::new("open")
                    .arg("-a")
                    .arg("Android Studio")
                    .status()
                    .expect("failed to open Android Studio");
            }
            "music" | "m" => {
                let _ = Command::new("open")
                    .arg("-a")
                    .arg("Spotify")
                    .status()
                    .expect("failed to open Spotify");
            }
            _ => {
                eprintln!("Unknown target: {}", args[2]);
            }
        }
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
    eprintln!("             youtube");
    eprintln!("             as");
    eprintln!("             music (or 'm')");
    eprintln!("  khelper -version (or 'v')");
}