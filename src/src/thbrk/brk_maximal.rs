use ::libc;
extern "C" {
    pub type TrieState_Option_CTrieData;
    pub type Trie_Option_CTrieData;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn trie_state_is_single(s: *const TrieState) -> Bool;
    fn trie_state_is_walkable(s: *const TrieState, c: AlphaChar) -> Bool;
    fn trie_state_walk(s: *mut TrieState, c: AlphaChar) -> Bool;
    fn trie_root(trie: *const LegacyTrie) -> *mut TrieState;
    fn trie_state_copy(dst: *mut TrieState, src: *const TrieState);
    fn trie_state_clone(s: *const TrieState) -> *mut TrieState;
    fn trie_state_free(s: *mut TrieState);
    fn trie_state_rewind(s: *mut TrieState);
    fn th_tis2uni_line(s: *const thchar_t, result: *mut thwchar_t, n: size_t) -> libc::c_int;
    fn brk_brkpos_hints(str: *const thchar_t, len: libc::c_int, hints: *mut libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type AlphaChar = uint32_t;
pub type LegacyTrie = Trie_Option_CTrieData;
pub type TrieState = TrieState_Option_CTrieData;
pub type thchar_t = libc::c_uchar;
pub type thwchar_t = wchar_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ThBrk {
    pub dict_trie: *mut LegacyTrie,
}
pub type ThBrk = _ThBrk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BrkEnv {
    pub env_brk: *mut ThBrk,
    pub free_list: *mut BrkPool,
}
pub type BrkPool = _BrkPool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BrkPool {
    pub next: *mut BrkPool,
    pub shot: BrkShot,
}
pub type BrkShot = _BrkShot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BrkShot {
    pub dict_state: *mut TrieState,
    pub str_pos: libc::c_int,
    pub brk_pos: *mut libc::c_int,
    pub n_brk_pos: libc::c_int,
    pub cur_brk_pos: libc::c_int,
    pub penalty: libc::c_int,
}
pub type BrkEnv = _BrkEnv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BestBrk {
    pub brk_pos: *mut libc::c_int,
    pub n_brk_pos: libc::c_int,
    pub cur_brk_pos: libc::c_int,
    pub str_pos: libc::c_int,
    pub penalty: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RecovHist {
    pub pos: libc::c_int,
    pub recov: libc::c_int,
}
pub const INT_MAX: libc::c_int = __INT_MAX__;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn brk_maximal_do(
    mut s: *const thchar_t,
    mut len: libc::c_int,
    mut pos: *mut libc::c_int,
    mut n: size_t,
    mut env: *mut BrkEnv,
) -> libc::c_int {
    let mut brkpos_hints: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ws: *mut thwchar_t = 0 as *mut thwchar_t;
    let mut ret: libc::c_int = 0;
    brkpos_hints = malloc(len as libc::c_ulong) as *mut libc::c_char;
    if !brkpos_hints.is_null() {
        brk_brkpos_hints(s, len, brkpos_hints);
        ws = malloc(
            ((len + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<thwchar_t>() as libc::c_ulong),
        ) as *mut thwchar_t;
        if ws.is_null() {
            free(brkpos_hints as *mut libc::c_void);
        } else {
            th_tis2uni_line(s, ws, (len + 1 as libc::c_int) as size_t);
            ret = brk_maximal_do_impl(ws, len, brkpos_hints, pos, n, env);
            free(ws as *mut libc::c_void);
            free(brkpos_hints as *mut libc::c_void);
            return ret;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn brk_maximal_do_impl(
    mut ws: *const thwchar_t,
    mut len: libc::c_int,
    mut brkpos_hints: *const libc::c_char,
    mut pos: *mut libc::c_int,
    mut n: size_t,
    mut env: *mut BrkEnv,
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
        let mut is_terminal: libc::c_int = 0;
        let mut is_recovered: libc::c_int = 0;
        let mut str_pos: libc::c_int = 0;
        is_keep_node = 1 as libc::c_int;
        is_recovered = 0 as libc::c_int;
        str_pos = (*shot).str_pos;
        loop {
            let fresh0 = str_pos;
            str_pos = str_pos + 1;
            if trie_state_walk((*shot).dict_state, *ws.offset(fresh0 as isize) as AlphaChar) as u64
                == 0
            {
                let mut recovered: libc::c_int = 0;
                is_terminal = 0 as libc::c_int;
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
                        as libc::c_int;
                if str_pos >= len {
                    if is_terminal == 0 {
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
                } else if is_terminal != 0
                    && *brkpos_hints.offset(str_pos as isize) as libc::c_int != 0
                {
                    break;
                }
            }
        }
        (*shot).str_pos = str_pos;
        if is_keep_node != 0 && (is_terminal != 0 || is_recovered != 0) {
            if (*shot).str_pos < len
                && is_terminal != 0
                && trie_state_is_single((*shot).dict_state) as u64 == 0
            {
                let mut new_node: *mut BrkPool = brk_pool_node_new(shot, env);
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
        if is_keep_node == 0 || (*shot).str_pos == len || (*shot).cur_brk_pos as size_t >= n {
            best_brk_contest(best_brk, shot);
            pool = brk_pool_delete_node(pool, node, env);
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
                pool = brk_pool_delete_node(pool, del_node, env);
                pool_tail = next;
            }
        }
    }
    ret = (*best_brk).cur_brk_pos;
    memcpy(
        pos as *mut libc::c_void,
        (*best_brk).brk_pos as *const libc::c_void,
        (ret as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    brk_pool_free(pool, env);
    best_brk_free(best_brk);
    return ret;
}
unsafe extern "C" fn brk_recover_try(
    mut ws: *const thwchar_t,
    mut len: libc::c_int,
    mut brkpos_hints: *const libc::c_char,
    mut recov_words: size_t,
    mut last_brk_pos: *mut libc::c_int,
    mut env: *mut BrkEnv,
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
        let mut is_terminal: libc::c_int = 0;
        is_keep_node = 1 as libc::c_int;
        loop {
            loop {
                let fresh4 = (*shot).str_pos;
                (*shot).str_pos = (*shot).str_pos + 1;
                if trie_state_walk((*shot).dict_state, *ws.offset(fresh4 as isize) as AlphaChar)
                    as u64
                    == 0
                {
                    is_keep_node = 0 as libc::c_int;
                    break;
                } else {
                    is_terminal =
                        trie_state_is_walkable((*shot).dict_state, 0 as libc::c_int as AlphaChar)
                            as libc::c_int;
                    if (*shot).str_pos >= len {
                        if is_terminal == 0 {
                            is_keep_node = 0 as libc::c_int;
                        }
                        break;
                    } else if is_terminal != 0
                        && *brkpos_hints.offset((*shot).str_pos as isize) as libc::c_int != 0
                    {
                        break;
                    }
                }
            }
            if is_keep_node == 0 {
                pool = brk_pool_delete_node(pool, node, env);
                break;
            } else {
                if (*shot).str_pos < len && trie_state_is_single((*shot).dict_state) as u64 == 0 {
                    let mut new_node: *mut BrkPool = brk_pool_node_new(shot, env);
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
                if (*shot).str_pos == len || (*shot).cur_brk_pos as size_t == recov_words {
                    if (*shot).cur_brk_pos > ret {
                        ret = (*shot).cur_brk_pos;
                        *last_brk_pos =
                            *((*shot).brk_pos).offset((ret - 1 as libc::c_int) as isize);
                    }
                    pool = brk_pool_delete_node(pool, node, env);
                    if ret as size_t == recov_words {
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
                        pool = brk_pool_delete_node(pool, match_0, env);
                        pool_tail = next;
                    }
                }
            }
        }
    }
    brk_pool_free(pool, env);
    return ret;
}
unsafe extern "C" fn brk_root_pool(
    mut pos_size: libc::c_int,
    mut env: *mut BrkEnv,
) -> *mut BrkPool {
    let mut brk: *mut ThBrk = 0 as *mut ThBrk;
    let mut pool: *mut BrkPool = 0 as *mut BrkPool;
    let mut node: *mut BrkPool = 0 as *mut BrkPool;
    let mut root_shot: BrkShot = _BrkShot {
        dict_state: 0 as *mut TrieState,
        str_pos: 0,
        brk_pos: 0 as *mut libc::c_int,
        n_brk_pos: 0,
        cur_brk_pos: 0,
        penalty: 0,
    };
    pool = NULL as *mut BrkPool;
    brk = (*env).env_brk;
    if brk.is_null() {
        return NULL as *mut BrkPool;
    }
    root_shot.dict_state = trie_root((*brk).dict_trie);
    root_shot.brk_pos = NULL as *mut libc::c_int;
    root_shot.n_brk_pos = pos_size;
    root_shot.cur_brk_pos = 0 as libc::c_int;
    root_shot.str_pos = root_shot.cur_brk_pos;
    root_shot.penalty = 0 as libc::c_int;
    node = brk_pool_node_new(&mut root_shot, env);
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
    mut env: *mut BrkEnv,
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
                RECOVERED_WORDS as size_t,
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
    if !((*shot).dict_state).is_null() {
        trie_state_free((*shot).dict_state);
    }
    if !((*shot).brk_pos).is_null() {
        free((*shot).brk_pos as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn brk_env_new(mut brk: *mut ThBrk) -> *mut BrkEnv {
    let mut env: *mut BrkEnv =
        malloc(::core::mem::size_of::<BrkEnv>() as libc::c_ulong) as *mut BrkEnv;
    if env.is_null() {
        return NULL as *mut BrkEnv;
    }
    (*env).env_brk = brk;
    (*env).free_list = NULL as *mut BrkPool;
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn brk_env_free(mut env: *mut BrkEnv) {
    while !((*env).free_list).is_null() {
        let mut next: *mut BrkPool = 0 as *mut BrkPool;
        next = (*(*env).free_list).next;
        brk_shot_destruct(&mut (*(*env).free_list).shot);
        free((*env).free_list as *mut libc::c_void);
        (*env).free_list = next;
    }
    free(env as *mut libc::c_void);
}
unsafe extern "C" fn brk_pool_node_new(
    mut shot: *const BrkShot,
    mut env: *mut BrkEnv,
) -> *mut BrkPool {
    let mut node: *mut BrkPool = 0 as *mut BrkPool;
    if !((*env).free_list).is_null() {
        node = (*env).free_list;
        (*env).free_list = (*(*env).free_list).next;
        brk_shot_reuse(&mut (*node).shot, shot);
    } else {
        node = malloc(::core::mem::size_of::<BrkPool>() as libc::c_ulong) as *mut BrkPool;
        if node.is_null() {
            return NULL as *mut BrkPool;
        }
        if brk_shot_init(&mut (*node).shot, shot) != 0 as libc::c_int {
            free(node as *mut libc::c_void);
            return NULL as *mut BrkPool;
        }
    }
    (*node).next = NULL as *mut BrkPool;
    return node;
}
unsafe extern "C" fn brk_pool_node_free(mut pool: *mut BrkPool, mut env: *mut BrkEnv) {
    (*pool).next = (*env).free_list;
    (*env).free_list = pool;
}
unsafe extern "C" fn brk_pool_free(mut pool: *mut BrkPool, mut env: *mut BrkEnv) {
    while !pool.is_null() {
        let mut next: *mut BrkPool = 0 as *mut BrkPool;
        next = (*pool).next;
        brk_pool_node_free(pool, env);
        pool = next;
    }
}
unsafe extern "C" fn brk_pool_get_node(mut pool: *mut BrkPool) -> *mut BrkPool {
    let mut min_pos: libc::c_int = 0;
    let mut chosen: *mut BrkPool = 0 as *mut BrkPool;
    chosen = NULL as *mut BrkPool;
    min_pos = INT_MAX;
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
    mut env: *mut BrkEnv,
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
    brk_pool_node_free(node, env);
    return pool;
}
unsafe extern "C" fn best_brk_new(mut n_brk_pos: libc::c_int) -> *mut BestBrk {
    let mut best_brk: *mut BestBrk = 0 as *mut BestBrk;
    if n_brk_pos as size_t
        > (18446744073709551615 as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        return NULL as *mut BestBrk;
    }
    best_brk = malloc(::core::mem::size_of::<BestBrk>() as libc::c_ulong) as *mut BestBrk;
    if best_brk.is_null() {
        return NULL as *mut BestBrk;
    }
    (*best_brk).brk_pos = malloc(
        (n_brk_pos as size_t).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*best_brk).brk_pos).is_null() {
        free(best_brk as *mut libc::c_void);
        return NULL as *mut BestBrk;
    } else {
        (*best_brk).n_brk_pos = n_brk_pos;
        (*best_brk).str_pos = 0 as libc::c_int;
        (*best_brk).cur_brk_pos = (*best_brk).str_pos;
        (*best_brk).penalty = 0 as libc::c_int;
        return best_brk;
    };
}
unsafe extern "C" fn best_brk_free(mut best_brk: *mut BestBrk) {
    free((*best_brk).brk_pos as *mut libc::c_void);
    free(best_brk as *mut libc::c_void);
}
unsafe extern "C" fn best_brk_contest(
    mut best_brk: *mut BestBrk,
    mut shot: *const BrkShot,
) -> libc::c_int {
    if (*shot).str_pos > (*best_brk).str_pos
        || (*shot).str_pos == (*best_brk).str_pos
            && ((*shot).penalty < (*best_brk).penalty
                || (*shot).penalty == (*best_brk).penalty
                    && (*shot).cur_brk_pos <= (*best_brk).cur_brk_pos)
    {
        memcpy(
            (*best_brk).brk_pos as *mut libc::c_void,
            (*shot).brk_pos as *const libc::c_void,
            ((*shot).cur_brk_pos as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*best_brk).cur_brk_pos = (*shot).cur_brk_pos;
        (*best_brk).str_pos = (*shot).str_pos;
        (*best_brk).penalty = (*shot).penalty;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
