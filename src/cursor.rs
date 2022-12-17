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

pub struct Cursor<'a, T> {
    inner: &'a mut [T],
    pos: usize,
}

impl<'a, T> Cursor<'a, T> {
    pub fn new(items: &'a mut [T]) -> Self {
        Self {
            inner: items,
            pos: 0,
        }
    }

    pub fn get_mut(&mut self) -> &mut [T] {
        self.inner
    }

    pub fn get_ref(&self) -> &[T] {
        self.inner
    }

    pub fn into_inner(self) -> &'a mut [T] {
        self.inner
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0 || self.pos == self.inner.len()
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining_slice(&self) -> &[T] {
        &self.inner[self.pos..]
    }

    pub fn write(&mut self, item: T) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.inner[self.pos] = item;
        self.pos += 1;
        1
    }
}

impl<'a, T: Copy> Cursor<'a, T> {
    pub fn copy_from(&mut self, item: &[T]) -> usize {
        let last_pos = (self.pos + item.len()).min(self.inner.len());
        let write_range = self.pos..last_pos;
        let write_len = write_range.len();

        self.inner[write_range].copy_from_slice(&item[..write_len]);
        self.pos += write_len;

        write_len
    }
}

impl<'a, T: Clone> Cursor<'a, T> {
    pub fn clone_from(&mut self, item: &[T]) -> usize {
        let last_pos = (self.pos + item.len()).min(self.inner.len());
        let write_range = self.pos..last_pos;
        let write_len = write_range.len();

        self.inner[write_range].clone_from_slice(&item[..write_len]);
        self.pos += write_len;

        write_len
    }
}

#[cfg(test)]
mod tests {
    use crate::cursor::Cursor;

    #[test]
    fn write_item() {
        let mut data = [0; 3];
        {
            let mut cur = Cursor::new(&mut data);
            cur.write(0);
            cur.write(1);
            cur.write(2);
            cur.write(3);
            cur.write(4);
            assert_eq!(cur.position(), 3);
            assert_eq!(cur.is_empty(), true);
            assert_eq!(cur.remaining_slice(), []);
        }
        assert_eq!(data, [0, 1, 2]);
    }

    #[test]
    fn write_item_partial() {
        let mut data = [0; 3];
        {
            let mut cur = Cursor::new(&mut data);
            cur.write(1);
            assert_eq!(cur.position(), 1);
            assert_eq!(cur.is_empty(), false);
            assert_eq!(cur.remaining_slice(), [0, 0]);
        }
        assert_eq!(data, [1, 0, 0]);
    }

    #[test]
    fn write_item_empty() {
        let mut data = [0; 0];
        {
            let mut cur = Cursor::new(&mut data);
            cur.write(1);
            assert_eq!(cur.position(), 0);
            assert_eq!(cur.is_empty(), true);
            assert_eq!(cur.remaining_slice(), []);
        }
    }

    #[test]
    fn write_items_copy() {
        let mut data = [0; 3];
        {
            let mut cur = Cursor::new(&mut data);
            cur.copy_from(&[1, 2, 3, 4, 5]);
            assert_eq!(cur.position(), 3);
            assert_eq!(cur.is_empty(), true);
            assert_eq!(cur.remaining_slice(), []);
        }
        assert_eq!(data, [1, 2, 3]);
    }

    #[test]
    fn write_items_copy_partial() {
        let mut data = [0; 3];
        {
            let mut cur = Cursor::new(&mut data);
            cur.copy_from(&[1]);
            assert_eq!(cur.position(), 1);
            assert_eq!(cur.is_empty(), false);
            assert_eq!(cur.remaining_slice(), [0, 0]);
        }
        assert_eq!(data, [1, 0, 0]);
    }

    #[test]
    fn write_items_copy_empty() {
        let mut data = [0; 0];
        {
            let mut cur = Cursor::new(&mut data);
            cur.copy_from(&[1]);
            assert_eq!(cur.position(), 0);
            assert_eq!(cur.is_empty(), true);
            assert_eq!(cur.remaining_slice(), []);
        }
    }

    #[test]
    fn write_items_clone() {
        let mut data = [0; 3];
        {
            let mut cur = Cursor::new(&mut data);
            cur.clone_from(&[1, 2, 3, 4, 5]);
            assert_eq!(cur.position(), 3);
            assert_eq!(cur.is_empty(), true);
            assert_eq!(cur.remaining_slice(), []);
        }
        assert_eq!(data, [1, 2, 3]);
    }

    #[test]
    fn write_items_clone_partial() {
        let mut data = [0; 3];
        {
            let mut cur = Cursor::new(&mut data);
            cur.clone_from(&[1]);
            assert_eq!(cur.position(), 1);
            assert_eq!(cur.is_empty(), false);
            assert_eq!(cur.remaining_slice(), [0, 0]);
        }
        assert_eq!(data, [1, 0, 0]);
    }

    #[test]
    fn write_items_clone_empty() {
        let mut data = [0; 0];
        {
            let mut cur = Cursor::new(&mut data);
            cur.clone_from(&[1]);
            assert_eq!(cur.position(), 0);
            assert_eq!(cur.is_empty(), true);
            assert_eq!(cur.remaining_slice(), []);
        }
    }
}
