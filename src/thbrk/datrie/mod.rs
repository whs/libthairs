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

use crate::thbrk::TisBreaker;
use encoding_rs::WINDOWS_874;
use lazy_static::lazy_static;

mod breaker;
mod loader;
mod maximal;

pub const DICT_NAME: &str = "thbrk";
pub const DICT_DIR: &str = "/usr/share/libthai";

lazy_static! {
    pub static ref SHARED_BRK: DatrieBrk =
        DatrieBrk::default().expect("unable to load default dict");
}

pub struct DatrieBrk {
    trie: fst::Set<Vec<u8>>,
}

impl DatrieBrk {
    pub fn new(dict_path: Option<&Path>) -> io::Result<Self> {
        match dict_path {
            Some(path) => Self::from_path(Path::new(path)),
            None => Self::default(),
        }
    }

    pub fn from_path(dict_path: &Path) -> io::Result<Self> {
        let fp = File::open(dict_path)?;
        let mut buf = BufReader::new(fp);
        let fst = loader::load(&mut buf)?;

        Ok(Self { trie: fst })
    }

    fn default() -> io::Result<Self> {
        // brk_load_default_dict
        match env::var("LIBTHAI_DICTDIR") {
            Ok(dict_dir) => {
                let mut path = PathBuf::from(dict_dir);
                path.push(format!("{}.tri", DICT_NAME));
                Self::from_path(&path)
            }
            Err(_) => {
                let mut path = PathBuf::from(DICT_DIR);
                path.push(format!("{}.tri", DICT_NAME));
                Self::from_path(&path)
            }
        }
    }
}

impl TisBreaker for DatrieBrk {
    fn find_breaks_tis<'a>(&'a self, input: &'a [u8], max_out: usize) -> Vec<usize> {
        let input_str = BreakInput::from_tis(input);
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
        crate::utils::as_str(&self.char)
    }
}

#[cfg(test)]
mod tests {
    use crate::thbrk::test::test_thbrk;
    use crate::DatrieBrk;

    #[test]
    fn thbrk() {
        let breaker = DatrieBrk::default().unwrap();
        test_thbrk(breaker);
    }
}
