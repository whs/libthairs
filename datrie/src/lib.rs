////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

pub use alphamap::AlphaMap;
pub use trie::{StoreError, Trie, TrieIter, TrieState};

mod alphamap;
mod alphamaploader;
#[cfg(feature = "cffi")]
mod cffi;
mod darray;
mod darrayloader;
mod symbols;
mod tail;
mod tailloader;
#[cfg(all(test, not(feature = "cffi")))]
mod test_cdatrie;
#[cfg(test)]
mod test_utils;
mod trie;

/// AlphaChar is the alphabet character used in words of a target language
pub type AlphaChar = u32;
pub type TrieIndex = i32;
pub type TrieChar = u8;
pub type TrieData = i32;
