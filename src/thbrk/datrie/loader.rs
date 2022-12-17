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

use datrie::Trie;
use std::char::{decode_utf16, DecodeUtf16Error};
use std::io;
use std::io::Read;

/// load .tri file into fst
/// tri file should have UCS-4 encoding (UTF-32) but it seems that decoding with UTF16 works...
pub fn load<R: Read>(reader: &mut R) -> io::Result<fst::Set<Vec<u8>>> {
    let out = Trie::from_reader(reader)?;
    fst::Set::from_iter(out.iter().map(|item| {
        debug_assert!(item.1.is_none());

        decode_utf16(item.0.iter().map(|i| *i as u16))
            .collect::<Result<String, DecodeUtf16Error>>()
            .unwrap()
    }))
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
