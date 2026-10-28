
use std::env;
use std::fs;
use std::path::Path;

const DATA_FILE: &str = "steps.json";

fn read_steps() -> u64 {
    if !Path::new(DATA_FILE).exists() {
        return 0;
    }

    let content = fs::read_to_string(DATA_FILE)
        .unwrap_or_else(|_| "0".to_string());

    content
        .trim()
        .parse::<u64>()
        .unwrap_or(0)
}

fn save_steps(steps: u64) {
    fs::write(DATA_FILE, steps.to_string())
        .expect("걸음 수 파일을 저장하지 못했습니다.");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("add") => {
            let amount = match args.get(1).and_then(|s| s.parse::<u64>().ok()) {
                Some(n) => n,
                None => {
                    println!("사용법: cargo run -- add 3000");
                    return;
                }
            };

            let Some(total) = read_steps().checked_add(amount) else {
                println!("걸음 수가 허용 범위를 초과했습니다.");
                return;
            };

            save_steps(total);
            println!("{amount}걸음을 추가했습니다. 누적 걸음 수: {total}");
        }
        Some("show") => {
            println!("현재 누적 걸음 수: {}", read_steps());
        }
        Some("reset") => {
            save_steps(0);
            println!("걸음 수를 초기화했습니다.");
        }
        _ => {
            println!("만보기 사용법:");
            println!("  cargo run -- add 3000");
            println!("  cargo run -- show");
            println!("  cargo run -- reset");
        }
    }
}
