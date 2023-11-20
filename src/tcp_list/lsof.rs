use crate::tcp::Tcp;

use super::strategy::ListTCP;

pub(super) struct Lsof;

impl ListTCP for Lsof {
    fn get_tcp_list() -> Vec<Tcp> {
        let output = std::process::Command::new("lsof")
            .arg("-i")
            .arg("tcp")
            .arg("-s")
            .arg("TCP:Listen")
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8_lossy(&output.stdout);
        let output = output.trim();

        let mut tcp_list: Vec<Tcp> = Vec::new();
        for line in output.lines().skip(1) {
            let mut line = line.split_whitespace();
            let process_name = match line.next() {
                Some(process_name) => process_name,
                None => continue,
            };

            let pid = match line.next() {
                Some(pid) => pid,
                None => continue,
            };
            let pid_number = match pid.parse::<usize>() {
                Ok(pid_number) => pid_number,
                Err(_) => continue,
            };

            tcp_list.push(Tcp {
                pid: pid_number,
                process_name: process_name.to_string(),
            });
        }

        tcp_list.sort();
        tcp_list.dedup();

        tcp_list
    }
}
