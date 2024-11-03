//! # B+IdTree
//!
//! B+IdTree is a B+Tree implementation for storing 128bit Ids.
//! All node size is 4KB, which will be called a page.
//! Page Index size is 31 bit.
//! B+IdTree can store 2^31 pages, total Ids are 2^31 * 255 = 2^38.
//!
//! Page Index '0' will be used as a null.
//! Endian is little.
//!
//! ## File Structure
//!
//! ### Free Page Stack
//!
//! Linked List, storing free page's offset in the file.
//! - Next Free Page Stack Node Index: u32
//! - Length in this page: u32
//! - Free Page Indexes: [u32; 1022]
//!
//! ### Header
//! Header has one page size.
//! - Free Page Stack Top Node Index: u32
//! - Root Node Index: u32
//!   - Root Node would be an Internal Node or a Leaf Node.
//!
//! ### Internal Node
//! - Leaf Type Bit and Parent Node Index: u32
//!   - 31 bit: Parent Node Index
//!   - MSB 1 bit: 0 for Internal Node
//!
//! ### Leaf Node
//! - Leaf Type Bit and Parent Node Index: u32
//!   - 31 bit: Parent Node Index
//!   - MSB 1 bit: 1 for Leaf Node

mod wal;

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Result, Seek, SeekFrom, Write},
    mem::MaybeUninit,
    num::NonZeroU32,
    path::Path,
};
use wal::*;

pub struct BpIdTree {
    file: File,
    wal: Wal,
    header: Header,
    // TODO: Remove nodes cache for memory usage.
    nodes: HashMap<PageIndex, Node>,
}

impl BpIdTree {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let mut wal = Wal::open(path.with_extension("wal"))?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        wal.flush(&mut file)?;

        if file.metadata()?.len() == 0 {
            wal.write_init()?;
            wal.flush(&mut file)?;
        }

        Self::read_from_file(file, wal)
    }
    pub fn insert(&mut self, id: u128) -> Result<()> {
        let (node_index, mut leaf_node) = self.find_leaf_node_for_insertion(id)?;
        if !leaf_node.is_full() {
            self.wal.write_insert_to_leaf_node(node_index, id)?;

            leaf_node.insert(id);
            self.nodes.insert(node_index, leaf_node.into_node());

            return Ok(());
        }
        // - Otherwise, before inserting the new record
        //     - Split the node.
        //         - original node has ⌈(K+1)/2⌉ items
        //         - new node has ⌊(K+1)/2⌋ items
        //     - Copy ⌈(K+1)/2⌉-th key to the parent, and insert the new node to the parent.
        //     - Repeat until a parent is found that need not split.
        //     - Insert the new record into the new node.
        // - If the root splits, treat it as if it has an empty parent and split as outline above.
        todo!()
    }
    pub fn delete(&mut self, id: u128) -> Result<()> {
        todo!()
    }
    pub fn iter(&self) -> Result<impl Iterator<Item = u128>> {
        // TODO
        Ok(std::iter::empty())
    }
    fn find_leaf_node_for_insertion(&mut self, id: u128) -> Result<(PageIndex, LeafNode)> {
        let mut node_index = self.header.root_node_index;

        loop {
            let node = self.node(node_index)?;
            if node.is_leaf() {
                return Ok((node_index, node.into_leaf_node()));
            }
            let internal_node = node.into_internal_node();
            node_index = internal_node.find_child_node_index_for(id);
        }
    }
    fn node(&mut self, node_index: PageIndex) -> Result<Node> {
        if let Some(node) = self.nodes.get(&node_index) {
            return Ok(*node);
        }

        self.read_node_from_file(node_index)
    }
    fn read_node_from_file(&mut self, node_index: PageIndex) -> Result<Node> {
        self.wal.flush(&mut self.file)?;

        let node = read_node_from_file(&mut self.file, node_index)?;

        self.nodes.insert(node_index, node);

        Ok(node)
    }
    fn read_from_file(mut file: File, wal: Wal) -> Result<Self> {
        let header = unsafe {
            let mut header = MaybeUninit::<Header>::uninit();
            file.seek(SeekFrom::Start(0))?;
            file.read_exact(std::slice::from_raw_parts_mut(
                header.as_mut_ptr() as *mut u8,
                std::mem::size_of::<Header>(),
            ))?;
            header.assume_init()
        };

        Ok(Self {
            file,
            wal,
            header,
            nodes: HashMap::new(),
        })
    }
}

fn read_node_from_file(file: &mut File, node_index: PageIndex) -> Result<Node> {
    file.seek(node_index.file_pos())?;

    let node = unsafe {
        let mut node = MaybeUninit::<Node>::uninit();
        file.read_exact(std::slice::from_raw_parts_mut(
            node.as_mut_ptr() as *mut u8,
            std::mem::size_of::<Node>(),
        ))?;
        node.assume_init()
    };
    Ok(node)
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PageIndex {
    value: u32,
}
impl PageIndex {
    const NULL: Self = Self { value: 0 };
    fn with_node_type_msb(&self, is_leaf: bool) -> u32 {
        self.value | if is_leaf { 0x80000000 } else { 0 }
    }

    fn without_node_type_msb(value: NonZeroU32) -> PageIndex {
        Self { value: value.get() }
    }

    fn file_pos(&self) -> SeekFrom {
        SeekFrom::Start((self.value & 0x7FFFFFFF) as u64 * 4096)
    }
}

trait AsSlice: Sized {
    fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self as *const _ as *const u8, std::mem::size_of::<Self>())
        }
    }
}

#[repr(C)]
struct Header {
    free_page_stack_top_page_index: PageIndex,
    root_node_index: PageIndex,
    padding: [u32; 1022],
}
impl AsSlice for Header {}

#[repr(C)]
struct FreePageStackNode {
    next_node_index: PageIndex,
    length: u32,
    free_page_indexes: [u32; 1022],
}
impl AsSlice for FreePageStackNode {}

#[repr(C)]
struct InternalNode {
    leaf_type_bit_and_parent_index: u32,
    id_count: u32,
    ids: [u128; 203],
    child_indexes: [PageIndex; 204],
    _padding: u32,
}
impl AsSlice for InternalNode {}

impl InternalNode {
    fn offset(&self) -> PageIndex {
        PageIndex {
            value: self.leaf_type_bit_and_parent_index & 0x7FFFFFFF,
        }
    }

    fn find_child_node_index_for(&self, id: u128) -> PageIndex {
        self.ids
            .iter()
            .take(self.id_count as usize)
            .enumerate()
            .find(|(_, &key_id)| id < key_id)
            .map(|(i, _)| self.child_indexes[i])
            .unwrap_or(self.child_indexes[self.id_count as usize])
    }
}

#[repr(C)]
struct LeafNode {
    leaf_type_bit_and_parent_node_index: u32,
    id_count: u32,
    ids: [u128; 255],
}
impl AsSlice for LeafNode {}

impl LeafNode {
    fn parent_node_index(&self) -> PageIndex {
        PageIndex {
            value: self.leaf_type_bit_and_parent_node_index & 0x7FFFFFFF,
        }
    }

    fn new(parent_node_index: PageIndex) -> Self {
        Self {
            leaf_type_bit_and_parent_node_index: parent_node_index.with_node_type_msb(true),
            id_count: 0,
            ids: [0; 255],
        }
    }

    fn is_full(&self) -> bool {
        self.id_count == self.ids.len() as u32
    }

    fn insert(&mut self, id: u128) {
        let index = self
            .ids
            .iter()
            .take(self.id_count as usize)
            .enumerate()
            .find(|(_, &key_id)| id < key_id)
            .map(|(i, _)| i)
            .unwrap_or(self.id_count as usize);
        self.ids.copy_within(index.., index + 1);
        self.ids[index] = id;
        self.id_count += 1;
    }

    fn into_node(self) -> Node {
        unsafe { std::mem::transmute(self) }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Node {
    leaf_type_bit_and_parent_node_index: u32,
    _padding: [u8; 4092],
}
impl AsSlice for Node {}
impl Node {
    fn is_leaf(&self) -> bool {
        self.leaf_type_bit_and_parent_node_index & 0x80000000 != 0
    }
    fn into_internal_node(self) -> InternalNode {
        unsafe { std::mem::transmute(self) }
    }
    fn into_leaf_node(self) -> LeafNode {
        unsafe { std::mem::transmute(self) }
    }
    fn as_internal_node_mut(&mut self) -> &mut InternalNode {
        unsafe { std::mem::transmute(self) }
    }
    fn as_leaf_node_mut(&mut self) -> &mut LeafNode {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_size() {
        assert_eq!(std::mem::size_of::<Header>(), 4096);
        assert_eq!(std::mem::size_of::<FreePageStackNode>(), 4096);
        assert_eq!(std::mem::size_of::<InternalNode>(), 4096);
        assert_eq!(std::mem::size_of::<LeafNode>(), 4096);
        assert_eq!(std::mem::size_of::<Node>(), 4096);
    }
}
