use crate::tcp_list::get_tcp_list;

mod tcp;
mod tcp_list;

fn main() {
    let tcp_list = get_tcp_list();
    println!("found {} tcp connections", tcp_list.len());
    for tcp in tcp_list {
        println!("{} {}", tcp.pid, tcp.process_name);
    }
}
