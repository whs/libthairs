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

/// thbrk::datrie implements datrie-based breaker as used by original libthai
use std::path::{Path, PathBuf};
use std::{env, io};

use crate::thbrk::TisBreaker;
use datrie::Trie;
use lazy_static::lazy_static;

mod breaker;
mod maximal;

pub const DICT_NAME: &str = "thbrk";
pub const DICT_DIR: &str = "/usr/share/libthai";

lazy_static! {
    pub static ref SHARED_BRK: DatrieBrk =
        DatrieBrk::default().expect("unable to load default dict");
}

pub struct DatrieBrk {
    trie: Trie,
}

impl DatrieBrk {
    pub fn new(dict_path: Option<&Path>) -> io::Result<Self> {
        match dict_path {
            Some(path) => Self::from_path(Path::new(path)),
            None => Self::default(),
        }
    }

    pub fn from_path(dict_path: &Path) -> io::Result<Self> {
        let out = Trie::from_file(dict_path.as_ref())?;

        Ok(Self { trie: out })
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
    fn find_breaks<'b>(&'b self, input: &'b [u8], max_out: usize) -> Vec<usize> {
        breaker::find_breaks(self, input, max_out)
    }
}
