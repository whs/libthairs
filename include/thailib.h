#ifndef THAI_THAILIB_H
#define THAI_THAILIB_H

/* Generated with cbindgen:0.27.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "stddef.h"

#define TIS_KO_KAI 161

#define TIS_KHO_KHAI 162

#define TIS_KHO_KHUAT 163

#define TIS_KHO_KHWAI 164

#define TIS_KHO_KHON 165

#define TIS_KHO_RAKHANG 166

#define TIS_NGO_NGU 167

#define TIS_CHO_CHAN 168

#define TIS_CHO_CHING 169

#define TIS_CHO_CHANG 170

#define TIS_SO_SO 171

#define TIS_CHO_CHOE 172

#define TIS_YO_YING 173

#define TIS_DO_CHADA 174

#define TIS_TO_PATAK 175

#define TIS_THO_THAN 176

#define TIS_THO_NANGMONTHO 177

#define TIS_THO_PHUTHAO 178

#define TIS_NO_NEN 179

#define TIS_DO_DEK 180

#define TIS_TO_TAO 181

#define TIS_THO_THUNG 182

#define TIS_THO_THAHAN 183

#define TIS_THO_THONG 184

#define TIS_NO_NU 185

#define TIS_BO_BAIMAI 186

#define TIS_PO_PLA 187

#define TIS_PHO_PHUNG 188

#define TIS_FO_FA 189

#define TIS_PHO_PHAN 190

#define TIS_FO_FAN 191

#define TIS_PHO_SAMPHAO 192

#define TIS_MO_MA 193

#define TIS_YO_YAK 194

#define TIS_RO_RUA 195

#define TIS_RU 196

#define TIS_LO_LING 197

#define TIS_LU 198

#define TIS_WO_WAEN 199

#define TIS_SO_SALA 200

#define TIS_SO_RUSI 201

#define TIS_SO_SUA 202

#define TIS_HO_HIP 203

#define TIS_LO_CHULA 204

#define TIS_O_ANG 205

#define TIS_HO_NOKHUK 206

#define TIS_PAIYANNOI 207

#define TIS_SARA_A 208

#define TIS_MAI_HAN_AKAT 209

#define TIS_SARA_AA 210

#define TIS_SARA_AM 211

#define TIS_SARA_I 212

#define TIS_SARA_II 213

#define TIS_SARA_UE 214

#define TIS_SARA_UEE 215

#define TIS_SARA_U 216

#define TIS_SARA_UU 217

#define TIS_PHINTHU 218

#define TIS_SYMBOL_BAHT 223

#define TIS_SARA_E 224

#define TIS_SARA_AE 225

#define TIS_SARA_O 226

#define TIS_SARA_AI_MAIMUAN 227

#define TIS_SARA_AI_MAIMALAI 228

#define TIS_LAKKHANGYAO 229

#define TIS_MAIYAMOK 230

#define TIS_MAITAIKHU 231

#define TIS_MAI_EK 232

#define TIS_MAI_THO 233

#define TIS_MAI_TRI 234

#define TIS_MAI_CHATTAWA 235

#define TIS_THANTHAKHAT 236

#define TIS_NIKHAHIT 237

#define TIS_YAMAKKAN 238

#define TIS_FONGMAN 239

#define TIS_THAI_DIGIT_ZERO 240

#define TIS_THAI_DIGIT_ONE 241

#define TIS_THAI_DIGIT_TWO 242

#define TIS_THAI_DIGIT_THREE 243

#define TIS_THAI_DIGIT_FOUR 244

#define TIS_THAI_DIGIT_FIVE 245

#define TIS_THAI_DIGIT_SIX 246

#define TIS_THAI_DIGIT_SEVEN 247

#define TIS_THAI_DIGIT_EIGHT 248

#define TIS_THAI_DIGIT_NINE 249

#define TIS_ANGKHANKHU 250

#define TIS_KHOMUT 251

#define TIS_YMBOL_BAHT 223

#define TOT_LEVELS (int)4

#define IGNORE (int)0

#define TH_BLANK_BASE_GLYPH (int)221

#define SIZE_MAX (unsigned long)18446744073709551615ull

#define NULL (int)0

#define THCHAR_ERR ~(int)0

#define TH_ERR ~(int)0

typedef struct ThBrk ThBrk;

typedef unsigned char thchar_t;

typedef uint32_t Bool;

typedef TrieState_Option_CTrieData LegacyTrieState;

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

typedef int wchar_t;

typedef wchar_t thwchar_t;

typedef unsigned int C2RustUnnamed;

typedef unsigned int C2RustUnnamed_0;

typedef unsigned int l3_symbols;

typedef unsigned int l4_symbols;

typedef unsigned int l2_symbols;

typedef unsigned int l1_symbols;

#define _th_IScons 2

#define CP 1

#define _th_IStone 128

#define _th_ISpunct 1024

#define _th_ISdigit 512

#define _th_ISdiac 256

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

#define _th_IStis 1

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

extern short TACchtype_[256];

extern short TACio_op_[17][17];

extern char *strcpy(char*, const char*);

extern size_t strlen(const char*);

extern void *malloc(size_t);

extern void free(void*);

ThBrk *th_brk_new(const char *dictpath);

void th_brk_delete(ThBrk *brk);

int th_brk_insert_breaks(ThBrk *brk,
                         const thchar_t *in_0,
                         thchar_t *out,
                         size_t out_sz,
                         const char *delim);

int32_t th_brk_find_breaks(const ThBrk *brk, const thchar_t *s, int32_t *pos, uintptr_t pos_sz);

int th_brk_line(const thchar_t *in_0, thchar_t *out, size_t out_sz, const char *delim);

int32_t th_brk(const thchar_t *s, int32_t *pos, uintptr_t pos_sz);

extern void *memcpy(void*, const void*, unsigned long);

extern void *malloc(unsigned long);

extern void *realloc(void*, unsigned long);

extern void free(void*);

extern Bool trie_state_is_single(const LegacyTrieState *s);

extern Bool trie_state_is_walkable(const LegacyTrieState *s, AlphaChar c);

extern Bool trie_state_walk(LegacyTrieState *s, AlphaChar c);

extern void trie_state_copy(LegacyTrieState *dst, const LegacyTrieState *src);

extern LegacyTrieState *trie_state_clone(const LegacyTrieState *s);

extern void trie_state_free(LegacyTrieState *s);

extern void trie_state_rewind(LegacyTrieState *s);

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

extern int th_brk_find_breaks(ThBrk *brk, const thchar_t *s, int *pos, size_t pos_sz);

int th_brk_wc_find_breaks(ThBrk *brk, const thwchar_t *s, int *pos, size_t pos_sz);

int th_brk_wc_insert_breaks(ThBrk *brk,
                            const thwchar_t *in_0,
                            thwchar_t *out,
                            size_t out_sz,
                            const thwchar_t *delim);

int th_wbrk(const thwchar_t *s, int *pos, size_t pos_sz);

int th_wbrk_line(const thwchar_t *in_0, thwchar_t *out, size_t out_sz, const thwchar_t *delim);

thwchar_t th_tis2uni(thchar_t c);

/**
 * Convert string from TIS-620 to Unicode
 */
int32_t th_tis2uni_line(const thchar_t *s, thwchar_t *result, uintptr_t n);

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

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THAI_THAILIB_H */
