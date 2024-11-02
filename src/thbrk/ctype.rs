use crate::thctype::thchar_t;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum BrkClass {
    /// Thai character
    Thai,
    /// Non-Thai Alphabet
    Alpha,
    /// Number
    Num,
    /// No Break Before
    NBB,
    /// No Break After
    NBA,
    /// No Break
    NB,
    /// Mandatory Break
    MB,
    /// White Space
    Space,
    /// Ambiguous Quotation
    Quote,
    /// Minus-Hyphen
    Hyphen,
    /// Close Parenthesis/Bracket/Brace
    Close,
    /// No Break Before in Numerical Mode
    NumNBB,
    /// Currency, No Break Before/After in Numerical Mode
    NumCUR,
    /// No Break in Numerical Mode, NBB Otherwise
    NumNB,
    /// Terminator (Non-starter)
    Term,
}

const char_class: [BrkClass; 256] = [
    /*0x00*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x01*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x02*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x03*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x04*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x05*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x06*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x07*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x08*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x09*/ BrkClass::Space, /* Unicode: BA */
    /* <control> */
    /*0x0A*/ BrkClass::MB, /* Unicode: LF */
    /* <control> */
    /*0x0B*/ BrkClass::MB, /* Unicode: BK */
    /* <control> */
    /*0x0C*/ BrkClass::MB, /* Unicode: BK */
    /* <control> */
    /*0x0D*/ BrkClass::MB, /* Unicode: CR */
    /* <control> */
    /*0x0E*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x0F*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x10*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x11*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x12*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x13*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x14*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x15*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x16*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x17*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x18*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x19*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x1A*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x1B*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x1C*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x1D*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x1E*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x1F*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x20*/ BrkClass::Space, /* Unicode: SP */
    /* SPACE */
    /*0x21*/ BrkClass::NBB, /* Unicode: EX */
    /* EXCLAMATION MARK */
    /*0x22*/ BrkClass::Quote, /* Unicode: QU */
    /* QUOTATION MARK */
    /*0x23*/ BrkClass::Quote, /* Unicode: AL */
    /* NUMBER SIGN */
    /*0x24*/ BrkClass::NumCUR, /* Unicode: PR */
    /* DOLLAR SIGN */
    /*0x25*/ BrkClass::NumNBB, /* Unicode: PO */
    /* PERCENT SIGN */
    /*0x26*/ BrkClass::Alpha, /* Unicode: AL */
    /* AMPERSAND */
    /*0x27*/ BrkClass::Quote, /* Unicode: QU */
    /* APOSTROPHE */
    /*0x28*/ BrkClass::NBA, /* Unicode: OP */
    /* LEFT PARENTHESIS */
    /*0x29*/ BrkClass::Close, /* Unicode: CL */
    /* RIGHT PARENTHESIS */
    /*0x2A*/ BrkClass::Alpha, /* Unicode: AL */
    /* ASTERISK */
    /*0x2B*/ BrkClass::NumCUR, /* Unicode: PR */
    /* PLUS SIGN */
    /*0x2C*/ BrkClass::NumNB, /* Unicode: IS */
    /* COMMA */
    /*0x2D*/ BrkClass::Hyphen, /* Unicode: HY */
    /* HYPHEN-MINUS */
    /*0x2E*/ BrkClass::NumNB, /* Unicode: IS */
    /* FULL STOP */
    /*0x2F*/ BrkClass::NBB, /* Unicode: SY */
    /* SOLIDUS */
    /*0x30*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT ZERO */
    /*0x31*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT ONE */
    /*0x32*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT TWO */
    /*0x33*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT THREE */
    /*0x34*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT FOUR */
    /*0x35*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT FIVE */
    /*0x36*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT SIX */
    /*0x37*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT SEVEN */
    /*0x38*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT EIGHT */
    /*0x39*/ BrkClass::Num, /* Unicode: NU */
    /* DIGIT NINE */
    /*0x3A*/ BrkClass::NumNB, /* Unicode: IS */
    /* COLON */
    /*0x3B*/ BrkClass::NumNB, /* Unicode: IS */
    /* SEMICOLON */
    /*0x3C*/ BrkClass::Alpha, /* Unicode: AL */
    /* LESS-THAN SIGN */
    /*0x3D*/ BrkClass::Alpha, /* Unicode: AL */
    /* EQUALS SIGN */
    /*0x3E*/ BrkClass::Alpha, /* Unicode: AL */
    /* GREATER-THAN SIGN */
    /*0x3F*/ BrkClass::NBB, /* Unicode: EX */
    /* QUESTION MARK */
    /*0x40*/ BrkClass::Alpha, /* Unicode: AL */
    /* COMMERCIAL AT */
    /*0x41*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER A */
    /*0x42*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER B */
    /*0x43*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER C */
    /*0x44*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER D */
    /*0x45*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER E */
    /*0x46*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER F */
    /*0x47*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER G */
    /*0x48*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER H */
    /*0x49*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER I */
    /*0x4A*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER J */
    /*0x4B*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER K */
    /*0x4C*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER L */
    /*0x4D*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER M */
    /*0x4E*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER N */
    /*0x4F*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER O */
    /*0x50*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER P */
    /*0x51*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER Q */
    /*0x52*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER R */
    /*0x53*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER S */
    /*0x54*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER T */
    /*0x55*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER U */
    /*0x56*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER V */
    /*0x57*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER W */
    /*0x58*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER X */
    /*0x59*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER Y */
    /*0x5A*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER Z */
    /*0x5B*/ BrkClass::NBA, /* Unicode: OP */
    /* LEFT SQUARE BRACKET */
    /*0x5C*/ BrkClass::NumCUR, /* Unicode: PR */
    /* REVERSE SOLIDUS */
    /*0x5D*/ BrkClass::Close, /* Unicode: CL */
    /* RIGHT SQUARE BRACKET */
    /*0x5E*/ BrkClass::Alpha, /* Unicode: AL */
    /* CIRCUMFLEX ACCENT */
    /*0x5F*/ BrkClass::Alpha, /* Unicode: AL */
    /* LOW LINE */
    /*0x60*/ BrkClass::Alpha, /* Unicode: AL */
    /* GRAVE ACCENT */
    /*0x61*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER A */
    /*0x62*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER B */
    /*0x63*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER C */
    /*0x64*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER D */
    /*0x65*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER E */
    /*0x66*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER F */
    /*0x67*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER G */
    /*0x68*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER H */
    /*0x69*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER I */
    /*0x6A*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER J */
    /*0x6B*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER K */
    /*0x6C*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER L */
    /*0x6D*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER M */
    /*0x6E*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER N */
    /*0x6F*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER O */
    /*0x70*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER P */
    /*0x71*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER Q */
    /*0x72*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER R */
    /*0x73*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER S */
    /*0x74*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER T */
    /*0x75*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER U */
    /*0x76*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER V */
    /*0x77*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER W */
    /*0x78*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER X */
    /*0x79*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER Y */
    /*0x7A*/ BrkClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER Z */
    /*0x7B*/ BrkClass::NBA, /* Unicode: OP */
    /* LEFT CURLY BRACKET */
    /*0x7C*/ BrkClass::Space, /* Unicode: BA */
    /* VERTICAL LINE */
    /*0x7D*/ BrkClass::Close, /* Unicode: CL */
    /* RIGHT CURLY BRACKET */
    /*0x7E*/ BrkClass::Alpha, /* Unicode: AL */
    /* TILDE */
    /*0x7F*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x80*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x81*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x82*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x83*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x84*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x85*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x86*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x87*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x88*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x89*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x8A*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x8B*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x8C*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x8D*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x8E*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x8F*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x90*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x91*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x92*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x93*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x94*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x95*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x96*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x97*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x98*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x99*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x9A*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x9B*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x9C*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x9D*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x9E*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0x9F*/ BrkClass::NBB, /* Unicode: CM */
    /* <control> */
    /*0xA0*/ BrkClass::NB, /* Unicode: GL */
    /* NO-BREAK SPACE */
    /*0xA1*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KO KAI */
    /*0xA2*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO KHAI */
    /*0xA3*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO KHUAT */
    /*0xA4*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO KHWAI */
    /*0xA5*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO KHON */
    /*0xA6*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO RAKHANG */
    /*0xA7*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER NGO NGU */
    /*0xA8*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER CHO CHAN */
    /*0xA9*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER CHO CHING */
    /*0xAA*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER CHO CHANG */
    /*0xAB*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SO SO */
    /*0xAC*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER CHO CHOE */
    /*0xAD*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER YO YING */
    /*0xAE*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER DO CHADA */
    /*0xAF*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER TO PATAK */
    /*0xB0*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO THAN */
    /*0xB1*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO NANGMONTHO */
    /*0xB2*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO PHUTHAO */
    /*0xB3*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER NO NEN */
    /*0xB4*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER DO DEK */
    /*0xB5*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER TO TAO */
    /*0xB6*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO THUNG */
    /*0xB7*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO THAHAN */
    /*0xB8*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO THONG */
    /*0xB9*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER NO NU */
    /*0xBA*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER BO BAIMAI */
    /*0xBB*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PO PLA */
    /*0xBC*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PHO PHUNG */
    /*0xBD*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER FO FA */
    /*0xBE*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PHO PHAN */
    /*0xBF*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER FO FAN */
    /*0xC0*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PHO SAMPHAO */
    /*0xC1*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MO MA */
    /*0xC2*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER YO YAK */
    /*0xC3*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER RO RUA */
    /*0xC4*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER RU */
    /*0xC5*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER LO LING */
    /*0xC6*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER LU */
    /*0xC7*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER WO WAEN */
    /*0xC8*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SO SALA */
    /*0xC9*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SO RUSI */
    /*0xCA*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SO SUA */
    /*0xCB*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER HO HIP */
    /*0xCC*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER LO CHULA */
    /*0xCD*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER O ANG */
    /*0xCE*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER HO NOKHUK */
    /*0xCF*/ BrkClass::NBB, /* Unicode: SA */
    /* THAI CHARACTER PAIYANNOI */
    /*0xD0*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA A */
    /*0xD1*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI HAN-AKAT */
    /*0xD2*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AA */
    /*0xD3*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AM */
    /*0xD4*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA I */
    /*0xD5*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA II */
    /*0xD6*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA UE */
    /*0xD7*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA UEE */
    /*0xD8*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA U */
    /*0xD9*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA UU */
    /*0xDA*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PHINTHU */
    /*0xDB*/ BrkClass::NBB, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xDC*/ BrkClass::NBB, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xDD*/ BrkClass::NBB, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xDE*/ BrkClass::NBB, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xDF*/ BrkClass::NumCUR, /* Unicode: PR */
    /* THAI CURRENCY SYMBOL BAHT */
    /*0xE0*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA E */
    /*0xE1*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AE */
    /*0xE2*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA O */
    /*0xE3*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AI MAIMUAN */
    /*0xE4*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AI MAIMALAI */
    /*0xE5*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER LAKKHANGYAO */
    /*0xE6*/ BrkClass::NBB, /* Unicode: SA */
    /* THAI CHARACTER MAIYAMOK */
    /*0xE7*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAITAIKHU */
    /*0xE8*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI EK */
    /*0xE9*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI THO */
    /*0xEA*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI TRI */
    /*0xEB*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI CHATTAWA */
    /*0xEC*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THANTHAKHAT */
    /*0xED*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER NIKHAHIT */
    /*0xEE*/ BrkClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER YAMAKKAN */
    /*0xEF*/ BrkClass::Alpha, /* Unicode: AL */
    /* THAI CHARACTER FONGMAN */
    /*0xF0*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT ZERO */
    /*0xF1*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT ONE */
    /*0xF2*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT TWO */
    /*0xF3*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT THREE */
    /*0xF4*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT FOUR */
    /*0xF5*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT FIVE */
    /*0xF6*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT SIX */
    /*0xF7*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT SEVEN */
    /*0xF8*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT EIGHT */
    /*0xF9*/ BrkClass::Num, /* Unicode: NU */
    /* THAI DIGIT NINE */
    /*0xFA*/ BrkClass::Term, /* Unicode: BA */
    /* THAI CHARACTER ANGKHANKHU */
    /*0xFB*/ BrkClass::Term, /* Unicode: BA */
    /* THAI CHARACTER KHOMUT */
    /*0xFC*/ BrkClass::NBB, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xFD*/ BrkClass::NBB, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xFE*/ BrkClass::NBB, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xFF*/
    BrkClass::NBB, /* Unicode: -  */  /* -- Unencoded -- */
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum BrkOp {
    /// no break, even with space in between
    Prohibited = 0,
    Allowed = 1,
    Indirect = 2,
}

const _P: BrkOp = BrkOp::Indirect;
const _A: BrkOp = BrkOp::Allowed;
const _I: BrkOp = BrkOp::Prohibited;

const break_table: [[BrkOp; 15]; 15] = [
    /* THA ALP NUM NBB NBA NB  MB  SPA QUO HYP CLO NUM NUM NUM TER */
    /* I   HA                      CE  TE  HEN SE  NBB CUR NB  M   */
    /*THAI*/
    [_I, _A, _I, _P, _A, _I, _P, _P, _I, _I, _P, _I, _A, _P, _I],
    /*ALPHA*/ [_A, _I, _I, _P, _I, _I, _P, _P, _I, _I, _P, _I, _I, _P, _I],
    /*NUM*/ [_A, _I, _I, _P, _A, _I, _P, _P, _I, _I, _P, _P, _P, _P, _I],
    /*NBB*/ [_A, _A, _A, _P, _A, _I, _P, _P, _I, _I, _P, _P, _A, _P, _I],
    /*NBA*/ [_P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P],
    /*NB*/ [_I, _I, _I, _P, _I, _I, _P, _P, _I, _I, _P, _I, _I, _P, _I],
    /*MB*/ [_A, _A, _A, _A, _A, _A, _A, _A, _A, _A, _A, _A, _A, _A, _A],
    /*SPACE*/ [_A, _A, _A, _P, _A, _A, _P, _P, _A, _A, _P, _A, _A, _P, _A],
    /*QUOTE*/ [_I, _I, _I, _P, _I, _I, _P, _P, _I, _I, _P, _I, _I, _P, _I],
    /*HYPHEN*/ [_A, _A, _I, _P, _A, _I, _P, _P, _I, _I, _P, _A, _A, _P, _I],
    /*CLOSE*/ [_A, _I, _A, _P, _A, _I, _P, _P, _I, _I, _P, _P, _P, _P, _I],
    /*NUM_NBB*/ [_A, _A, _A, _P, _A, _I, _P, _P, _I, _A, _P, _A, _A, _P, _I],
    /*NUM_CUR*/ [_A, _I, _P, _P, _P, _I, _P, _P, _I, _P, _P, _A, _I, _P, _I],
    /*NUM_NB*/ [_A, _A, _I, _P, _A, _I, _P, _P, _I, _I, _P, _A, _A, _P, _I],
    /*TERM*/ [_A, _A, _A, _P, _A, _I, _P, _P, _I, _I, _P, _A, _A, _P, _I],
];

pub(super) const fn brk_class(c: thchar_t) -> BrkClass {
    char_class[c as usize]
}

pub(super) const fn brk_op(prev: BrkClass, next: BrkClass) -> BrkOp {
    break_table[prev as usize][next as usize]
}
