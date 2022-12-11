use crate::binding::*;
use std::ops::Deref;
use std::sync::RwLockReadGuard;

pub struct TrieState<'a> {
    // Hold the trie read lock
    pub(super) _lock: RwLockReadGuard<'a, *mut crate::binding::Trie>,
    pub(super) trie: &'a crate::trie::Trie,
    pub(crate) c: *mut crate::binding::TrieState,
}

impl<'a> TrieState<'a> {
    pub fn rewind(&mut self) {
        unsafe {
            trie_state_rewind(self.c);
        }
    }

    pub fn walk(&mut self, c: AlphaChar) -> bool {
        unsafe { trie_state_walk(self.c, c) }
    }

    pub fn is_walkable(&self, c: AlphaChar) -> bool {
        unsafe { trie_state_is_walkable(self.c, c) }
    }

    pub fn walkable_chars(&self) -> Vec<AlphaChar> {
        let nelm = 20; // TODO: What is the value here
        let mut out = vec![0; nelm];

        unsafe {
            let count =
                trie_state_walkable_chars(self.c, out.as_mut_ptr(), out.len() as i32) as usize;
            out.truncate(count);

            out
        }
    }

    #[inline]
    pub fn is_terminal(&self) -> bool {
        self.is_walkable(0)
    }

    pub fn is_single(&self) -> bool {
        unsafe { trie_state_is_single(self.c) }
    }

    pub fn is_leaf(&self) -> bool {
        self.is_single() && self.is_terminal()
    }

    pub fn get_data(&self) -> TrieData {
        unsafe { trie_state_get_data(self.c) }
    }
}

impl<'a> Clone for TrieState<'a> {
    fn clone(&self) -> Self {
        unsafe {
            let c = trie_state_clone(self.c);
            let rlock = self.trie.c.read().unwrap();
            TrieState {
                c,
                trie: self.trie,
                _lock: rlock,
            }
        }
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            trie_state_copy(self.c, source.c);
        }
        self.trie = source.trie;
        self._lock = source.trie.c.read().unwrap();
    }
}

impl<'a> Drop for TrieState<'a> {
    fn drop(&mut self) {
        unsafe {
            trie_state_free(self.c);
        }
    }
}

impl<'a> Deref for TrieState<'a> {
    type Target = *mut crate::binding::TrieState;

    fn deref(&self) -> &Self::Target {
        &self.c
    }
}
