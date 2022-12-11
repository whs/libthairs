use crate::binding::*;
use crate::types::AlphaCharEx;
use std::sync::RwLockReadGuard;

pub struct TrieIter<'a> {
    pub(super) c: *mut TrieIterator,

    pub(super) _initial_state: crate::triestate::TrieState<'a>,
    // Hold the trie read lock
    pub(super) _trie: RwLockReadGuard<'a, *mut crate::binding::Trie>,
}

impl<'a> TrieIter<'a> {
    fn get_key(&self) -> Option<Vec<AlphaChar>> {
        unsafe {
            let out = trie_iterator_get_key(self.c);
            if out.is_null() {
                return None;
            }

            let out_vec = Vec::from((out as *const AlphaChar).as_slice());
            libc::free(out.cast());

            Some(out_vec)
        }
    }

    fn get_data(&self) -> Option<TrieData> {
        unsafe {
            let out = trie_iterator_get_data(self.c);

            if out == TRIE_DATA_ERROR {
                return None;
            }

            Some(out)
        }
    }
}

impl<'a> Iterator for TrieIter<'a> {
    type Item = (Vec<AlphaChar>, Option<TrieData>);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let has_next = trie_iterator_next(self.c);
            if !has_next {
                return None;
            }

            Some((self.get_key().unwrap(), self.get_data()))
        }
    }
}

impl<'a> Drop for TrieIter<'a> {
    fn drop(&mut self) {
        unsafe { trie_iterator_free(self.c) }
    }
}
