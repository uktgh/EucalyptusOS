pub mod vfs;
pub mod ext;

pub struct FileSystemManager {
    mounted_fs: Option<Box<dyn vfs::FileSystem>>,
}

impl FileSystemManager {
    pub fn new() -> Self {
        Self { mounted_fs: None }
    }

    pub fn mount(&mut self, fs: &str) {
        if fs == "ext" {
            self.mounted_fs = Some(Box::new(ext::ExtFileSystem::new()));
        }
    }

    pub fn unmount(&mut self) {
        self.mounted_fs = None;
    }

    pub fn read(&self, path: &str, buffer: &mut [u8]) -> usize {
        if let Some(ref fs) = self.mounted_fs {
            fs.read(path, buffer)
        } else {
            0
        }
    }

    pub fn write(&self, path: &str, buffer: &[u8]) -> usize {
        if let Some(ref fs) = self.mounted_fs {
            fs.write(path, buffer)
        } else {
            0
        }
    }

    pub fn open(&self, path: &str) -> usize {
        if let Some(ref fs) = self.mounted_fs {
            fs.open(path)
        } else {
            usize::MAX
        }
    }

    pub fn close(&self, fd: usize) {
        if let Some(ref fs) = self.mounted_fs {
            fs.close(fd);
        }
    }
}
