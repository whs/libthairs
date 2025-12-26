use crate::tis::{
    TIS_KO_KAI, TIS_LU, TIS_MAITAIKHU, TIS_MAI_HAN_AKAT, TIS_O_ANG, TIS_RU, TIS_SARA_A,
    TIS_SARA_AA, TIS_SARA_AE, TIS_SARA_E, TIS_SARA_UEE, TIS_THANTHAKHAT, TIS_WO_WAEN,
};
use crate::{thchar_t, ThaiCharacter};
use datrie::{CTrieData, ROTrie, TrieState};
use std::path::Path;
use std::{env, io};

const DICT_DIR: &'static str = "share/libthai";
const DICT_NAME: &'static str = "thbrk";

pub type ThTrie = ROTrie<Option<CTrieData>>;
pub type ThTrieState<'a> = TrieState<'a, Option<CTrieData>>;

pub(super) fn brk_load_default_dict() -> io::Result<ThTrie> {
    let env = env::var("LIBTHAI_DICTDIR");
    let dict_file = match env {
        Ok(v) => Path::new(&v).join(format!("{}.tri", DICT_NAME)),
        Err(_) => Path::new(DICT_DIR).join(format!("{}.tri", DICT_NAME)),
    };

    ThTrie::from_file(dict_file)
}

pub(super) fn brk_brkpos_hints(str: &[thchar_t]) -> Vec<bool> {
    let mut hints = vec![false; str.len()];

    let mut i = 0;

    while i < str.len() {
        match str[i] {
            v if v.is_th_cons() => {
                if str.get(i + 1) == Some(&TIS_THANTHAKHAT) {
                    i += 2; /* the cons + THANTHAKHAT */
                } else if str.get(i + 2) == Some(&TIS_THANTHAKHAT) {
                    i += 3; /* the cons + intermediate char + THANTHAKHAT */
                } else if i + 2 < str.len()
                    && str[i] != TIS_KO_KAI
                    && str[i + 1] == TIS_MAITAIKHU
                    && (str[i + 2] == TIS_O_ANG || str[i + 2] == TIS_WO_WAEN)
                {
                    hints[i] = true;
                    i += 4; /* the cons + MAITAIKHU + OANG/WOWAEN + cons */
                } else if (i > 0 && (str[i - 1] == TIS_MAI_HAN_AKAT || str[i - 1] == TIS_SARA_UEE))
                    || (i > 1
                        && str[i - 1].is_th_tone()
                        && (str[i - 2] == TIS_MAI_HAN_AKAT || str[i - 2] == TIS_SARA_UEE))
                {
                    i += 1;
                } else {
                    hints[i] = true;
                    i += 1;
                }
            }
            TIS_SARA_E | TIS_SARA_AE => {
                hints[i] = true; /* sara e/ae */
                i += 2; /* sara e/ae + the supposedly cons */
                if i >= str.len() {
                    break;
                }

                if str[i] == TIS_MAITAIKHU {
                    i += 2; /* MAITAIKHU + the supposedly cons */
                } else if str[i].is_upper_vowel() {
                    i += 1; /* the upper vowel, as part of composite vowel */
                    if i < str.len() && str[i].is_th_tone() {
                        i += 1;
                    }
                    i += 1; /* the supposedly cons */
                } else if i + 2 < str.len()
                    && ((str[i + 1] == TIS_SARA_AA && str[i + 2] == TIS_SARA_A)
                        || (str[i] != TIS_KO_KAI
                            && str[i + 1] == TIS_MAITAIKHU
                            && str[i + 2] != TIS_O_ANG
                            && str[i + 2] != TIS_WO_WAEN))
                {
                    i += 3; /* 2nd cons + SARA_AA + SARA_A, or
                             * 2nd cons + MAITAIKHU + final cons
                             */
                }
            }
            v if v.is_leading_vowel() => {
                hints[i] = true; /* the ldvowel */
                i += 2; /* the ldvowel + the supposedly cons */
            }
            TIS_RU | TIS_LU => {
                hints[i] = true;
                i += 1;
            }
            _ => i += 1,
        }
    }

    hints
}
