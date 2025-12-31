#ifndef THAI_THAILIB_H
#define THAI_THAILIB_H

/* Generated with cbindgen:0.29.2 */

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

typedef enum WTTClass {
  /**
   * control chars
   */
  CTRL = 0,
  /**
   * non composibles
   */
  NON = 1,
  /**
   * consonants
   */
  CONS = 2,
  /**
   * leading vowels
   */
  LV = 3,
  /**
   * following vowels 1
   */
  FV1 = 4,
  /**
   * following vowels 2
   */
  FV2 = 5,
  /**
   * following vowels 3
   */
  FV3 = 6,
  /**
   * below vowels 1
   */
  BV1 = 7,
  /**
   * below vowels 2
   */
  BV2 = 8,
  /**
   * below diacritics
   */
  BD = 9,
  /**
   * tonemarks
   */
  TONE = 10,
  /**
   * above diacritics 1
   */
  AD1 = 11,
  /**
   * above diacritics 2
   */
  AD2 = 12,
  /**
   * above diacritics 3
   */
  AD3 = 13,
  /**
   * above vowels 1
   */
  AV1 = 14,
  /**
   * above vowels 2
   */
  AV2 = 15,
  /**
   * above vowels 3
   */
  AV3 = 16,
} WTTClass;

typedef enum WTTOp {
  /**
   * COMPOSIBLE - following char is displayed in the same cell as leading char, also implies ACCEPT
   */
  CP = 1,
  /**
   * Non-display
   */
  XC = 2,
  /**
   * ACCEPT - display the following char in the next cell
   */
  AC = 3,
  /**
   * REJECT - discard that following char, ignore it
   */
  RJ = 4,
  /**
   * STRICT REJECT - REJECT only if in strict mode
   */
  SR = 5,
} WTTOp;

typedef struct ThBrk ThBrk;

/**
 * Thai character type for storing TIS-620 character
 */
typedef uint8_t thchar_t;

typedef uint32_t Bool;

typedef TrieState_Option_CTrieData LegacyTrieState;

/**
 * Thai character type for storing Unicode character
 */
typedef wchar_t thwchar_t;





#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern char *strcpy(char*, const char*);

extern size_t strlen(const char*);

extern void *malloc(size_t);

extern void free(void*);

struct ThBrk *th_brk_new(const char *dictpath);

void th_brk_delete(struct ThBrk *brk);

int th_brk_insert_breaks(struct ThBrk *brk,
                         const thchar_t *in_0,
                         thchar_t *out,
                         size_t out_sz,
                         const char *delim);

int32_t th_brk_find_breaks(const struct ThBrk *brk,
                           const thchar_t *s,
                           int32_t *pos,
                           uintptr_t pos_sz);

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

/**
 * Is the character a valid TIS-620 code?
 */
bool th_istis(thchar_t c);

/**
 * Is the character a Thai character?
 */
bool th_isthai(thchar_t c);

/**
 * Is the character an English character?
 */
bool th_iseng(thchar_t c);

/**
 * Is the character a Thai consonant?
 */
bool th_isthcons(thchar_t c);

/**
 * Is the character a Thai vowel?
 */
bool th_isthvowel(thchar_t c);

/**
 * Is the character a Thai tone mark?
 */
bool th_isthtone(thchar_t c);

/**
 * Is the character a Thai diacritic?
 */
bool th_isthdiac(thchar_t c);

/**
 * Is the character a Thai digit?
 */
bool th_isthdigit(thchar_t c);

/**
 * Is the character a Thai punctuation?
 */
bool th_isthpunct(thchar_t c);

/**
 * Is the character a Thai consonant that fits the x-height?
 */
bool th_istaillesscons(thchar_t c);

/**
 * Is the character a Thai consonant with stem above ascender?
 */
bool th_isovershootcons(thchar_t c);

/**
 * Is the character a Thai consonant with stem below baseline?
 */
bool th_isundershootcons(thchar_t c);

/**
 * Is the character a Thai consonant with split part below baseline?
 */
bool th_isundersplitcons(thchar_t c);

/**
 * Is the character a Thai leading vowel?
 */
bool th_isldvowel(thchar_t c);

/**
 * Is the character a Thai following vowel?
 */
bool th_isflvowel(thchar_t c);

/**
 * Is the character a Thai upper vowel?
 */
bool th_isupvowel(thchar_t c);

/**
 * Is the character a Thai below vowel?
 */
bool th_isblvowel(thchar_t c);

/**
 * Position for rendering
 */
int th_chlevel(thchar_t c);

/**
 * Is the character a combining character?
 */
bool th_iscombchar(thchar_t c);

/**
 * WTT character class
 */
enum WTTClass TACchtype(thchar_t c);

/**
 * WTT I/O operation
 */
enum WTTOp TACio_op(thchar_t c1, thchar_t c2);

/**
 * Normalize character order and remove excessive characters
 */
uintptr_t th_normalize(thchar_t *dest, const thchar_t *src, uintptr_t n);

/**
 * Convert character code from TIS-620 to Unicode.
 */
thwchar_t th_tis2uni(thchar_t c);

/**
 * Convert string from TIS-620 to Unicode
 */
int th_tis2uni_line(const thchar_t *source, thwchar_t *result, uintptr_t n);

/**
 * Convert character code from Thai Windows extended code to Unicode.
 */
thwchar_t th_winthai2uni(thchar_t c);

/**
 * Convert character code from Mac Thai extended code to Unicode
 */
thwchar_t th_macthai2uni(thchar_t c);

/**
 * Convert character code from Unicode to TIS-620
 */
thchar_t th_uni2tis(thwchar_t wc);

/**
 * Convert string from Unicode to TIS-620.
 */
int th_uni2tis_line(const thwchar_t *source, thchar_t *result, uintptr_t n);

/**
 * Convert character code from Unicode to Thai Windows extended code
 */
thchar_t th_uni2winthai(thwchar_t wc);

/**
 * Convert character code from Unicode to Mac Thai extended code
 */
thchar_t th_uni2macthai(thwchar_t wc);

/**
 * Is the wide character convertible to a valid TIS-620 code?
 */
bool th_wcistis(thwchar_t wc);

/**
 * Is the wide character a Thai character?
 */
bool th_wcisthai(thwchar_t wc);

/**
 * Is the wide character an English character?
 */
bool th_wciseng(thwchar_t wc);

/**
 * Is the wide character a Thai consonant?
 */
bool th_wcisthcons(thwchar_t wc);

/**
 * Is the wide character a Thai vowel?
 */
bool th_wcisthvowel(thwchar_t wc);

/**
 * Is the wide character a Thai tone mark?
 */
bool th_wcisthtone(thwchar_t wc);

/**
 * Is the wide character a Thai diacritic?
 */
bool th_wcisthdiac(thwchar_t wc);

/**
 * Is the character a Thai digit?
 */
bool th_wcisthdigit(thwchar_t wc);

/**
 * Is the character a Thai punctuation?
 */
bool th_wcisthpunct(thwchar_t wc);

/**
 * Is the wide character a Thai consonant that fits the x-height?
 */
bool th_wcistaillesscons(thwchar_t wc);

/**
 * Is the wide character a Thai consonant with stem above ascender?
 */
bool th_wcisovershootcons(thwchar_t wc);

/**
 * Is the wide character a Thai consonant with stem below baseline?
 */
bool th_wcisundershootcons(thwchar_t wc);

/**
 * Is the wide character a Thai consonant with split part below baseline?
 */
bool th_wcisundersplitcons(thwchar_t wc);

/**
 * Is the wide character a Thai leading vowel?
 */
bool th_wcisldvowel(thwchar_t wc);

/**
 * Is the wide character a Thai following vowel?
 */
bool th_wcisflvowel(thwchar_t wc);

/**
 * Is the wide character a Thai upper vowel?
 */
bool th_wcisupvowel(thwchar_t wc);

/**
 * Is the wide character a Thai below vowel?
 */
bool th_wcisblvowel(thwchar_t wc);

/**
 * Position for rendering
 */
int th_wcchlevel(thwchar_t wc);

/**
 * Normalize character order and remove excessive characters
 */
uintptr_t th_wnormalize(thwchar_t *wdest, const thwchar_t *wsrc, uintptr_t n);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THAI_THAILIB_H */
