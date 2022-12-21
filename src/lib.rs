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

mod cursor;
pub mod data;
mod thbrk;
mod thwchar;
mod utils;

pub use thbrk::{DatrieBrk, TisBreaker};
pub use thwchar::{
    macthai2string, macthai2uni, str2macthai, str2tis, str2winthai, tis2string, tis2uni,
    uni2macthai, uni2tis, uni2winthai, winthai2string, winthai2uni,
};
