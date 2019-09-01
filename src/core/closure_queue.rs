use std::collections::LinkedList;

pub struct Closure {}

pub struct TaskClosure {}

pub trait ClosureQueue {
    /// clear all closure in queue
    fn clear(&mut self);
    ///Reset the first index in queue
    /// @param first_index the first index of queue
    fn reset_first_index(&mut self, first_index: u64);
    /// Append a new closure into queue
    /// @param closure the closure to append
    fn append_pending_closure(closure: Closure);
    /// Pop closure from queue until index(inclusion), returns the first popped out index,
    /// returns -1 when out of range, returns index+1 when not found.
    /// @param end_index the index of queue
    /// @param closures closure list
    /// @return returns the first popped out index, returns  -1 when out of range, returns index + 1
    /// when not found.
    fn pop_closure_until(&self, end_index: u64, closure: LinkedList<Closure>) -> u64;

    /// Pop closure from queue until index(inclusion), returns the first popped out index, returns -1
    /// when out of range, returns index+1 when not found.
    /// @param end_index the index of queue.
    /// @param closures closure list
    /// @param task_closures, task closure list
    /// @return returns the first popped out index, returns -1 when out of range , returns index+ 1
    /// when not found.
    fn pop_closure_until_v2(
        &mut self,
        end_index: u64,
        closures: LinkedList<Closure>,
        task_closures: LinkedList<TaskClosure>,
    ) -> u64;
}
