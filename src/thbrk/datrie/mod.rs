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
