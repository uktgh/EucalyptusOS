pub trait FileSystem {
    fn read(&self, path: &str, buffer: &mut [u8]) -> usize;
    fn write(&self, path: &str, buffer: &[u8]) -> usize;
    fn open(&self, path: &str) -> usize;
    fn close(&self, fd: usize);
}
