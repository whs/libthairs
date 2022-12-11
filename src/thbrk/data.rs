#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BreakClass {
    /// Thai Character
    Thai,
    /// Non-Thai Alphabet
    Alpha,
    /// Number
    Num,
    /// No Break Before
    Nbb,
    /// No Break After
    Nba,
    /// No Break
    Nb,
    /// Mandatory Break
    Mb,
    /// White Space
    Space,
    /// Ambiguous Quotation
    Quote,
    /// Minus-Hyphen
    Hyphen,
    /// Close Parenthesis/Bracket/Brace
    Close,
    /// No Break Before in Numerical Mode
    NumNbb,
    /// Currency, No Break Before/After in Numerical Mode
    NumCur,
    /// No Break in Numerical Mode, NBB Otherwise
    NumNb,
    /// Terminator (Non-starter)
    Term,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BreakOperation {
    /// no break, even with space in between
    Prohibited,
    /// direct break, break immediately
    Allowed,
    /// indirect break, break if with space in between
    Indirect,
}

const CHAR_CLASS: [BreakClass; 256] = [
    /*0x00*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x01*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x02*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x03*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x04*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x05*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x06*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x07*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x08*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x09*/ BreakClass::Space, /* Unicode: BA */
    /* <control> */
    /*0x0A*/ BreakClass::Mb, /* Unicode: LF */
    /* <control> */
    /*0x0B*/ BreakClass::Mb, /* Unicode: BK */
    /* <control> */
    /*0x0C*/ BreakClass::Mb, /* Unicode: BK */
    /* <control> */
    /*0x0D*/ BreakClass::Mb, /* Unicode: CR */
    /* <control> */
    /*0x0E*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x0F*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x10*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x11*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x12*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x13*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x14*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x15*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x16*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x17*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x18*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x19*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x1A*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x1B*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x1C*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x1D*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x1E*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x1F*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x20*/ BreakClass::Space, /* Unicode: SP */
    /* SPACE */
    /*0x21*/ BreakClass::Nbb, /* Unicode: EX */
    /* EXCLAMATION MARK */
    /*0x22*/ BreakClass::Quote, /* Unicode: QU */
    /* QUOTATION MARK */
    /*0x23*/ BreakClass::Quote, /* Unicode: AL */
    /* NUMBER SIGN */
    /*0x24*/ BreakClass::NumCur, /* Unicode: PR */
    /* DOLLAR SIGN */
    /*0x25*/ BreakClass::NumNbb, /* Unicode: PO */
    /* PERCENT SIGN */
    /*0x26*/ BreakClass::Alpha, /* Unicode: AL */
    /* AMPERSAND */
    /*0x27*/ BreakClass::Quote, /* Unicode: QU */
    /* APOSTROPHE */
    /*0x28*/ BreakClass::Nba, /* Unicode: OP */
    /* LEFT PARENTHESIS */
    /*0x29*/ BreakClass::Close, /* Unicode: CL */
    /* RIGHT PARENTHESIS */
    /*0x2A*/ BreakClass::Alpha, /* Unicode: AL */
    /* ASTERISK */
    /*0x2B*/ BreakClass::NumCur, /* Unicode: PR */
    /* PLUS SIGN */
    /*0x2C*/ BreakClass::NumNb, /* Unicode: IS */
    /* COMMA */
    /*0x2D*/ BreakClass::Hyphen, /* Unicode: HY */
    /* HYPHEN-MINUS */
    /*0x2E*/ BreakClass::NumNb, /* Unicode: IS */
    /* FULL STOP */
    /*0x2F*/ BreakClass::Nbb, /* Unicode: SY */
    /* SOLIDUS */
    /*0x30*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT ZERO */
    /*0x31*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT ONE */
    /*0x32*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT TWO */
    /*0x33*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT THREE */
    /*0x34*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT FOUR */
    /*0x35*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT FIVE */
    /*0x36*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT SIX */
    /*0x37*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT SEVEN */
    /*0x38*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT EIGHT */
    /*0x39*/ BreakClass::Num, /* Unicode: NU */
    /* DIGIT NINE */
    /*0x3A*/ BreakClass::NumNb, /* Unicode: IS */
    /* COLON */
    /*0x3B*/ BreakClass::NumNb, /* Unicode: IS */
    /* SEMICOLON */
    /*0x3C*/ BreakClass::Alpha, /* Unicode: AL */
    /* LESS-THAN SIGN */
    /*0x3D*/ BreakClass::Alpha, /* Unicode: AL */
    /* EQUALS SIGN */
    /*0x3E*/ BreakClass::Alpha, /* Unicode: AL */
    /* GREATER-THAN SIGN */
    /*0x3F*/ BreakClass::Nbb, /* Unicode: EX */
    /* QUESTION MARK */
    /*0x40*/ BreakClass::Alpha, /* Unicode: AL */
    /* COMMERCIAL AT */
    /*0x41*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER A */
    /*0x42*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER B */
    /*0x43*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER C */
    /*0x44*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER D */
    /*0x45*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER E */
    /*0x46*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER F */
    /*0x47*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER G */
    /*0x48*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER H */
    /*0x49*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER I */
    /*0x4A*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER J */
    /*0x4B*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER K */
    /*0x4C*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER L */
    /*0x4D*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER M */
    /*0x4E*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER N */
    /*0x4F*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER O */
    /*0x50*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER P */
    /*0x51*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER Q */
    /*0x52*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER R */
    /*0x53*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER S */
    /*0x54*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER T */
    /*0x55*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER U */
    /*0x56*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER V */
    /*0x57*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER W */
    /*0x58*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER X */
    /*0x59*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER Y */
    /*0x5A*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN CAPITAL LETTER Z */
    /*0x5B*/ BreakClass::Nba, /* Unicode: OP */
    /* LEFT SQUARE BRACKET */
    /*0x5C*/ BreakClass::NumCur, /* Unicode: PR */
    /* REVERSE SOLIDUS */
    /*0x5D*/ BreakClass::Close, /* Unicode: CL */
    /* RIGHT SQUARE BRACKET */
    /*0x5E*/ BreakClass::Alpha, /* Unicode: AL */
    /* CIRCUMFLEX ACCENT */
    /*0x5F*/ BreakClass::Alpha, /* Unicode: AL */
    /* LOW LINE */
    /*0x60*/ BreakClass::Alpha, /* Unicode: AL */
    /* GRAVE ACCENT */
    /*0x61*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER A */
    /*0x62*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER B */
    /*0x63*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER C */
    /*0x64*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER D */
    /*0x65*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER E */
    /*0x66*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER F */
    /*0x67*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER G */
    /*0x68*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER H */
    /*0x69*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER I */
    /*0x6A*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER J */
    /*0x6B*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER K */
    /*0x6C*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER L */
    /*0x6D*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER M */
    /*0x6E*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER N */
    /*0x6F*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER O */
    /*0x70*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER P */
    /*0x71*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER Q */
    /*0x72*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER R */
    /*0x73*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER S */
    /*0x74*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER T */
    /*0x75*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER U */
    /*0x76*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER V */
    /*0x77*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER W */
    /*0x78*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER X */
    /*0x79*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER Y */
    /*0x7A*/ BreakClass::Alpha, /* Unicode: AL */
    /* LATIN SMALL LETTER Z */
    /*0x7B*/ BreakClass::Nba, /* Unicode: OP */
    /* LEFT CURLY BRACKET */
    /*0x7C*/ BreakClass::Space, /* Unicode: BA */
    /* VERTICAL LINE */
    /*0x7D*/ BreakClass::Close, /* Unicode: CL */
    /* RIGHT CURLY BRACKET */
    /*0x7E*/ BreakClass::Alpha, /* Unicode: AL */
    /* TILDE */
    /*0x7F*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x80*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x81*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x82*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x83*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x84*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x85*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x86*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x87*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x88*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x89*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x8A*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x8B*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x8C*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x8D*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x8E*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x8F*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x90*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x91*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x92*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x93*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x94*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x95*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x96*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x97*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x98*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x99*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x9A*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x9B*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x9C*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x9D*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x9E*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0x9F*/ BreakClass::Nbb, /* Unicode: CM */
    /* <control> */
    /*0xA0*/ BreakClass::Nb, /* Unicode: GL */
    /* NO-BREAK SPACE */
    /*0xA1*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KO KAI */
    /*0xA2*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO KHAI */
    /*0xA3*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO KHUAT */
    /*0xA4*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO KHWAI */
    /*0xA5*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO KHON */
    /*0xA6*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER KHO RAKHANG */
    /*0xA7*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER NGO NGU */
    /*0xA8*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER CHO CHAN */
    /*0xA9*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER CHO CHING */
    /*0xAA*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER CHO CHANG */
    /*0xAB*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SO SO */
    /*0xAC*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER CHO CHOE */
    /*0xAD*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER YO YING */
    /*0xAE*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER DO CHADA */
    /*0xAF*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER TO PATAK */
    /*0xB0*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO THAN */
    /*0xB1*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO NANGMONTHO */
    /*0xB2*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO PHUTHAO */
    /*0xB3*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER NO NEN */
    /*0xB4*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER DO DEK */
    /*0xB5*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER TO TAO */
    /*0xB6*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO THUNG */
    /*0xB7*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO THAHAN */
    /*0xB8*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THO THONG */
    /*0xB9*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER NO NU */
    /*0xBA*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER BO BAIMAI */
    /*0xBB*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PO PLA */
    /*0xBC*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PHO PHUNG */
    /*0xBD*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER FO FA */
    /*0xBE*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PHO PHAN */
    /*0xBF*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER FO FAN */
    /*0xC0*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PHO SAMPHAO */
    /*0xC1*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MO MA */
    /*0xC2*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER YO YAK */
    /*0xC3*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER RO RUA */
    /*0xC4*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER RU */
    /*0xC5*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER LO LING */
    /*0xC6*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER LU */
    /*0xC7*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER WO WAEN */
    /*0xC8*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SO SALA */
    /*0xC9*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SO RUSI */
    /*0xCA*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SO SUA */
    /*0xCB*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER HO HIP */
    /*0xCC*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER LO CHULA */
    /*0xCD*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER O ANG */
    /*0xCE*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER HO NOKHUK */
    /*0xCF*/ BreakClass::Nbb, /* Unicode: SA */
    /* THAI CHARACTER PAIYANNOI */
    /*0xD0*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA A */
    /*0xD1*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI HAN-AKAT */
    /*0xD2*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AA */
    /*0xD3*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AM */
    /*0xD4*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA I */
    /*0xD5*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA II */
    /*0xD6*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA UE */
    /*0xD7*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA UEE */
    /*0xD8*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA U */
    /*0xD9*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA UU */
    /*0xDA*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER PHINTHU */
    /*0xDB*/ BreakClass::Nbb, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xDC*/ BreakClass::Nbb, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xDD*/ BreakClass::Nbb, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xDE*/ BreakClass::Nbb, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xDF*/ BreakClass::NumCur, /* Unicode: PR */
    /* THAI CURRENCY SYMBOL BAHT */
    /*0xE0*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA E */
    /*0xE1*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AE */
    /*0xE2*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA O */
    /*0xE3*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AI MAIMUAN */
    /*0xE4*/
    BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER SARA AI MAIMALAI */
    /*0xE5*/
    BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER LAKKHANGYAO */
    /*0xE6*/ BreakClass::Nbb, /* Unicode: SA */
    /* THAI CHARACTER MAIYAMOK */
    /*0xE7*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAITAIKHU */
    /*0xE8*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI EK */
    /*0xE9*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI THO */
    /*0xEA*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI TRI */
    /*0xEB*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER MAI CHATTAWA */
    /*0xEC*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER THANTHAKHAT */
    /*0xED*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER NIKHAHIT */
    /*0xEE*/ BreakClass::Thai, /* Unicode: SA */
    /* THAI CHARACTER YAMAKKAN */
    /*0xEF*/ BreakClass::Alpha, /* Unicode: AL */
    /* THAI CHARACTER FONGMAN */
    /*0xF0*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT ZERO */
    /*0xF1*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT ONE */
    /*0xF2*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT TWO */
    /*0xF3*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT THREE */
    /*0xF4*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT FOUR */
    /*0xF5*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT FIVE */
    /*0xF6*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT SIX */
    /*0xF7*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT SEVEN */
    /*0xF8*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT EIGHT */
    /*0xF9*/ BreakClass::Num, /* Unicode: NU */
    /* THAI DIGIT NINE */
    /*0xFA*/ BreakClass::Term, /* Unicode: BA */
    /* THAI CHARACTER ANGKHANKHU */
    /*0xFB*/ BreakClass::Term, /* Unicode: BA */
    /* THAI CHARACTER KHOMUT */
    /*0xFC*/ BreakClass::Nbb, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xFD*/ BreakClass::Nbb, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xFE*/ BreakClass::Nbb, /* Unicode: -  */
    /* -- Unencoded -- */
    /*0xFF*/
    BreakClass::Nbb, /* Unicode: -  */  /* -- Unencoded -- */
];

#[inline]
pub fn brk_class(c: u8) -> BreakClass {
    CHAR_CLASS[c as usize]
}

const _P: BreakOperation = BreakOperation::Prohibited;
const _A: BreakOperation = BreakOperation::Allowed;
const _I: BreakOperation = BreakOperation::Indirect;

const BREAK_TABLE: [[BreakOperation; 15]; 15] = [
    /*           THA ALP NUM NBB NBA NB  MB  SPA QUO HYP CLO NUM NUM NUM TER */
    /*           I   HA                      CE  TE  HEN SE  NBB CUR NB  M   */
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

#[inline]
pub fn brk_op(prev: BreakClass, next: BreakClass) -> BreakOperation {
    BREAK_TABLE[prev as usize][next as usize]
}
