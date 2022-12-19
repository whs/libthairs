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

use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;
/// thbrk::datrie implements datrie-based breaker as used by original libthai
use std::path::{Path, PathBuf};
use std::{env, io};

use crate::thbrk::{StrBreaker, TisBreaker};
use crate::utils;
use encoding_rs::WINDOWS_874;
use lazy_static::lazy_static;
use memmap::Mmap;

mod breaker;
mod loader;
mod maximal;

lazy_static! {
    static ref LIBTHAI_PATH: PathBuf = PathBuf::from("/usr/share/libthai/thbrk.tri");
    static ref NATIVE_PATH: PathBuf = PathBuf::from(format!("{}/thbrk.fst", env!("OUT_DIR")));
    pub static ref SHARED_BRK: DatrieBrk = default_breaker().expect("unable to load default dict");
}

enum SetStorage {
    Vec(fst::Set<Vec<u8>>),
    Mmap(fst::Set<Mmap>),
}

pub struct DatrieBrk {
    trie: SetStorage,
}

pub fn default_breaker() -> io::Result<DatrieBrk> {
    // brk_load_default_dict
    match env::var("LIBTHAI_DICTDIR") {
        Ok(dict_dir) => {
            let mut path = PathBuf::from(dict_dir);
            path.push("thbrk.tri");
            DatrieBrk::from_datrie_path(&path)
        }
        Err(_) => {
            let out = DatrieBrk::from_native_path(&NATIVE_PATH);
            if out.is_ok() {
                return Ok(out.unwrap());
            }
            DatrieBrk::from_datrie_path(&LIBTHAI_PATH)
        }
    }
}

impl DatrieBrk {
    pub fn from_datrie_path(dict_path: &Path) -> io::Result<Self> {
        let fp = File::open(dict_path)?;
        let mut buf = BufReader::new(fp);
        let fst = loader::load(&mut buf)?;

        Ok(Self {
            trie: SetStorage::Vec(fst),
        })
    }

    pub fn from_native_path(dict_path: &Path) -> io::Result<Self> {
        let fp = File::open(dict_path)?;
        let mmap = unsafe { Mmap::map(&fp)? };
        let fst = fst::Set::new(mmap).or(Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unable to load",
        )))?;
        Ok(Self {
            trie: SetStorage::Mmap(fst),
        })
    }
}

impl TisBreaker for DatrieBrk {
    fn find_breaks_tis<'a>(&'a self, input: &'a [u8], max_out: usize) -> Vec<usize> {
        let input_str = BreakInput::from_tis(input);
        breaker::find_breaks(self, &input_str, max_out)
    }
}

impl StrBreaker for DatrieBrk {
    fn find_breaks<'a>(&'a self, input: &'a str, max_out: usize) -> Vec<usize> {
        let input_str = BreakInput::from_utf(input);
        breaker::find_breaks(self, &input_str, max_out)
    }
}

pub(super) struct BreakInput<'a> {
    pub(super) tis: Cow<'a, [u8]>,
    pub(super) char: Cow<'a, [char]>,
}

impl<'a> BreakInput<'a> {
    pub(super) fn from_tis(tis: &'a [u8]) -> Self {
        let (utf, _) = WINDOWS_874.decode_without_bom_handling(tis);
        debug_assert_eq!(tis.len(), utf.chars().count());
        Self {
            tis: Cow::from(tis),
            char: Cow::from(utf.chars().collect::<Vec<char>>()),
        }
    }

    pub(super) fn from_utf(utf: &'a str) -> Self {
        let tis = crate::utils::to_windows874(utf, u8::MAX);
        debug_assert_eq!(tis.len(), utf.chars().count());
        Self {
            tis: Cow::from(tis),
            char: Cow::from(utf.chars().collect::<Vec<char>>()),
        }
    }

    pub(super) fn substring(&'a self, min: usize, max: usize) -> Self {
        Self {
            tis: Cow::from(&self.tis[min..max]),
            char: Cow::from(&self.char[min..max]),
        }
    }

    /// Return a copy of the String stored
    pub fn str(&'a self) -> String {
        utils::as_str(&self.char)
    }

    pub fn str_buf(&'a self, out: &mut String) {
        utils::as_str_buf(&self.char, out)
    }
}

#[cfg(test)]
mod tests {
    use crate::thbrk::datrie::SHARED_BRK;
    use crate::thbrk::test::test_thbrk;

    #[test]
    fn thbrk() {
        test_thbrk(&*SHARED_BRK);
    }
}
