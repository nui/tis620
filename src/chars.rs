use std::fmt::{self, Display, Write};

macro_rules! declare_thai_char {
    ([$(($value:literal, $char:literal, $variant:ident)),+ $(,)?]) => {
        // reference
        // https://en.wikipedia.org/wiki/Thai_(Unicode_block)
        // https://en.wikipedia.org/wiki/Thai_Industrial_Standard_620-2533
        /// An enum represent thai character.
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[allow(clippy::upper_case_acronyms)]
        #[allow(non_camel_case_types)]
        pub enum ThaiChar {
            $(
                $variant = $value
            ),+
        }

        impl ThaiChar {
            pub fn from_char(ch: char) -> Option<Self> {
                use ThaiChar::*;
                let variant = match ch {
                    $(
                        $char => $variant
                    ),+,
                    _ => return None,
                };
                Some(variant)
            }

            pub fn from_tis620_byte(byte: u8) -> Option<Self> {
                use ThaiChar::*;
                let variant = match byte {
                    $(
                        $value => $variant
                    ),+,
                    _ => return None,
                };
                Some(variant)
            }

            pub fn to_tis620_byte(self) -> u8 {
                self as u8
            }

            pub fn to_char(self) -> char {
                use ThaiChar::*;
                match self {
                    $(
                        $variant => $char
                    ),+,
                }
            }
        }
    };
}

declare_thai_char!([
    (0xA1, 'ก', KO_KAI),
    (0xA2, 'ข', KHO_KHAI),
    (0xA3, 'ฃ', KHO_KHUAT),
    (0xA4, 'ค', KHO_KHWAI),
    (0xA5, 'ฅ', KHO_KHON),
    (0xA6, 'ฆ', KHO_RAKHANG),
    (0xA7, 'ง', NGO_NGU),
    (0xA8, 'จ', CHO_CHAN),
    (0xA9, 'ฉ', CHO_CHING),
    (0xAA, 'ช', CHO_CHANG),
    (0xAB, 'ซ', SO_SO),
    (0xAC, 'ฌ', CHO_CHOE),
    (0xAD, 'ญ', YO_YING),
    (0xAE, 'ฎ', DO_CHADA),
    (0xAF, 'ฏ', TO_PATAK),
    (0xB0, 'ฐ', THO_THAN),
    (0xB1, 'ฑ', THO_NANGMONTHO),
    (0xB2, 'ฒ', THO_PHUTHAO),
    (0xB3, 'ณ', NO_NEN),
    (0xB4, 'ด', DO_DEK),
    (0xB5, 'ต', TO_TAO),
    (0xB6, 'ถ', THO_THUNG),
    (0xB7, 'ท', THO_THAHAN),
    (0xB8, 'ธ', THO_THONG),
    (0xB9, 'น', NO_NU),
    (0xBA, 'บ', BO_BAIMAI),
    (0xBB, 'ป', PO_PLA),
    (0xBC, 'ผ', PHO_PHUNG),
    (0xBD, 'ฝ', FO_FA),
    (0xBE, 'พ', PHO_PHAN),
    (0xBF, 'ฟ', FO_FAN),
    (0xC0, 'ภ', PHO_SAMPHAO),
    (0xC1, 'ม', MO_MA),
    (0xC2, 'ย', YO_YAK),
    (0xC3, 'ร', RO_RUA),
    (0xC4, 'ฤ', RU),
    (0xC5, 'ล', LO_LING),
    (0xC6, 'ฦ', LU),
    (0xC7, 'ว', WO_WAEN),
    (0xC8, 'ศ', SO_SALA),
    (0xC9, 'ษ', SO_RUSI),
    (0xCA, 'ส', SO_SUA),
    (0xCB, 'ห', HO_HIP),
    (0xCC, 'ฬ', LO_CHULA),
    (0xCD, 'อ', O_ANG),
    (0xCE, 'ฮ', HO_NOKHUK),
    (0xCF, 'ฯ', PAIYANNOI),
    (0xD0, 'ะ', SARA_A),
    (0xD1, 'ั', MAI_HAN_AKAT),
    (0xD2, 'า', SARA_AA),
    (0xD3, 'ำ', SARA_AM),
    (0xD4, 'ิ', SARA_I),
    (0xD5, 'ี', SARA_II),
    (0xD6, 'ึ', SARA_UE),
    (0xD7, 'ื', SARA_UEE),
    (0xD8, 'ุ', SARA_U),
    (0xD9, 'ู', SARA_UU),
    (0xDA, 'ฺ', PHINTHU),
    (0xDF, '฿', BAHT),
    (0xE0, 'เ', SARA_E),
    (0xE1, 'แ', SARA_AE),
    (0xE2, 'โ', SARA_O),
    (0xE3, 'ใ', SARA_AI_MAIMUAN),
    (0xE4, 'ไ', SARA_AI_MAIMALAI),
    (0xE5, 'ๅ', LAKKHANGYAO),
    (0xE6, 'ๆ', MAIYAMOK),
    (0xE7, '็', MAITAIKHU),
    (0xE8, '่', MAI_EK),
    (0xE9, '้', MAI_THO),
    (0xEA, '๊', MAI_TRI),
    (0xEB, '๋', MAI_CHATTAWA),
    (0xEC, '์', THANTHAKHAT),
    (0xED, 'ํ', NIKHAHIT),
    (0xEE, '๎', YAMAKKAN),
    (0xEF, '๏', FONGMAN),
    (0xF0, '๐', ZERO),
    (0xF1, '๑', ONE),
    (0xF2, '๒', TWO),
    (0xF3, '๓', THREE),
    (0xF4, '๔', FOUR),
    (0xF5, '๕', FIVE),
    (0xF6, '๖', SIX),
    (0xF7, '๗', SEVEN),
    (0xF8, '๘', EIGHT),
    (0xF9, '๙', NINE),
    (0xFA, '๚', ANGKHANKHU),
    (0xFB, '๛', KHOMUT),
]);

impl Display for ThaiChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.to_char())
    }
}

impl TryFrom<char> for ThaiChar {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        ThaiChar::from_char(ch).ok_or(())
    }
}

impl From<ThaiChar> for char {
    fn from(tc: ThaiChar) -> Self {
        tc.to_char()
    }
}

impl From<ThaiChar> for u8 {
    fn from(tc: ThaiChar) -> Self {
        tc.to_tis620_byte()
    }
}

/// TIS-620 replacement character.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReplacementChar(u8);

impl ReplacementChar {
    pub fn from_char(ch: char) -> Option<Self> {
        let byte = if ch.is_ascii() {
            ch as u32 as u8
        } else {
            ThaiChar::from_char(ch)?.to_tis620_byte()
        };
        Some(Self(byte))
    }

    pub fn to_tis620_byte(self) -> u8 {
        self.0
    }
}
