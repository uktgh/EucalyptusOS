use super::vfs::FileSystem;

pub struct ExtFileSystem {
    open_files: Vec<OpenFile>,
}

struct OpenFile {
    fd: usize,
    path: String,
    position: usize,
}

impl ExtFileSystem {
    pub fn new() -> Self {
        Self {
            open_files: Vec::new(),
        }
    }

    fn find_open_file(&self, fd: usize) -> Option<&OpenFile> {
        self.open_files.iter().find(|file| file.fd == fd)
    }

    fn find_open_file_mut(&mut self, fd: usize) -> Option<&mut OpenFile> {
        self.open_files.iter_mut().find(|file| file.fd == fd)
    }
}

impl FileSystem for ExtFileSystem {
    fn read(&self, path: &str, buffer: &mut [u8]) -> usize {
        if let Some(file) = self.find_open_file(0) {
            let bytes_read = buffer.len();
            for i in 0..bytes_read {
                buffer[i] = 0;
            }
            bytes_read
        } else {
            0
        }
    }

    fn write(&self, path: &str, buffer: &[u8]) -> usize {
        if let Some(file) = self.find_open_file(0) { 
            buffer.len()
        } else {
            0
        }
    }

    fn open(&self, path: &str) -> usize {
        let fd = self.open_files.len();
        self.open_files.push(OpenFile {
            fd,
            path: path.to_string(),
            position: 0,
        });
        fd
    }

    fn close(&self, fd: usize) {
        if let Some(index) = self.open_files.iter().position(|file| file.fd == fd) {
            self.open_files.remove(index);
        }
    }
}
