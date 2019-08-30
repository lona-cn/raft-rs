use super::readonly_option::ReadOnlyOption;

#[derive(Default)]
pub struct RaftOptions {
    ///Maximum of block size per RPC
    max_byte_count_per_rpc: i32,
    ///File service check hold switch, default disable
    file_check_hole: bool,
    ///The maximum number of entries in  AppendEntriesRequest
    max_body_size: i32,
    ///Flush buffer to LogStorage if the buffer size reaches the limit
    max_append_buffer_size: i32,
    ///Maximum election delay time allowed by user
    max_election_delay_ms: i32,
    ///Raft election:heartbeat timeout factor
    election_heartbeat_factor: i32,
    ///Maximum number of tasks that can be applied in a batch
    apply_batch: i32,
    ///Call fsync when need
    sync: bool,
    ///Sync log meta, snapshot meta and raft meta
    sync_meta: bool,
    ///Whether to enable replicator pipeline.
    replicator_pipeline: bool,
    ///The maximum replicator pipeline in-flight requests/responses, only valid when replicator pipeline.
    max_replicator_inflight_msgs: i32,
    ///disruptor buffs size for Node/FSMCaller/LogManager etc.
    disruptor_buffer_size: i32,
    ///The maximum timeout in seconds to wait when publishing events into disruptor, default is 10 seconds
    disruptor_publish_event_wait_timeout_seconds: i32,
    ///When true, validate log entry checksum when transferring the log entry from disk or network,default is false.
    /// If true, it would hurt the performance but gain the data safety
    enable_log_entry_checksum: bool,
    ///ReadOnlyOption specified how the read only request is processed.
    readonly_option: ReadOnlyOption,
}
