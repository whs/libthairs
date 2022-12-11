use crate::binding::*;
use std::ffi::{CString, OsStr};
use std::io;
use std::io::Write;
use std::sync::RwLock;

pub struct Trie {
    pub(crate) c: RwLock<*mut crate::binding::Trie>,
}

impl Trie {
    pub fn from_file(path: &OsStr) -> io::Result<Self> {
        unsafe {
            let name_bytes = {
                #[cfg(unix)]
                {
                    use std::os::unix::ffi::OsStrExt;
                    CString::new(path.as_bytes())?
                }
                #[cfg(windows)]
                {
                    use std::os::windows::ffi::OsStrExt;
                    todo!()
                }
            };
            let out = trie_new_from_file(name_bytes.as_ptr());
            if out.is_null() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "fail to read file",
                ));
            }

            return Ok(Trie {
                c: RwLock::new(out),
            });
        }
    }

    pub fn save(&self, path: &OsStr) -> io::Result<()> {
        let lock = self.c.read().or(Err(io::Error::new(
            io::ErrorKind::Other,
            "unable to lock trie",
        )))?;

        unsafe {
            let name_bytes = {
                #[cfg(unix)]
                {
                    use std::os::unix::ffi::OsStrExt;
                    CString::new(path.as_bytes())?
                }
                #[cfg(windows)]
                {
                    use std::os::windows::ffi::OsStrExt;
                    todo!()
                }
            };
            let out = trie_save(lock.cast(), name_bytes.as_ptr());

            match out {
                0 => Ok(()),
                _ => Err(io::Error::new(io::ErrorKind::Other, "fail to write data")),
            }
        }
    }

    pub fn write<T: Write>(&self, mut writer: T) -> io::Result<usize> {
        let lock = self.c.read().or(Err(io::Error::new(
            io::ErrorKind::Other,
            "unable to lock trie",
        )))?;

        unsafe {
            let size = trie_get_serialized_size(lock.cast());
            let mut buf = vec![0 as u8; size];
            trie_serialize(lock.cast(), buf.as_mut_ptr());
            writer.write(&buf)
        }
    }

    pub fn is_dirty(&self) -> bool {
        unsafe { trie_is_dirty(self.c.read().unwrap().cast()) }
    }

    pub fn retrieve(&self, key: &[AlphaChar]) -> Option<TrieData> {
        let mut out = TrieData::default();

        unsafe {
            let status = trie_retrieve(self.c.read().ok()?.cast(), key.as_ptr(), &mut out);
            if !status {
                return None;
            }
        }

        Some(out)
    }

    pub fn store(&mut self, key: &[AlphaChar], data: TrieData) -> bool {
        let ptr = match self.c.write() {
            Err(_) => return false,
            Ok(v) => v,
        };
        unsafe { trie_store(ptr.cast(), key.as_ptr(), data) }
    }

    pub fn store_if_absent(&mut self, key: &[AlphaChar], data: TrieData) -> bool {
        let ptr = match self.c.write() {
            Err(_) => return false,
            Ok(v) => v,
        };
        unsafe { trie_store_if_absent(ptr.cast(), key.as_ptr(), data) }
    }

    pub fn delete(&mut self, key: &[AlphaChar]) -> bool {
        let ptr = match self.c.write() {
            Err(_) => return false,
            Ok(v) => v,
        };
        unsafe { trie_delete(ptr.cast(), key.as_ptr()) }
    }

    pub fn root<'a>(&'a self) -> crate::triestate::TrieState<'a> {
        unsafe {
            let lock = self.c.read().unwrap();
            let root = trie_root(lock.cast());
            if root.is_null() {
                panic!("unable to get root");
            }

            crate::triestate::TrieState {
                c: root,
                _lock: lock,
                trie: self,
            }
        }
    }

    pub fn iter(&self) -> crate::trieiter::TrieIter {
        self.iter_from(self.root())
    }

    pub fn iter_from<'a>(
        &'a self,
        state: crate::triestate::TrieState<'a>,
    ) -> crate::trieiter::TrieIter<'a> {
        let trie = self.c.read().unwrap();
        let iter = unsafe { trie_iterator_new(*state) };

        if iter.is_null() {
            panic!("unable to start iterator")
        }

        crate::trieiter::TrieIter {
            _trie: trie,
            _initial_state: state,
            c: iter,
        }
    }
}

impl Drop for Trie {
    fn drop(&mut self) {
        let value = self.c.get_mut().unwrap();
        unsafe {
            trie_free(*value);
        }
    }
}

unsafe impl Send for Trie {}
unsafe impl Sync for Trie {}

#[cfg(test)]
mod test {
    use crate::Trie;

    #[test]
    fn load_error() {
        let out = Trie::from_file("!@#$!@#$%!@$%^^.");
        assert!(out.is_err());
    }
}
