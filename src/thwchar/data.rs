////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

pub(crate) const WC_ERR: char = char::MAX;
pub(crate) const TH_ERR: u8 = u8::MAX;

pub(crate) const TIS2UNI_TABLE: [char; 128] = [
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, '\u{0e01}', '\u{0e02}',
    '\u{0e03}', '\u{0e04}', '\u{0e05}', '\u{0e06}', '\u{0e07}', '\u{0e08}', '\u{0e09}', '\u{0e0a}',
    '\u{0e0b}', '\u{0e0c}', '\u{0e0d}', '\u{0e0e}', '\u{0e0f}', '\u{0e10}', '\u{0e11}', '\u{0e12}',
    '\u{0e13}', '\u{0e14}', '\u{0e15}', '\u{0e16}', '\u{0e17}', '\u{0e18}', '\u{0e19}', '\u{0e1a}',
    '\u{0e1b}', '\u{0e1c}', '\u{0e1d}', '\u{0e1e}', '\u{0e1f}', '\u{0e20}', '\u{0e21}', '\u{0e22}',
    '\u{0e23}', '\u{0e24}', '\u{0e25}', '\u{0e26}', '\u{0e27}', '\u{0e28}', '\u{0e29}', '\u{0e2a}',
    '\u{0e2b}', '\u{0e2c}', '\u{0e2d}', '\u{0e2e}', '\u{0e2f}', '\u{0e30}', '\u{0e31}', '\u{0e32}',
    '\u{0e33}', '\u{0e34}', '\u{0e35}', '\u{0e36}', '\u{0e37}', '\u{0e38}', '\u{0e39}', '\u{0e3a}',
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, '\u{0e3f}', '\u{0e40}', '\u{0e41}', '\u{0e42}', '\u{0e43}',
    '\u{0e44}', '\u{0e45}', '\u{0e46}', '\u{0e47}', '\u{0e48}', '\u{0e49}', '\u{0e4a}', '\u{0e4b}',
    '\u{0e4c}', '\u{0e4d}', '\u{0e4e}', '\u{0e4f}', '\u{0e50}', '\u{0e51}', '\u{0e52}', '\u{0e53}',
    '\u{0e54}', '\u{0e55}', '\u{0e56}', '\u{0e57}', '\u{0e58}', '\u{0e59}', '\u{0e5a}', '\u{0e5b}',
    WC_ERR, WC_ERR, WC_ERR, WC_ERR,
];

pub(crate) const UNI2TIS_TABLE: [u8; 96] = [
    TH_ERR, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae,
    0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe,
    0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce,
    0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, TH_ERR, TH_ERR, TH_ERR,
    TH_ERR, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec,
    0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb,
    TH_ERR, TH_ERR, TH_ERR, TH_ERR,
];

pub(crate) const MACTHAI2UNI_TABLE: [char; 128] = [
    '\u{00ab}', '\u{00bb}', '\u{2026}', '\u{f88c}', '\u{f88f}', '\u{f892}', '\u{f895}', '\u{f898}',
    '\u{f88b}', '\u{f88e}', '\u{f891}', '\u{f894}', '\u{f897}', '\u{201c}', '\u{201d}', '\u{f899}',
    WC_ERR, '\u{2022}', '\u{f884}', '\u{f889}', '\u{f885}', '\u{f886}', '\u{f887}', '\u{f888}',
    '\u{f88a}', '\u{f88d}', '\u{f890}', '\u{f893}', '\u{f896}', '\u{2018}', '\u{2019}', WC_ERR,
    '\u{00a0}', '\u{0e01}', '\u{0e02}', '\u{0e03}', '\u{0e04}', '\u{0e05}', '\u{0e06}', '\u{0e07}',
    '\u{0e08}', '\u{0e09}', '\u{0e0a}', '\u{0e0b}', '\u{0e0c}', '\u{0e0d}', '\u{0e0e}', '\u{0e0f}',
    '\u{0e10}', '\u{0e11}', '\u{0e12}', '\u{0e13}', '\u{0e14}', '\u{0e15}', '\u{0e16}', '\u{0e17}',
    '\u{0e18}', '\u{0e19}', '\u{0e1a}', '\u{0e1b}', '\u{0e1c}', '\u{0e1d}', '\u{0e1e}', '\u{0e1f}',
    '\u{0e20}', '\u{0e21}', '\u{0e22}', '\u{0e23}', '\u{0e24}', '\u{0e25}', '\u{0e26}', '\u{0e27}',
    '\u{0e28}', '\u{0e29}', '\u{0e2a}', '\u{0e2b}', '\u{0e2c}', '\u{0e2d}', '\u{0e2e}', '\u{0e2f}',
    '\u{0e30}', '\u{0e31}', '\u{0e32}', '\u{0e33}', '\u{0e34}', '\u{0e35}', '\u{0e36}', '\u{0e37}',
    '\u{0e38}', '\u{0e39}', '\u{0e3a}', '\u{feff}', '\u{200b}', '\u{2013}', '\u{2014}', '\u{0e3f}',
    '\u{0e40}', '\u{0e41}', '\u{0e42}', '\u{0e43}', '\u{0e44}', '\u{0e45}', '\u{0e46}', '\u{0e47}',
    '\u{0e48}', '\u{0e49}', '\u{0e4a}', '\u{0e4b}', '\u{0e4c}', '\u{0e4d}', '\u{2122}', '\u{0e4f}',
    '\u{0e50}', '\u{0e51}', '\u{0e52}', '\u{0e53}', '\u{0e54}', '\u{0e55}', '\u{0e56}', '\u{0e57}',
    '\u{0e58}', '\u{0e59}', '\u{00ae}', '\u{00a9}', WC_ERR, WC_ERR, WC_ERR, WC_ERR,
];

pub(crate) const WINTHAI2UNI_TABLE: [char; 128] = [
    '\u{f700}', '\u{f701}', '\u{f702}', '\u{f703}', '\u{f704}', '\u{2026}', '\u{f705}', '\u{f706}',
    '\u{f707}', '\u{f708}', '\u{f709}', '\u{f70a}', '\u{f70b}', '\u{f70c}', '\u{f70d}', '\u{f70e}',
    '\u{f70f}', '\u{2018}', '\u{2019}', '\u{201c}', '\u{201d}', '\u{2022}', '\u{2013}', '\u{2014}',
    '\u{f710}', '\u{f711}', '\u{f712}', '\u{f713}', '\u{f714}', '\u{f715}', '\u{f716}', '\u{f717}',
    '\u{00a0}', '\u{0e01}', '\u{0e02}', '\u{0e03}', '\u{0e04}', '\u{0e05}', '\u{0e06}', '\u{0e07}',
    '\u{0e08}', '\u{0e09}', '\u{0e0a}', '\u{0e0b}', '\u{0e0c}', '\u{0e0d}', '\u{0e0e}', '\u{0e0f}',
    '\u{0e10}', '\u{0e11}', '\u{0e12}', '\u{0e13}', '\u{0e14}', '\u{0e15}', '\u{0e16}', '\u{0e17}',
    '\u{0e18}', '\u{0e19}', '\u{0e1a}', '\u{0e1b}', '\u{0e1c}', '\u{0e1d}', '\u{0e1e}', '\u{0e1f}',
    '\u{0e20}', '\u{0e21}', '\u{0e22}', '\u{0e23}', '\u{0e24}', '\u{0e25}', '\u{0e26}', '\u{0e27}',
    '\u{0e28}', '\u{0e29}', '\u{0e2a}', '\u{0e2b}', '\u{0e2c}', '\u{0e2d}', '\u{0e2e}', '\u{0e2f}',
    '\u{0e30}', '\u{0e31}', '\u{0e32}', '\u{0e33}', '\u{0e34}', '\u{0e35}', '\u{0e36}', '\u{0e37}',
    '\u{0e38}', '\u{0e39}', '\u{0e3a}', WC_ERR, WC_ERR, WC_ERR, WC_ERR, '\u{0e3f}', '\u{0e40}',
    '\u{0e41}', '\u{0e42}', '\u{0e43}', '\u{0e44}', '\u{0e45}', '\u{0e46}', '\u{0e47}', '\u{0e48}',
    '\u{0e49}', '\u{0e4a}', '\u{0e4b}', '\u{0e4c}', '\u{0e4d}', '\u{0e4e}', '\u{0e4f}', '\u{0e50}',
    '\u{0e51}', '\u{0e52}', '\u{0e53}', '\u{0e54}', '\u{0e55}', '\u{0e56}', '\u{0e57}', '\u{0e58}',
    '\u{0e59}', '\u{0e5a}', '\u{0e5b}', '\u{f718}', '\u{f719}', '\u{f71a}', WC_ERR,
];
