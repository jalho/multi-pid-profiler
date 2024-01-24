use std::collections::HashMap;
use std::fs::{DirEntry, ReadDir};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

struct ProcDetails {
    pid: u32,
    parent_pid: u32,
    vmrss_kb: u32,
}

fn main() {
    let main_pid = std::env::args()
        .nth(1)
        .expect("Please provide a PID as a command-line argument")
        .parse()
        .expect("Invalid PID");

    let mut all_pids: HashMap<u32, ProcDetails> = HashMap::new();

    let mut output = std::io::stdout();

    let main_proc: ProcDetails;
    match read_proc(main_pid) {
        Ok(n) => {
            main_proc = n;
        }
        Err(_) => todo!(),
    }
    all_pids.insert(main_pid, main_proc);

    // write CSV like header line
    read_descendants(main_pid, &mut all_pids);
    write!(output, "seconds_elapsed").unwrap();
    for (_, details) in all_pids.iter() {
        write!(output, ",PID-{}", details.pid).unwrap();
    }
    writeln!(output).unwrap();

    // write CSV like body lines
    let mut step = 0;
    loop {
        read_descendants(main_pid, &mut all_pids);
        write!(output, "{}", step).unwrap();
        for (_, details) in all_pids.iter() {
            write!(output, ",{}", details.vmrss_kb).unwrap();
        }
        writeln!(output).unwrap();
        step = step + 1;
        sleep(Duration::from_millis(1000));
    }
}

fn read_proc(pid: u32) -> Result<ProcDetails, &'static str> {
    let proc_status_content: String;
    match std::fs::read_to_string(format!("/proc/{}/status", pid)) {
        Ok(n) => {
            proc_status_content = n;
        }
        Err(_) => {
            return Err("No entry found in /proc for given PID");
        }
    }
    let line: &str;
    match get_line(&proc_status_content, "PPid") {
        Some(n) => {
            line = n;
        }
        None => todo!(),
    }
    let parent_pid: u32;
    match parse_int(line) {
        Some(n) => {
            parent_pid = n;
        }
        None => todo!(),
    }

    let line: &str;
    match get_line(&proc_status_content, "VmRSS") {
        Some(n) => {
            line = n;
        }
        None => {
            return Err("foo");
        }
    }
    let vmrss_kb: u32;
    match parse_int(line) {
        Some(n) => {
            vmrss_kb = n;
        }
        None => todo!(),
    }

    return Ok(ProcDetails {
        parent_pid,
        pid,
        vmrss_kb,
    });
}

fn read_descendants(main_pid: u32, all_pids: &mut HashMap<u32, ProcDetails>) {
    let entries: ReadDir;
    match std::fs::read_dir("/proc") {
        Ok(n) => {
            entries = n;
        }
        Err(_) => todo!(),
    }

    for _entry in entries {
        let entry: DirEntry;
        match _entry {
            Ok(n) => {
                entry = n;
            }
            Err(_) => todo!(),
        }
        let file_name: &str;
        let _file_name = entry.file_name();
        match _file_name.to_str() {
            Some(n) => {
                file_name = n;
            }
            None => todo!(),
        }
        let pid: u32;
        match file_name.parse::<u32>() {
            Ok(n) => {
                pid = n;
            }
            Err(_) => {
                continue;
            }
        }

        let proc: ProcDetails;
        match read_proc(pid) {
            Ok(n) => {
                proc = n;
            }
            Err(_) => {
                continue;
            }
        }

        if proc.parent_pid == main_pid {
            all_pids.insert(pid, proc);
        }
    }
}

fn parse_int(input: &str) -> Option<u32> {
    let mut numeric_part = String::new();
    let mut found_digit = false;
    let radix = 10;

    for c in input.chars() {
        if c.is_digit(radix) {
            numeric_part.push(c);
            found_digit = true;
        } else if found_digit {
            break;
        }
    }

    if let Ok(parsed_value) = numeric_part.parse() {
        Some(parsed_value)
    } else {
        None
    }
}

fn get_line<'a>(text: &'a str, matcher: &'a str) -> Option<&'a str> {
    for line in text.lines() {
        if line.trim().starts_with(matcher) {
            return Some(line);
        }
    }
    None
}
