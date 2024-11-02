use crate::thbrk::common::brk_brkpos_hints_rs;
use crate::thbrk::ThBrk;
use crate::thctype::thchar_t;
use crate::thwchar::thwchar::th_tis2uni_line_rs;
use crate::thwctype::thwctype::thwchar_t;
use ::libc;
use datrie::AlphaChar;
use std::ptr::NonNull;
use std::{ptr, slice};

extern "C" {
    pub type TrieState_Option_CTrieData;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn trie_state_is_single(s: *const LegacyTrieState) -> Bool;
    fn trie_state_is_walkable(s: *const LegacyTrieState, c: AlphaChar) -> Bool;
    fn trie_state_walk(s: *mut LegacyTrieState, c: AlphaChar) -> Bool;
    fn trie_state_copy(dst: *mut LegacyTrieState, src: *const LegacyTrieState);
    fn trie_state_clone(s: *const LegacyTrieState) -> *mut LegacyTrieState;
    fn trie_state_free(s: *mut LegacyTrieState);
    fn trie_state_rewind(s: *mut LegacyTrieState);
}
pub type LegacyTrieState = TrieState_Option_CTrieData;

#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
struct Bool(u32);

const DA_TRUE: Bool = Bool(1);
const DA_FALSE: Bool = Bool(0);

#[derive(Default)]
pub(super) struct BrkEnv<'brk> {
    brk: Option<&'brk ThBrk>,
}

impl<'brk> BrkEnv<'brk> {
    pub(super) fn new(brk: Option<&'brk ThBrk>) -> Self {
        Self { brk }
    }
}

#[derive(Clone)]
#[repr(C)]
struct BrkPool {
    next: *mut BrkPool,
    shot: BrkShot,
}

#[derive(Clone)]
#[repr(C)]
struct BrkShot {
    dict_state: *mut LegacyTrieState,
    str_pos: libc::c_int,
    brk_pos: *mut libc::c_int,
    n_brk_pos: libc::c_int,
    cur_brk_pos: libc::c_int,
    penalty: libc::c_int,
}

impl BrkShot {
    // TODO: Deprecate this once the type settles
    fn brk_pos_slice(&self) -> &[i32] {
        unsafe { slice::from_raw_parts(self.brk_pos, self.cur_brk_pos as usize) }
    }
}

// impl Drop for BrkShot {
//     fn drop(&mut self) {
//         unsafe {
//             if !self.dict_state.is_null() {
//                 trie_state_free(self.dict_state);
//             }
//             if !self.brk_pos.is_null() {
//                 free(self.brk_pos as *mut libc::c_void);
//             }
//         }
//     }
// }

#[derive(Clone)]
#[repr(C)]
struct BestBrk {
    brk_pos: Box<[i32]>,
    cur_brk_pos: i32,
    str_pos: i32,
    penalty: i32,
}

impl BestBrk {
    fn new(n_brk_pos: usize) -> Self {
        assert!(n_brk_pos <= isize::MAX as usize / size_of::<i32>());

        Self {
            brk_pos: vec![0; n_brk_pos].into_boxed_slice(),
            cur_brk_pos: 0,
            str_pos: 0,
            penalty: 0,
        }
    }

    fn contest(&mut self, shot: &BrkShot) -> bool {
        if shot.str_pos > self.str_pos
            || shot.str_pos == self.str_pos
                && (shot.penalty < self.penalty
                    || shot.penalty == self.penalty && shot.cur_brk_pos <= self.cur_brk_pos)
        {
            self.clone_from_brkshot(shot);
            return true;
        }
        false
    }

    fn clone_from_brkshot(&mut self, shot: &BrkShot) {
        let shot_slice = shot.brk_pos_slice();
        self.brk_pos[..shot_slice.len()].clone_from_slice(shot_slice);
        self.cur_brk_pos = shot.cur_brk_pos;
        self.str_pos = shot.str_pos;
        self.penalty = shot.penalty;
    }
}

#[derive(Copy, Clone)]
pub struct RecovHist {
    pos: libc::c_int,
    recov: libc::c_int,
}

const NULL: libc::c_int = 0 as libc::c_int;

#[no_mangle]
pub(crate) fn brk_maximal_do(
    s: *const thchar_t,
    len: i32,
    mut pos: *mut i32,
    n: usize,
    env: &BrkEnv,
) -> libc::c_int {
    let s = unsafe { slice::from_raw_parts(s, len as usize) };
    // let pos = unsafe { slice::from_raw_parts_mut(pos, n as usize) };
    let brkpos_hints = brk_brkpos_hints_rs(s);

    let mut ws = th_tis2uni_line_rs(s);
    ws.push(0);

    unsafe {
        brk_maximal_do_impl(
            ws.as_ptr(),
            (ws.len() - 1) as libc::c_int,
            brkpos_hints.as_slice() as *const [bool] as *const i8,
            pos,
            n,
            env,
        )
    }
}

unsafe extern "C" fn brk_maximal_do_impl(
    mut ws: *const thwchar_t,
    mut len: libc::c_int,
    mut brkpos_hints: *const libc::c_char,
    mut pos: *mut libc::c_int,
    mut n: usize,
    env: &BrkEnv,
) -> libc::c_int {
    let mut pool: *mut BrkPool = 0 as *mut BrkPool;
    let mut node: *mut BrkPool = 0 as *mut BrkPool;
    let mut best_brk: *mut BestBrk = 0 as *mut BestBrk;
    let mut recov_hist: RecovHist = RecovHist { pos: 0, recov: 0 };
    let mut ret: libc::c_int = 0;
    pool = brk_root_pool(n as libc::c_int, env);
    best_brk = best_brk_new(n as libc::c_int);
    if best_brk.is_null() {
        return 0 as libc::c_int;
    }
    recov_hist.recov = -(1 as libc::c_int);
    recov_hist.pos = recov_hist.recov;
    loop {
        node = brk_pool_get_node(pool);
        if node.is_null() {
            break;
        }
        let mut shot: *mut BrkShot = &mut (*node).shot;
        let mut is_keep_node: libc::c_int = 0;
        let mut is_terminal = false;
        let mut is_recovered: libc::c_int = 0;
        let mut str_pos: libc::c_int = 0;
        is_keep_node = 1 as libc::c_int;
        is_recovered = 0 as libc::c_int;
        str_pos = (*shot).str_pos;
        loop {
            let fresh0 = str_pos;
            str_pos = str_pos + 1;
            if trie_state_walk((*shot).dict_state, *ws.offset(fresh0 as isize) as AlphaChar)
                == DA_FALSE
            {
                let mut recovered: libc::c_int = 0;
                is_terminal = false;
                recovered = brk_recover(
                    ws,
                    len,
                    (*shot).str_pos + 1 as libc::c_int,
                    brkpos_hints,
                    &mut recov_hist,
                    env,
                );
                if -(1 as libc::c_int) != recovered {
                    (*shot).penalty += recovered;
                    if (*shot).cur_brk_pos > 0 as libc::c_int {
                        (*shot).penalty -= *((*shot).brk_pos)
                            .offset(((*shot).cur_brk_pos - 1 as libc::c_int) as isize);
                    }
                    str_pos = recovered;
                    is_recovered = 1 as libc::c_int;
                } else {
                    (*shot).penalty += len;
                    if (*shot).cur_brk_pos > 0 as libc::c_int {
                        (*shot).penalty -= *((*shot).brk_pos)
                            .offset(((*shot).cur_brk_pos - 1 as libc::c_int) as isize);
                    }
                    str_pos = len;
                    let fresh1 = (*shot).cur_brk_pos;
                    (*shot).cur_brk_pos = (*shot).cur_brk_pos + 1;
                    *((*shot).brk_pos).offset(fresh1 as isize) = str_pos;
                    is_keep_node = 0 as libc::c_int;
                }
                break;
            } else {
                is_terminal =
                    trie_state_is_walkable((*shot).dict_state, 0 as libc::c_int as AlphaChar)
                        == DA_TRUE;
                if str_pos >= len {
                    if !is_terminal {
                        (*shot).penalty += len;
                        if (*shot).cur_brk_pos > 0 as libc::c_int {
                            (*shot).penalty -= *((*shot).brk_pos)
                                .offset(((*shot).cur_brk_pos - 1 as libc::c_int) as isize);
                        }
                        let fresh2 = (*shot).cur_brk_pos;
                        (*shot).cur_brk_pos = (*shot).cur_brk_pos + 1;
                        *((*shot).brk_pos).offset(fresh2 as isize) = len;
                        is_keep_node = 0 as libc::c_int;
                    }
                    break;
                } else if is_terminal && *brkpos_hints.offset(str_pos as isize) as libc::c_int != 0
                {
                    break;
                }
            }
        }
        (*shot).str_pos = str_pos;
        if is_keep_node != 0 && (is_terminal || is_recovered != 0) {
            if (*shot).str_pos < len
                && is_terminal
                && trie_state_is_single((*shot).dict_state) == DA_FALSE
            {
                let mut new_node: *mut BrkPool = brk_pool_node_new(shot);
                if !new_node.is_null() {
                    node = new_node;
                    pool = brk_pool_add(pool, node);
                    shot = &mut (*node).shot;
                }
            }
            trie_state_rewind((*shot).dict_state);
            let fresh3 = (*shot).cur_brk_pos;
            (*shot).cur_brk_pos = (*shot).cur_brk_pos + 1;
            *((*shot).brk_pos).offset(fresh3 as isize) = (*shot).str_pos;
        }
        if is_keep_node == 0 || (*shot).str_pos == len || (*shot).cur_brk_pos as usize >= n {
            best_brk_contest(NonNull::new_unchecked(best_brk), shot);
            pool = brk_pool_delete_node(pool, node);
        } else {
            let mut pool_tail: *mut BrkPool = pool;
            let mut match_0: *mut BrkPool = 0 as *mut BrkPool;
            loop {
                match_0 = brk_pool_match(pool_tail, node);
                if match_0.is_null() {
                    break;
                }
                let mut next: *mut BrkPool = (*match_0).next;
                let mut del_node: *mut BrkPool = 0 as *mut BrkPool;
                if (*match_0).shot.penalty < (*node).shot.penalty
                    || (*match_0).shot.penalty == (*node).shot.penalty
                        && (*match_0).shot.cur_brk_pos < (*node).shot.cur_brk_pos
                {
                    del_node = node;
                    if next == node {
                        next = (*node).next;
                    }
                    node = match_0;
                } else {
                    del_node = match_0;
                }
                pool = brk_pool_delete_node(pool, del_node);
                pool_tail = next;
            }
        }
    }
    ret = (*best_brk).cur_brk_pos;
    memcpy(
        pos as *mut libc::c_void,
        (*best_brk).brk_pos.as_ptr() as *const libc::c_void,
        (ret as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    brk_pool_free(pool);
    best_brk_free(NonNull::new_unchecked(best_brk));
    return ret;
}
unsafe extern "C" fn brk_recover_try(
    mut ws: *const thwchar_t,
    mut len: libc::c_int,
    mut brkpos_hints: *const libc::c_char,
    mut recov_words: usize,
    mut last_brk_pos: *mut libc::c_int,
    env: &BrkEnv,
) -> libc::c_int {
    let mut pool: *mut BrkPool = 0 as *mut BrkPool;
    let mut node: *mut BrkPool = 0 as *mut BrkPool;
    let mut ret: libc::c_int = 0;
    pool = brk_root_pool(recov_words as libc::c_int, env);
    ret = 0 as libc::c_int;
    's_13: loop {
        node = brk_pool_get_node(pool);
        if node.is_null() {
            break;
        }
        let mut shot: *mut BrkShot = &mut (*node).shot;
        let mut is_keep_node: libc::c_int = 0;
        let mut is_terminal = false;
        is_keep_node = 1 as libc::c_int;
        loop {
            loop {
                let fresh4 = (*shot).str_pos;
                (*shot).str_pos = (*shot).str_pos + 1;
                if trie_state_walk((*shot).dict_state, *ws.offset(fresh4 as isize) as AlphaChar)
                    == DA_FALSE
                {
                    is_keep_node = 0 as libc::c_int;
                    break;
                } else {
                    is_terminal =
                        trie_state_is_walkable((*shot).dict_state, 0 as libc::c_int as AlphaChar)
                            == DA_TRUE;
                    if (*shot).str_pos >= len {
                        if !is_terminal {
                            is_keep_node = 0 as libc::c_int;
                        }
                        break;
                    } else if is_terminal
                        && *brkpos_hints.offset((*shot).str_pos as isize) as libc::c_int != 0
                    {
                        break;
                    }
                }
            }
            if is_keep_node == 0 {
                pool = brk_pool_delete_node(pool, node);
                break;
            } else {
                if (*shot).str_pos < len && trie_state_is_single((*shot).dict_state) == DA_FALSE {
                    let mut new_node: *mut BrkPool = brk_pool_node_new(shot);
                    if !new_node.is_null() {
                        node = new_node;
                        pool = brk_pool_add(pool, node);
                        shot = &mut (*node).shot;
                    }
                }
                trie_state_rewind((*shot).dict_state);
                let fresh5 = (*shot).cur_brk_pos;
                (*shot).cur_brk_pos = (*shot).cur_brk_pos + 1;
                *((*shot).brk_pos).offset(fresh5 as isize) = (*shot).str_pos;
                if (*shot).str_pos == len || (*shot).cur_brk_pos as usize == recov_words {
                    if (*shot).cur_brk_pos > ret {
                        ret = (*shot).cur_brk_pos;
                        *last_brk_pos =
                            *((*shot).brk_pos).offset((ret - 1 as libc::c_int) as isize);
                    }
                    pool = brk_pool_delete_node(pool, node);
                    if ret as usize == recov_words {
                        break 's_13;
                    } else {
                        break;
                    }
                } else {
                    let mut pool_tail: *mut BrkPool = pool;
                    let mut match_0: *mut BrkPool = 0 as *mut BrkPool;
                    loop {
                        match_0 = brk_pool_match(pool_tail, node);
                        if match_0.is_null() {
                            break;
                        }
                        let mut next: *mut BrkPool = (*match_0).next;
                        pool = brk_pool_delete_node(pool, match_0);
                        pool_tail = next;
                    }
                }
            }
        }
    }
    brk_pool_free(pool);
    return ret;
}
unsafe extern "C" fn brk_root_pool(pos_size: i32, env: &BrkEnv) -> *mut BrkPool {
    let mut pool: *mut BrkPool = 0 as *mut BrkPool;
    let mut node: *mut BrkPool = 0 as *mut BrkPool;
    let mut root_shot: BrkShot = BrkShot {
        dict_state: 0 as *mut LegacyTrieState,
        str_pos: 0,
        brk_pos: 0 as *mut libc::c_int,
        n_brk_pos: 0,
        cur_brk_pos: 0,
        penalty: 0,
    };
    pool = NULL as *mut BrkPool;
    let brk = match (*env).brk {
        Some(v) => v,
        None => return ptr::null_mut(),
    };
    root_shot.dict_state =
        Box::into_raw(Box::new(brk.dict_trie.root())) as *mut TrieState_Option_CTrieData;
    root_shot.brk_pos = NULL as *mut libc::c_int;
    root_shot.n_brk_pos = pos_size;
    root_shot.cur_brk_pos = 0 as libc::c_int;
    root_shot.str_pos = root_shot.cur_brk_pos;
    root_shot.penalty = 0 as libc::c_int;
    node = brk_pool_node_new(&mut root_shot);
    if !node.is_null() {
        pool = brk_pool_add(pool, node);
    }
    brk_shot_destruct(&mut root_shot);
    return pool;
}
pub const RECOVERED_WORDS: libc::c_int = 3 as libc::c_int;
unsafe extern "C" fn brk_recover(
    mut wtext: *const thwchar_t,
    mut len: libc::c_int,
    mut pos: libc::c_int,
    mut brkpos_hints: *const libc::c_char,
    mut rh: *mut RecovHist,
    env: &BrkEnv,
) -> libc::c_int {
    let mut last_brk_pos: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    while pos < len && *brkpos_hints.offset(pos as isize) == 0 {
        pos += 1;
        pos;
    }
    if (*rh).pos == pos {
        return (*rh).recov;
    }
    p = pos;
    while p < len {
        if *brkpos_hints.offset(p as isize) != 0 {
            n = brk_recover_try(
                wtext.offset(p as isize),
                len - p,
                brkpos_hints.offset(p as isize),
                RECOVERED_WORDS as usize,
                &mut last_brk_pos,
                env,
            );
            if n == RECOVERED_WORDS
                || n > 0 as libc::c_int && '\0' as i32 == *wtext.offset(last_brk_pos as isize)
            {
                (*rh).pos = pos;
                (*rh).recov = p;
                return p;
            }
        }
        p += 1;
        p;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn brk_shot_init(mut dst: *mut BrkShot, mut src: *const BrkShot) -> libc::c_int {
    (*dst).dict_state = trie_state_clone((*src).dict_state);
    (*dst).str_pos = (*src).str_pos;
    (*dst).brk_pos = malloc(
        ((*src).n_brk_pos as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*dst).brk_pos).is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(
        (*dst).brk_pos as *mut libc::c_void,
        (*src).brk_pos as *const libc::c_void,
        ((*src).cur_brk_pos as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*dst).n_brk_pos = (*src).n_brk_pos;
    (*dst).cur_brk_pos = (*src).cur_brk_pos;
    (*dst).penalty = (*src).penalty;
    return 0 as libc::c_int;
}
unsafe extern "C" fn brk_shot_reuse(mut dst: *mut BrkShot, mut src: *const BrkShot) {
    trie_state_copy((*dst).dict_state, (*src).dict_state);
    (*dst).str_pos = (*src).str_pos;
    if (*dst).n_brk_pos < (*src).n_brk_pos {
        (*dst).brk_pos = realloc(
            (*dst).brk_pos as *mut libc::c_void,
            ((*src).n_brk_pos as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    memcpy(
        (*dst).brk_pos as *mut libc::c_void,
        (*src).brk_pos as *const libc::c_void,
        ((*src).cur_brk_pos as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*dst).n_brk_pos = (*src).n_brk_pos;
    (*dst).cur_brk_pos = (*src).cur_brk_pos;
    (*dst).penalty = (*src).penalty;
}

unsafe extern "C" fn brk_shot_destruct(mut shot: *mut BrkShot) {
    // XXX: This can't be ported to Drop impl, as shot can be a stack pointer
    // and not a Box
    if !((*shot).dict_state).is_null() {
        trie_state_free((*shot).dict_state);
    }
    if !((*shot).brk_pos).is_null() {
        free((*shot).brk_pos as *mut libc::c_void);
    }
}

unsafe extern "C" fn brk_pool_node_new(mut shot: *const BrkShot) -> *mut BrkPool {
    // TODO: Originally this can reused freed nodes from a free list
    let mut node: *mut BrkPool = 0 as *mut BrkPool;
    node = malloc(::core::mem::size_of::<BrkPool>() as libc::c_ulong) as *mut BrkPool;
    if node.is_null() {
        return NULL as *mut BrkPool;
    }
    if brk_shot_init(&mut (*node).shot, shot) != 0 as libc::c_int {
        free(node as *mut libc::c_void);
        return NULL as *mut BrkPool;
    }
    (*node).next = NULL as *mut BrkPool;
    return node;
}

unsafe extern "C" fn brk_pool_node_free(mut pool: *mut BrkPool) {
    brk_shot_destruct((&mut (*pool).shot) as *mut BrkShot);
    free(pool as *mut libc::c_void);
}

unsafe extern "C" fn brk_pool_free(mut pool: *mut BrkPool) {
    while !pool.is_null() {
        let mut next: *mut BrkPool = 0 as *mut BrkPool;
        next = (*pool).next;
        brk_pool_node_free(pool);
        pool = next;
    }
}
unsafe extern "C" fn brk_pool_get_node(mut pool: *mut BrkPool) -> *mut BrkPool {
    let mut min_pos: libc::c_int = 0;
    let mut chosen: *mut BrkPool = 0 as *mut BrkPool;
    chosen = NULL as *mut BrkPool;
    min_pos = i32::MAX;
    while !pool.is_null() {
        let mut pos: libc::c_int = 0;
        if 0 as libc::c_int == (*pool).shot.cur_brk_pos {
            return pool;
        }
        pos =
            *((*pool).shot.brk_pos).offset(((*pool).shot.cur_brk_pos - 1 as libc::c_int) as isize);
        if pos < min_pos {
            min_pos = pos;
            chosen = pool;
        }
        pool = (*pool).next;
    }
    return chosen;
}
unsafe extern "C" fn brk_pool_match(
    mut pool: *mut BrkPool,
    mut node: *const BrkPool,
) -> *mut BrkPool {
    let mut node_cur_pos: libc::c_int = 0;
    node_cur_pos = (*node).shot.cur_brk_pos;
    if node_cur_pos == 0 as libc::c_int {
        while !pool.is_null() {
            if pool != node as *mut BrkPool && (*pool).shot.cur_brk_pos == 0 as libc::c_int {
                break;
            }
            pool = (*pool).next;
        }
    } else {
        let mut node_brk_pos: libc::c_int =
            *((*node).shot.brk_pos).offset((node_cur_pos - 1 as libc::c_int) as isize);
        while !pool.is_null() {
            if pool != node as *mut BrkPool
                && (*pool).shot.cur_brk_pos > 0 as libc::c_int
                && *((*pool).shot.brk_pos)
                    .offset(((*pool).shot.cur_brk_pos - 1 as libc::c_int) as isize)
                    == node_brk_pos
            {
                break;
            }
            pool = (*pool).next;
        }
    }
    return pool;
}
unsafe extern "C" fn brk_pool_add(mut pool: *mut BrkPool, mut node: *mut BrkPool) -> *mut BrkPool {
    (*node).next = pool;
    return node;
}
unsafe extern "C" fn brk_pool_delete_node(
    mut pool: *mut BrkPool,
    mut node: *mut BrkPool,
) -> *mut BrkPool {
    if pool == node {
        pool = (*pool).next;
    } else {
        let mut p: *mut BrkPool = 0 as *mut BrkPool;
        p = pool;
        while !p.is_null() && (*p).next != node {
            p = (*p).next;
        }
        if !p.is_null() {
            (*p).next = (*node).next;
        }
    }
    brk_pool_node_free(node);
    return pool;
}

#[deprecated(note = "Use BestBrk::new()")]
extern "C" fn best_brk_new(n_brk_pos: libc::c_int) -> *mut BestBrk {
    Box::into_raw(Box::new(BestBrk::new(n_brk_pos as usize)))
}

#[deprecated(note = "Drop best_brk")]
unsafe extern "C" fn best_brk_free(mut best_brk: NonNull<BestBrk>) {
    drop(Box::from_raw(best_brk.as_ptr()));
}

#[deprecated(note = "Use best_brk.contest(shot)")]
extern "C" fn best_brk_contest(mut best_brk: NonNull<BestBrk>, shot: *const BrkShot) -> i32 {
    let best_brk = unsafe { best_brk.as_mut() };
    match best_brk.contest(unsafe { &*shot }) {
        true => 1,
        false => 0,
    }
}
