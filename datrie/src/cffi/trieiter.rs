use crate::cffi::trie::TRIE_DATA_ERROR;
use crate::trie::{TrieIter, TrieState};
use crate::{AlphaChar, TrieData};
use std::ptr::null_mut;

/// Create a new trie iterator for iterating entries of a sub-trie rooted at state
///
/// Use it with the result of [trie_root] to iterate the whole trie.
///
/// The created object must be freed with [trie_iterator_free].
#[no_mangle]
extern "C" fn trie_iterator_new<'a>(state: &'a TrieState) -> *mut TrieIter<'a> {
    // TrieIterator *  trie_iterator_new (TrieState *s);
    todo!()
}

/// Destruct the iterator and free its allocated memory.
#[no_mangle]
extern "C" fn trie_iterator_free(iter: *mut TrieIter) {
    // void            trie_iterator_free (TrieIterator *iter);
    unsafe { drop(Box::from_raw(iter)) }
}

/// Move trie iterator to the next entry
///
/// Move trie iterator to the next entry.
/// On return, the iterator iter is updated to reference to the new entry
/// if successfully moved.
#[no_mangle]
extern "C" fn trie_iterator_next(iter: &mut TrieIter) -> bool {
    // Bool            trie_iterator_next (TrieIterator *iter);
    iter.next().is_some()
}

/// Get key for the current entry referenced by the trie iterator @a iter.
///
/// The return string must be freed with free().
#[no_mangle]
extern "C" fn trie_iterator_get_key(iter: &mut TrieIter) -> *mut AlphaChar {
    // AlphaChar *     trie_iterator_get_key (const TrieIterator *iter);
    match iter.get_key() {
        Some(mut key) => key.as_mut_ptr(), // TODO: This is incorrect - it may have too short lifetime or leak
        None => null_mut(),
    }
}

/// Get value for the entry referenced by an iterator. Getting value from an
/// un-iterated (or broken for any reason) iterator will result in
/// TRIE_DATA_ERROR.
extern "C" fn trie_iterator_get_data(iter: &mut TrieIter) -> TrieData {
    // TrieData        trie_iterator_get_data (const TrieIterator *iter);
    iter.get_data().unwrap_or(TRIE_DATA_ERROR)
}