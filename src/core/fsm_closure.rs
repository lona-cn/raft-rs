trait LoadSnapshotClosure {}

trait LastAppliedLogIndexListener {
    fn on_applied(last_applied_log_index: u64);
}

trait FSMCaller {
    fn add_last_applied_log_index_listener<T>(listener: T) where T: LastAppliedLogIndexListener;

    /// Called when log entry committed
    /// @param committed_index committed log index
    fn on_committed(committed_index: u64) -> bool;

    ///Called after loading snapshot.
    /// @param done callback
    fn on_snapshot_load<T>(done: T) where T: LoadSnapshotClosure;
}