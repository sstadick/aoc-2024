use std::iter;

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
pub enum MemoryBlock {
    Free,
    File { id: usize },
}

impl MemoryBlock {
    #[inline]
    pub fn is_free(&self) -> bool {
        matches!(self, Self::Free)
    }

    #[inline]
    pub fn unchecked_get_file_id(&self) -> usize {
        match self {
            Self::File { id } => *id,
            _ => panic!("Attempted to access file id on non-file block."),
        }
    }
}

pub fn expand(input: &[u8]) -> Vec<MemoryBlock> {
    let mut result = vec![];
    let mut current_file_id = 0;
    let mut is_file = true;
    for byte in input.iter() {
        let num = byte - 48;
        let record = if is_file {
            let id = current_file_id;
            current_file_id += 1;
            MemoryBlock::File { id }
        } else {
            MemoryBlock::Free
        };
        result.extend(iter::repeat(record).take(num as usize));
        is_file = !is_file;
    }

    result
}

/// Move file blocks one at a time from the end of the disk to the leftmost
/// free space block (until there are no gaps remaining between file blocks).
pub fn naive_compaction(mem: &mut [MemoryBlock]) {
    let mut lhs_offset = 0;
    let mut rhs_offset = mem.len() - 1;

    while lhs_offset < rhs_offset {
        // advance lhs_offset to the next free space
        while lhs_offset < mem.len() && !mem[lhs_offset].is_free() {
            lhs_offset += 1;
        }

        // advance rhs_offset to the next file block
        while rhs_offset > lhs_offset && mem[rhs_offset].is_free() {
            rhs_offset -= 1;
        }

        // Swap the two elements
        mem.swap(lhs_offset, rhs_offset);
        lhs_offset += 1;
        rhs_offset = rhs_offset.saturating_sub(1);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ContiguousBlock {
    kind: MemoryBlock,
    // inclusive
    start: usize,
    // exclusive
    end: usize,
}

impl ContiguousBlock {
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub fn less_naive_compaction(mem: &mut [MemoryBlock]) {
    // Buildup structure of free memory
    let mut free_mem = vec![];
    let mut files = vec![];
    for (is_free, mut chunk) in &mem
        .iter()
        .enumerate()
        .chunk_by(|(_i, block)| block.is_free())
    {
        if is_free {
            let (first_block_index, first_block) = chunk.next().unwrap();
            let (last_block_index, _last_block) =
                chunk.last().unwrap_or((first_block_index, first_block));
            let c_block = ContiguousBlock {
                kind: *first_block,
                start: first_block_index,
                end: last_block_index + 1,
            };
            free_mem.push(c_block);
        } else {
            // Files may be back to back
            for (_file_id, mut chunk) in
                &chunk.chunk_by(|(_i, block)| block.unchecked_get_file_id())
            {
                let (first_block_index, first_block) = chunk.next().unwrap();
                let (last_block_index, _last_block) =
                    chunk.last().unwrap_or((first_block_index, first_block));
                let c_block = ContiguousBlock {
                    kind: *first_block,
                    start: first_block_index,
                    end: last_block_index + 1,
                };
                files.push(c_block)
            }
        };
    }

    // Go through files right to left
    for file in files.into_iter().rev() {
        let size = file.len();
        // Find the most leftmost space that will fit the file
        let Some((free_index, _free_mem_block)) = free_mem
            .iter()
            .find_position(|block| block.len() >= size && block.start < file.start)
        else {
            continue;
        };

        let free_mem_block = &mut free_mem[free_index];

        // Move the file
        for (file_index, free_index) in
            (file.start..file.end).zip(free_mem_block.start..free_mem_block.end)
        {
            mem.swap(file_index, free_index);
        }

        // Update the free list
        free_mem_block.start += file.len();
    }
}

pub fn checksum(mem: &[MemoryBlock]) -> usize {
    mem.iter()
        .enumerate()
        .filter(|(_i, block)| !block.is_free())
        .map(|(i, block)| i * block.unchecked_get_file_id())
        .sum()
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let mut expanded = expand(input);
    naive_compaction(&mut expanded);
    let checksum = checksum(&expanded);
    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"2333133121414131402";

        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
