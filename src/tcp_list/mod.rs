use crate::tcp::Tcp;

use self::strategy::ListTCP;

mod lsof;
mod strategy;

pub(crate) fn get_tcp_list() -> Vec<Tcp> {
    lsof::Lsof::get_tcp_list()
}
