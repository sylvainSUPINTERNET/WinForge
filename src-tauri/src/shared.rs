pub struct Shared {
    queue: Mutex<VecDeque<u32>>,
    condvar: Condvar
}