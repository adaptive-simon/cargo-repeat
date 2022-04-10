use std::process::Command;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Verbose
    #[clap(short = 'v', long = "verbose", takes_value = false)]
    v: bool,
    /// Exit code to repeat until it's hit
    #[clap(short = 'c', long = "code", value_name = "i32", default_value = "0")]
    c: i32,
    /// Delay between retries
    #[clap(short = 's', long = "sleep", value_name = "f64", default_value = "1")]
    s: f64,
    /// Command and arguments to repeat
    #[clap(last = true)]
    target: Vec<String>,
}

fn run(args: &Args) -> Result<i32, std::io::Error> {
    let shell = if cfg!(target_os = "windows") {
        ["cmd", "/C"]
    } else {
        ["sh", "-c"]
    };

    let child_process = Command::new(shell[0])
        .arg(shell[1])
        .args(&args.target)
        .spawn()
        .expect("failed to execute process");

    let n = child_process.wait_with_output()?;

    let exit_code = n.status.code().unwrap();
    Ok(exit_code)
}

fn main() -> Result<(), std::io::Error> {
    let start = std::time::SystemTime::now();
    let args = Args::parse();
    let expected_status = args.c.clone();

    let mut counter = 0;

    loop {
        counter += 1;
        if run(&args)? == expected_status {
            break;
        } else {
            std::thread::sleep(std::time::Duration::from_millis((args.s * 1000.0) as u64));
        }
    }
    let end = std::time::SystemTime::now();
    if args.v {
        println!("");
        println!("Retries: {}", counter);
        println!("Duration: {:?}", end.duration_since(start).unwrap());
    }
    Ok(())
}
