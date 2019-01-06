pub struct ThreadPool;

impl ThreadPool {
    pub fn new(cnt: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<T>(&self, f: T)
    where T: FnOnce() + Send + 'static {

    }
}
