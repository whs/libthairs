mod brkpos;
mod c_api;
mod data;
mod datrie;

pub use self::datrie::DatrieBrk;

/// TisBreaker implement Thai word breaking algorithm with TIS-620 input
pub trait TisBreaker {
    fn find_breaks<'a>(&'a self, input: &'a [u8], max_out: usize) -> Vec<usize>;

    fn split<'a>(&'a self, input: &'a [u8]) -> Vec<&[u8]> {
        let breaks = self.find_breaks(input, input.len());
        let mut out = Vec::new();

        let mut last_break = 0;
        for brk in breaks {
            out.push(&input[last_break..brk]);
            last_break = brk;
        }
        let remainder = &input[last_break..];
        if remainder.len() > 0 {
            out.push(&remainder);
        }

        out
    }
}
