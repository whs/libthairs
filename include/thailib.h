#ifndef THAI_THAILIB_H
#define THAI_THAILIB_H

/* Generated with cbindgen:0.27.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "stddef.h"

typedef unsigned int WTTClass;

typedef uint8_t thchar_t;

typedef unsigned int WTTOp;

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

#define SR 5

#define RJ 4

#define AC 3

#define XC 2

#define CP 1

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern short TACchtype_[256];

extern short TACio_op_[17][17];

WTTClass TACchtype(thchar_t c);

WTTOp TACio_op(thchar_t c1, thchar_t c2);

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

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THAI_THAILIB_H */
