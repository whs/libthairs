#ifndef THAI_THAILIB_H
#define THAI_THAILIB_H

/* Generated with cbindgen:0.27.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "stddef.h"

#define TIS_KO_KAI (int)161

#define TIS_RU (int)196

#define TIS_LU (int)198

#define TIS_WO_WAEN (int)199

#define TIS_O_ANG (int)205

#define TIS_SARA_A (int)208

#define TIS_MAI_HAN_AKAT (int)209

#define TIS_SARA_AA (int)210

#define TIS_SARA_UEE (int)215

#define TIS_SARA_E (int)224

#define TIS_SARA_AE (int)225

#define TIS_MAITAIKHU (int)231

#define TIS_THANTHAKHAT (int)236

#define INT_MAX __INT_MAX__

#define NULL (int)0

#define __INT_MAX__ (int)2147483647

#define RECOVERED_WORDS (int)3

#define MAX_ACRONYM_FRAG_LEN (int)3

#define TIS_SARA_AM (int)211

#define TOT_LEVELS (int)4

#define IGNORE (int)0

#define _none (int)0

#define _cntrl (int)_th_IStis

#define _space (int)_th_IStis

#define _edigit ((int)_th_IStis | (int)_th_ISdigit)

#define _elower (int)_th_IStis

#define _eupper (int)_th_IStis

#define _epunct ((int)_th_IStis | (int)_th_ISpunct)

#define _tdigit ((int)_th_IStis | (int)_th_ISdigit)

#define _tcons ((int)_th_IStis | (int)_th_IScons)

#define _tflvowel ((int)_th_IStis | (int)_th_VCflvowel)

#define _tldvowel ((int)_th_IStis | (int)_th_VCldvowel)

#define _tupvowel ((int)_th_IStis | (int)_th_VCupvowel)

#define _tblvowel ((int)_th_IStis | (int)_th_VCblvowel)

#define _ttone ((int)_th_IStis | (int)_th_IStone)

#define _tdiac ((int)_th_IStis | (int)_th_ISdiac)

#define _tpunct ((int)_th_IStis | (int)_th_ISpunct)

#define TIS_LAKKHANGYAO (int)229

#define TIS_NIKHAHIT (int)237

#define TH_BLANK_BASE_GLYPH (int)221

#define TIS_YO_YING (int)173

#define TIS_SARA_U (int)216

#define SIZE_MAX (unsigned long)18446744073709551615ull

#define THCHAR_ERR ~(int)0

#define TH_ERR ~(int)0

#define MAXLINELENGTH (int)100

#define TIS_NO_NU (int)185

#define TIS_SARA_I (int)212

#define TIS_SARA_II (int)213

#define TIS_MAI_EK (int)232

#define TIS_MAI_THO (int)233

#define TESTCELLS (int)10

#define MAX_DATA (int)40000

typedef struct BrkClass BrkClass;

typedef struct BrkOp BrkOp;

typedef ROTrie<Option<CTrieData>> ThTrie;

typedef uint8_t thchar_t;

typedef unsigned int Bool;

typedef TrieState_Option_CTrieData TrieState;

typedef uint32_t AlphaChar;

typedef Trie_Option_CTrieData LegacyTrie;

typedef int wchar_t;

typedef wchar_t thwchar_t;

typedef struct ThBrk {
  ThTrie dict_trie;
} ThBrk;

typedef struct _BrkShot {
  TrieState *dict_state;
  int str_pos;
  int *brk_pos;
  int n_brk_pos;
  int cur_brk_pos;
  int penalty;
} _BrkShot;

typedef struct _BrkShot BrkShot;

typedef struct _BrkPool {
  BrkPool *next;
  BrkShot shot;
} _BrkPool;

typedef struct _BrkPool BrkPool;

typedef struct _BrkEnv {
  struct ThBrk *env_brk;
  BrkPool *free_list;
} _BrkEnv;

typedef struct _BrkEnv BrkEnv;

typedef struct thcell_t {
  thchar_t base;
  thchar_t hilo;
  thchar_t top;
} thcell_t;

typedef unsigned int WTTClass;

typedef unsigned int WTTOp;

typedef unsigned int thstrict_t;

typedef struct thinpconv_t {
  thchar_t conv[4];
  int offset;
} thinpconv_t;

typedef unsigned char thglyph_t;

typedef void *iconv_t;

typedef struct _Sample {
  const char *str_0;
  int n_brk;
  int brk_pos[100];
  const char *ins_str;
} _Sample;

typedef struct _Sample Sample;

typedef long __off_t;

typedef long __off64_t;

typedef struct _IO_FILE {
  int _flags;
  char *_IO_read_ptr;
  char *_IO_read_end;
  char *_IO_read_base;
  char *_IO_write_base;
  char *_IO_write_ptr;
  char *_IO_write_end;
  char *_IO_buf_base;
  char *_IO_buf_end;
  char *_IO_save_base;
  char *_IO_backup_base;
  char *_IO_save_end;
  _IO_marker *_markers;
  struct _IO_FILE *_chain;
  int _fileno;
  int _flags2;
  __off_t _old_offset;
  unsigned short _cur_column;
  signed char _vtable_offset;
  char _shortbuf[1];
  void *_lock;
  __off64_t _offset;
  _IO_codecvt *_codecvt;
  _IO_wide_data *_wide_data;
  struct _IO_FILE *_freeres_list;
  void *_freeres_buf;
  struct _IO_FILE **_prevchain;
  int _mode;
  char _unused2[20];
} _IO_FILE;

typedef struct _IO_FILE FILE;

typedef struct char_range {
  thchar_t begin;
  thchar_t end;
} char_range;

typedef int (*__compar_fn_t)(const void*, const void*);

typedef unsigned int C2RustUnnamed;

typedef unsigned int C2RustUnnamed_0;

typedef unsigned int l3_symbols;

typedef unsigned int l4_symbols;

typedef unsigned int l2_symbols;

typedef unsigned int l1_symbols;

#define _th_ISpunct 1024

#define _th_ISdigit 512

#define _th_ISdiac 256

#define _th_IStone 128

#define _th_VCblvowel 112

#define _th_VCupvowel 80

#define _th_VCldvowel 48

#define _th_VCflvowel 16

#define _th_VClassMsk 112

#define _th_ISvowel 16

#define _th_CCundersplit 14

#define _th_CCundershoot 10

#define _th_CCovershoot 6

#define _th_CCtailless 2

#define _th_CClassMsk 14

#define _th_IScons 2

#define _th_IStis 1







#define DA_TRUE 1

#define DA_FALSE 0

#define CP 1

#define SR 5

#define RJ 4

#define AC 3

#define XC 2

#define L3_KHOMUT 34

#define L3_ANGKHANKHU 33

#define L4_BLK 4

#define L3_BLANK 4

#define L2_THAII 5

#define L1_98 13

#define L1_88 12

#define L1_78 11

#define L1_68 10

#define L1_58 9

#define L1_48 8

#define L1_38 7

#define L1_28 6

#define L1_18 5

#define L1_08 4

#define L3_FONGMAN 32

#define L2_YAMAK 6

#define L2_BLANK 4

#define L1_NKHIT 86

#define L2_GARAN 8

#define L2_TONE4 13

#define L2_TONE3 12

#define L2_TONE2 11

#define L2_TONE1 10

#define L2_TYKHU 9

#define L3_MAI_YAMOK 17

#define L4_EXT 7

#define L1_SARA_AA 89

#define L1_SARA_AI_MAIMALAI 101

#define L1_SARA_AI_MAIMUAN 100

#define L1_SARA_O 99

#define L1_SARA_AE 98

#define L1_SARA_E 97

#define L3_BAHT 30

#define L2_PINTU 7

#define L1_SARA_UU 96

#define L1_SARA_U 95

#define L1_SARA_UEE 94

#define L1_SARA_UE 93

#define L1_SARA_II 92

#define L1_SARA_I 91

#define L1_SARA_AM 90

#define L1_MAI_HAN_AKAT 88

#define L1_SARA_A 87

#define L3_PAIYAN_NOI 16

#define L1_HO_NOKHUK 85

#define L1_O_ANG 84

#define L1_LO_CHULA 83

#define L1_HO_HIP 82

#define L1_SO_SUA 81

#define L1_SO_RUSI 80

#define L1_SO_SALA 79

#define L1_WO_WAEN 78

#define L1_LU 77

#define L1_LO_LING 76

#define L1_RU 75

#define L1_RO_RUA 74

#define L1_YO_YAK 73

#define L1_MO_MA 72

#define L1_PHO_SAMPHAO 71

#define L1_FO_FAN 70

#define L1_PHO_PHAN 69

#define L1_FO_FA 68

#define L1_PHO_PHUNG 67

#define L1_PO_PLA 66

#define L1_BO_BAIMAI 65

#define L1_NO_NU 64

#define L1_THO_THONG 63

#define L1_THO_THAHAN 62

#define L1_THO_THUNG 61

#define L1_TO_TAO 60

#define L1_DO_DEK 59

#define L1_NO_NEN 58

#define L1_THO_PHUTHAO 57

#define L1_THO_NANGMONTHO 56

#define L1_THO_THAN 55

#define L1_TO_PATAK 54

#define L1_DO_CHADA 53

#define L1_YO_YING 52

#define L1_CHO_CHOE 51

#define L1_SO_SO 50

#define L1_CHO_CHANG 49

#define L1_CHO_CHING 48

#define L1_CHO_CHAN 47

#define L1_NGO_NGU 46

#define L1_KHO_RAKHANG 45

#define L1_KHO_KHON 44

#define L1_KHO_KHWAI 43

#define L1_KHO_KHUAT 42

#define L1_KHO_KHAI 41

#define L1_KO_KAI 40

#define L3_NB_SACE 6

#define L3_TILDE 20

#define L3_R_BRACE 26

#define L3_V_LINE 44

#define L3_L_BRACE 25

#define L4_MIN 5

#define L1_Z8 39

#define L1_Y8 38

#define L1_X8 37

#define L1_W8 36

#define L1_V8 35

#define L1_U8 34

#define L1_T8 33

#define L1_S8 32

#define L1_R8 31

#define L1_Q8 30

#define L1_P8 29

#define L1_O8 28

#define L1_N8 27

#define L1_M8 26

#define L1_L8 25

#define L1_K8 24

#define L1_J8 23

#define L1_I8 22

#define L1_H8 21

#define L1_G8 20

#define L1_F8 19

#define L1_E8 18

#define L1_D8 17

#define L1_C8 16

#define L1_B8 15

#define L1_A8 14

#define L3_GRAVE 18

#define L3_LOW_LINE 7

#define L3_CIRCUMFLEX 19

#define L3_R_BRACKET 27

#define L3_BK_SOLIDUS 36

#define L3_L_BRACKET 24

#define L4_CAP 6

#define L3_AT 29

#define L3_QUESTION 13

#define L3_GREATER_THAN 43

#define L3_EQUAL 42

#define L3_LESS_THAN 41

#define L3_SEMICOLON 10

#define L3_COLON 11

#define L3_SOLIDUS 14

#define L3_FULL_STOP 15

#define L3_HYPHEN 8

#define L3_COMMA 9

#define L3_PLUS 40

#define L3_ASTERISK 35

#define L3_R_PARENTHESIS 28

#define L3_L_PARANTHESIS 23

#define L3_APOSTROPHE 21

#define L3_AMPERSAND 37

#define L3_PERCENT 39

#define L3_DOLLAR 31

#define L3_NUMBER 38

#define L3_QUOTATION 22

#define L3_EXCLAMATION 12

#define L3_SPACE 5

#define AV3 16

#define AV2 15

#define AV1 14

#define AD3 13

#define AD2 12

#define AD1 11

#define TONE 10

#define BD 9

#define BV2 8

#define BV1 7

#define FV3 6

#define FV2 5

#define FV1 4

#define LV 3

#define CONS 2

#define NON 1

#define CTRL 0

#define ISC_STRICT 2

#define ISC_BASICCHECK 1

#define ISC_PASSTHROUGH 0

#define THWCHAR_ERR ~(int)0

#define WC_ERR THWCHAR_ERR

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern unsigned short _th_ctype_tbl[256];

extern int _th_chlevel_tbl[256];

extern short TACchtype_[256];

extern short TACio_op_[17][17];

extern Sample TestSamples[12];

extern iconv_t utf8_to_tis_iconv;

extern iconv_t tis_to_utf8_iconv;

extern struct char_range tis_ranges[4];

extern struct char_range thai_ranges[3];

extern struct char_range eng_ranges[2];

extern struct char_range thcons_ranges[4];

extern struct char_range tlcons_ranges[9];

extern struct char_range oscons_ranges[5];

extern struct char_range uscons_ranges[2];

extern struct char_range spcons_ranges[3];

extern struct char_range thvowel_ranges[5];

extern struct char_range ldvowel_ranges[2];

extern struct char_range flvowel_ranges[6];

extern struct char_range upvowel_ranges[3];

extern struct char_range blvowel_ranges[2];

extern struct char_range thtone_ranges[2];

extern struct char_range thdiac_ranges[4];

extern struct char_range thdigit_ranges[3];

extern struct char_range thpunct_ranges[10];

extern thchar_t test_keys[55];

extern thchar_t res_level0[55];

extern thchar_t res_level1[44];

extern thchar_t res_level2[38];

extern thchar_t res_validate[45];

extern void *memset(void*, int, unsigned long);

extern unsigned long strlen(const char*);

ThTrie *brk_load_default_dict(void);

void brk_brkpos_hints(const thchar_t *str, int len, char *hints);

struct BrkClass brk_class(thchar_t c);

struct BrkOp brk_op(struct BrkClass prev, struct BrkClass next);

extern void *memcpy(void*, const void*, unsigned long);

extern void *malloc(unsigned long);

extern void *realloc(void*, unsigned long);

extern void free(void*);

extern Bool trie_state_is_single(const TrieState *s);

extern Bool trie_state_is_walkable(const TrieState *s, AlphaChar c);

extern Bool trie_state_walk(TrieState *s, AlphaChar c);

extern TrieState *trie_root(const LegacyTrie *trie);

extern void trie_state_copy(TrieState *dst, const TrieState *src);

extern TrieState *trie_state_clone(const TrieState *s);

extern void trie_state_free(TrieState *s);

extern void trie_state_rewind(TrieState *s);

extern int th_tis2uni_line(const thchar_t *s, thwchar_t *result, size_t n);

extern void brk_brkpos_hints(const thchar_t *str, int len, char *hints);

int brk_maximal_do(const thchar_t *s, int len, int *pos, size_t n, BrkEnv *env);

BrkEnv *brk_env_new(struct ThBrk *brk);

void brk_env_free(BrkEnv *env);

extern char *strcpy(char*, const char*);

extern size_t strlen(const char*);

extern void *malloc(size_t);

extern void free(void*);

extern BrkEnv *brk_env_new(struct ThBrk *brk);

extern void brk_env_free(BrkEnv *env);

extern int brk_maximal_do(const thchar_t *s, int len, int *pos, size_t n, BrkEnv *env);

struct ThBrk *th_brk_new(const char *dictpath);

void th_brk_delete(struct ThBrk *brk);

int th_brk_insert_breaks(struct ThBrk *brk,
                         const thchar_t *in_0,
                         thchar_t *out,
                         size_t out_sz,
                         const char *delim);

int th_brk_find_breaks(struct ThBrk *brk, const thchar_t *s, int *pos, size_t pos_sz);

int th_brk_line(const thchar_t *in_0, thchar_t *out, size_t out_sz, const char *delim);

int th_brk(const thchar_t *s, int *pos, size_t pos_sz);

/**
 * Get the global, shared instance of ThBrk
 *
 * The Rust version of this is thread safe
 */
const struct ThBrk *brk_get_shared_brk(void);

/**
 * Does nothing in the Rust version
 */
void brk_free_shared_brk(void);

void th_init_cell(struct thcell_t *cell);

size_t th_next_cell(const thchar_t *s, size_t len, struct thcell_t *cell, int is_decomp_am);

size_t th_prev_cell(const thchar_t *s, size_t pos, struct thcell_t *cell, int is_decomp_am);

size_t th_make_cells(const thchar_t *s,
                     size_t len,
                     struct thcell_t *cells,
                     size_t *ncells,
                     int is_decomp_am);

thchar_t th_char_weight_(thchar_t c, int level);

thchar_t th_char_weight_delim_(int level);

extern thchar_t th_char_weight_(thchar_t c, int level);

extern thchar_t th_char_weight_delim_(int level);

int th_strcoll(const thchar_t *s1, const thchar_t *s2);

size_t th_strxfrm(thchar_t *dest, const thchar_t *src, size_t n);

int th_istis(thchar_t c);

int th_isthai(thchar_t c);

int th_iseng(thchar_t c);

int th_isthcons(thchar_t c);

int th_isthvowel(thchar_t c);

int th_isthtone(thchar_t c);

int th_isthdiac(thchar_t c);

int th_isthdigit(thchar_t c);

int th_isthpunct(thchar_t c);

int th_istaillesscons(thchar_t c);

int th_isovershootcons(thchar_t c);

int th_isundershootcons(thchar_t c);

int th_isundersplitcons(thchar_t c);

int th_isldvowel(thchar_t c);

int th_isflvowel(thchar_t c);

int th_isupvowel(thchar_t c);

int th_isblvowel(thchar_t c);

int th_chlevel(thchar_t c);

int th_iscombchar(thchar_t c);

WTTClass TACchtype(thchar_t c);

WTTOp TACio_op(thchar_t c1, thchar_t c2);

extern char *strcpy(char*, const char*);

int th_isaccept(thchar_t c1, thchar_t c2, thstrict_t s);

int th_validate(struct thcell_t context, thchar_t c, struct thinpconv_t *conv);

int th_validate_leveled(struct thcell_t context,
                        thchar_t c,
                        struct thinpconv_t *conv,
                        thstrict_t s);

extern unsigned long strlen(const char*);

extern size_t th_next_cell(const thchar_t *s, size_t len, struct thcell_t *cell, int is_decomp_am);

int th_render_cell_tis(struct thcell_t cell, thglyph_t *res, size_t res_sz, int is_decomp_am);

int th_render_cell_win(struct thcell_t cell, thglyph_t *res, size_t res_sz, int is_decomp_am);

int th_render_cell_mac(struct thcell_t cell, thglyph_t *res, size_t res_sz, int is_decomp_am);

int th_render_text_tis(const thchar_t *s, thglyph_t *res, size_t res_sz, int is_decomp_am);

int th_render_text_win(const thchar_t *s, thglyph_t *res, size_t res_sz, int is_decomp_am);

int th_render_text_mac(const thchar_t *s, thglyph_t *res, size_t res_sz, int is_decomp_am);

size_t th_normalize(thchar_t *dest, const thchar_t *src, size_t n);

extern void *malloc(unsigned long);

extern void free(void*);

extern wchar_t *wcscpy(wchar_t *__dest, const wchar_t *__src);

extern unsigned long wcslen(const int*);

extern int th_uni2tis_line(const thwchar_t *s, thchar_t *result, size_t n);

extern int th_brk_find_breaks(struct ThBrk *brk, const thchar_t *s, int *pos, size_t pos_sz);

int th_brk_wc_find_breaks(struct ThBrk *brk, const thwchar_t *s, int *pos, size_t pos_sz);

int th_brk_wc_insert_breaks(struct ThBrk *brk,
                            const thwchar_t *in_0,
                            thwchar_t *out,
                            size_t out_sz,
                            const thwchar_t *delim);

int th_wbrk(const thwchar_t *s, int *pos, size_t pos_sz);

int th_wbrk_line(const thwchar_t *in_0, thwchar_t *out, size_t out_sz, const thwchar_t *delim);

thwchar_t th_tis2uni(thchar_t c);

int th_tis2uni_line(const thchar_t *s, thwchar_t *result, size_t n);

thwchar_t th_winthai2uni(thchar_t c);

thwchar_t th_macthai2uni(thchar_t c);

thchar_t th_uni2tis(thwchar_t wc);

int th_uni2tis_line(const thwchar_t *s, thchar_t *result, size_t n);

thchar_t th_uni2winthai(thwchar_t wc);

thchar_t th_uni2macthai(thwchar_t wc);

extern thchar_t th_uni2tis(thwchar_t wc);

int th_wcistis(thwchar_t wc);

int th_wcisthai(thwchar_t wc);

int th_wciseng(thwchar_t wc);

int th_wcisthcons(thwchar_t wc);

int th_wcisthvowel(thwchar_t wc);

int th_wcisthtone(thwchar_t wc);

int th_wcisthdiac(thwchar_t wc);

int th_wcisthdigit(thwchar_t wc);

int th_wcisthpunct(thwchar_t wc);

int th_wcistaillesscons(thwchar_t wc);

int th_wcisovershootcons(thwchar_t wc);

int th_wcisundershootcons(thwchar_t wc);

int th_wcisundersplitcons(thwchar_t wc);

int th_wcisldvowel(thwchar_t wc);

int th_wcisflvowel(thwchar_t wc);

int th_wcisupvowel(thwchar_t wc);

int th_wcisblvowel(thwchar_t wc);

int th_wcchlevel(thwchar_t wc);

extern thchar_t th_uni2tis(thwchar_t wc);

extern thwchar_t th_tis2uni(thchar_t c);

extern size_t th_normalize(thchar_t *dest, const thchar_t *src, size_t n);

extern void free(void*);

extern void *malloc(unsigned long);

size_t th_wnormalize(thwchar_t *wdest, const thwchar_t *wsrc, size_t n);

extern int printf(const char*, ...);

extern void exit(int);

extern int strcmp(const char*, const char*);

extern unsigned long strlen(const char*);

extern struct ThBrk *th_brk_new(const char *dictpath);

extern void th_brk_delete(struct ThBrk *brk);

extern int th_brk_find_breaks(struct ThBrk *brk, const thchar_t *s, int *pos, size_t pos_sz);

extern int th_brk_insert_breaks(struct ThBrk *brk,
                                const thchar_t *in_0,
                                thchar_t *out,
                                size_t out_sz,
                                const char *delim);

extern int iconv_close(iconv_t __cd);

extern iconv_t iconv_open(const char *__tocode, const char *__fromcode);

extern size_t iconv(iconv_t __cd,
                    char **__inbuf,
                    size_t *__inbytesleft,
                    char **__outbuf,
                    size_t *__outbytesleft);

void init_iconv(void);

void close_iconv(void);

size_t utf8_to_tis(const char *utf8_str, thchar_t *tis, size_t tis_sz);

size_t tis_to_utf8(const thchar_t *tis_str, char *utf8, size_t utf8_sz);

void show_breaks(int *brk_pos, int n_brk);

int test_samples(struct ThBrk *brk, const Sample *samples);

extern size_t th_next_cell(const thchar_t *s, size_t len, struct thcell_t *cell, int is_decomp_am);

extern size_t th_prev_cell(const thchar_t *s, size_t pos, struct thcell_t *cell, int is_decomp_am);

extern size_t th_make_cells(const thchar_t *s,
                            size_t len,
                            struct thcell_t *cells,
                            size_t *ncells,
                            int is_decomp_am);

extern unsigned long strlen(const char*);

extern int fprintf(FILE*, const char*, ...);

extern void *malloc(unsigned long);

extern void free(void*);

int test_th_next_cell(void);

int test_th_prev_cell(void);

int test_th_make_cells(void);

extern int th_isblvowel(thchar_t c);

extern int th_isupvowel(thchar_t c);

extern int th_isflvowel(thchar_t c);

extern int th_isldvowel(thchar_t c);

extern int th_isundersplitcons(thchar_t c);

extern int th_isundershootcons(thchar_t c);

extern int th_isovershootcons(thchar_t c);

extern int th_istaillesscons(thchar_t c);

extern int th_isthpunct(thchar_t c);

extern int th_isthdigit(thchar_t c);

extern int th_isthdiac(thchar_t c);

extern int th_isthtone(thchar_t c);

extern int th_isthvowel(thchar_t c);

extern int th_isthcons(thchar_t c);

extern int th_iseng(thchar_t c);

extern int th_isthai(thchar_t c);

extern int th_istis(thchar_t c);

extern int fprintf(FILE*, const char*, ...);

int test_bool_funcs(const struct char_range *ranges, int (*fn_0)(thchar_t));

extern int th_isaccept(thchar_t c1, thchar_t c2, thstrict_t s);

extern size_t th_prev_cell(const thchar_t *s, size_t pos, struct thcell_t *cell, int is_decomp_am);

extern int th_validate(struct thcell_t context, thchar_t c, struct thinpconv_t *conv);

extern char *strcpy(char*, const char*);

extern int strcmp(const char*, const char*);

extern unsigned long strlen(const char*);

extern int fprintf(FILE*, const char*, ...);

extern int th_render_text_tis(const thchar_t *s, thglyph_t *res, size_t res_sz, int is_decomp_am);

extern int th_render_text_win(const thchar_t *s, thglyph_t *res, size_t res_sz, int is_decomp_am);

extern int th_render_text_mac(const thchar_t *s, thglyph_t *res, size_t res_sz, int is_decomp_am);

extern int strcmp(const char*, const char*);

extern int fprintf(FILE*, const char*, ...);

int test_th_render_tis(void);

int test_th_render_win(void);

int test_th_render_mac(void);

extern size_t th_normalize(thchar_t *dest, const thchar_t *src, size_t n);

extern int strcmp(const char*, const char*);

int test_th_normalize(void);

extern int printf(const char*, ...);

extern void exit(int);

extern char *strcpy(char*, const char*);

extern int strcmp(const char*, const char*);

extern unsigned long strlen(const char*);

extern struct ThBrk *th_brk_new(const char *dictpath);

extern void th_brk_delete(struct ThBrk *brk);

extern int th_brk_insert_breaks(struct ThBrk *brk,
                                const thchar_t *in_0,
                                thchar_t *out,
                                size_t out_sz,
                                const char *delim);

extern unsigned long wcslen(const int*);

extern int th_uni2tis_line(const thwchar_t *s, thchar_t *result, size_t n);

extern int th_tis2uni_line(const thchar_t *s, thwchar_t *result, size_t n);

extern int th_brk_wc_find_breaks(struct ThBrk *brk, const thwchar_t *s, int *pos, size_t pos_sz);

extern int th_brk_wc_insert_breaks(struct ThBrk *brk,
                                   const thwchar_t *in_0,
                                   thwchar_t *out,
                                   size_t out_sz,
                                   const thwchar_t *delim);

extern int fprintf(FILE*, const char*, ...);

extern char *strcpy(char*, const char*);

extern int strcmp(const char*, const char*);

extern unsigned long strlen(const char*);

extern unsigned long wcslen(const int*);

extern thchar_t th_uni2macthai(thwchar_t wc);

extern thchar_t th_uni2winthai(thwchar_t wc);

extern int th_uni2tis_line(const thwchar_t *s, thchar_t *result, size_t n);

extern thwchar_t th_macthai2uni(thchar_t c);

extern thwchar_t th_winthai2uni(thchar_t c);

extern int th_tis2uni_line(const thchar_t *s, thwchar_t *result, size_t n);

extern int th_strcoll(const thchar_t *s1, const thchar_t *s2);

extern int fclose(FILE *__stream);

extern FILE *fopen(const char*, const char*);

extern int fprintf(FILE*, const char*, ...);

extern int printf(const char*, ...);

extern char *fgets(char *__s, int __n, FILE *__stream);

extern void perror(const char *__s);

extern char *strcpy(char*, const char*);

extern unsigned long strlen(const char*);

extern void *malloc(unsigned long);

extern void free(void*);

extern void qsort(void *__base, size_t __nmemb, size_t __size, __compar_fn_t __compar);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THAI_THAILIB_H */
