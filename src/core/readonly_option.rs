use std::net::Shutdown::Read;
use crate::core::readonly_option::ReadOnlyOption::ReadOnlyLeaseBased;


pub enum ReadOnlyOption {
    ///ReadOnlySafe guarantees the linearizability of read only request by
    /// communicating with the quorum. It is the default and suggested option.
    ReadOnlySafe,
    ///ReadOnlyLeaseBased ensures linearizability of the read only request by
    /// relying on the leader lease. It can be affected by clock drift.
    /// If the clock drift is unbounded, leader might keep the lease longer than it
    /// should (clock can move backward/pause without any bound). ReadIndex is not safe
    /// in that case.
    ReadOnlyLeaseBased,
}

impl Default for ReadOnlyOption {
    fn default() -> ReadOnlyOption {
        ReadOnlyOption::ReadOnlySafe
    }
}