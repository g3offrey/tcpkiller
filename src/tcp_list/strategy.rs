use crate::tcp::Tcp;

pub(super) trait ListTCP {
    fn get_tcp_list() -> Vec<Tcp>;
}
