use std::ops::Deref;

use serde::{Deserialize, Serialize};
use yada::DoubleArray;

use crate::viterbi::WordEntry;

#[derive(Serialize, Deserialize)]
#[serde(remote = "DoubleArray")]
struct DoubleArrayDef<T>(pub T)
where
    T: Deref<Target = [u8]>;

#[derive(Clone, Serialize, Deserialize)]
pub struct PrefixDictionary<T: Deref<Target = [u8]>> {
    #[serde(with = "DoubleArrayDef")]
    pub da: DoubleArray<T>,
    pub vals_data: T,
    pub words_idx_data: T,
    pub words_data: T,
    pub is_system: bool,
}

impl<T: Deref<Target = [u8]>> PrefixDictionary<T> {
    pub fn load(
        da_data: T,
        vals_data: T,
        words_idx_data: T,
        words_data: T,
    ) -> PrefixDictionary<T> {
        let da = DoubleArray::new(da_data);

        PrefixDictionary {
            da,
            vals_data,
            words_idx_data,
            words_data,
            is_system: true,
        }
    }
}

impl<D: Deref<Target = [u8]>> PrefixDictionary<D> {
    pub fn prefix<'a>(&'a self, s: &'a str) -> impl Iterator<Item = (usize, WordEntry)> + 'a {
        self.da
            .common_prefix_search(s)
            .flat_map(move |(offset_len, prefix_len)| {
                let len = offset_len & ((1u32 << 5) - 1u32);
                let offset = offset_len >> 5u32;
                let offset_bytes = (offset as usize) * WordEntry::SERIALIZED_LEN;
                let data: &[u8] = &self.vals_data[offset_bytes..];
                (0..len as usize).map(move |i| {
                    (
                        prefix_len,
                        WordEntry::deserialize(
                            &data[WordEntry::SERIALIZED_LEN * i..],
                            self.is_system,
                        ),
                    )
                })
            })
    }

    /// Find `WordEntry`s with surface
    pub fn find_surface(&self, surface: &str) -> Vec<WordEntry> {
        match self.da.exact_match_search(surface) {
            Some(offset_len) => {
                let offset = offset_len >> 5u32;
                let offset_bytes = (offset as usize) * WordEntry::SERIALIZED_LEN;
                let data: &[u8] = &self.vals_data[offset_bytes..];
                let len = offset_len & ((1u32 << 5) - 1u32);
                (0..len as usize)
                    .map(|i| {
                        WordEntry::deserialize(
                            &data[WordEntry::SERIALIZED_LEN * i..],
                            self.is_system,
                        )
                    })
                    .collect::<Vec<WordEntry>>()
            }
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {}
