//! # B+IdTree
//!
//! B+IdTree is a B+Tree implementation for storing 128bit Keys.
//! All node size is 4KB, which will be called a page.
//! Page Offset size is 32 bit.
//! B+IdTree can store 2^32 pages, total Keys are 2^32 * 255 = 2^40.
//!
//! Page Offset '0' will be used as a null.
//! Endian is little.
//!
//! ## File Structure
//!
//! ### Free Page Stack
//!
//! Linked List, storing free page's offset in the file.
//! - Next Free Page Stack Node Offset: u32
//! - Length in this page: u32
//! - Free Page Keys: [u32; 1022]
//!

mod operator;
mod wal;

use operator::Operator;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Result, Seek, SeekFrom},
    mem::MaybeUninit,
    path::Path,
};
use wal::*;

pub struct BpIdTree {
    file: File,
    wal: Wal,
    header: Header,
    // TODO: Remove nodes cache for memory usage.
    pages: HashMap<PageOffset, Page>,
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
    pub fn insert(&mut self, key: u128) -> Result<()> {
        let done = Operator::new(&self.header, &self.pages, &mut self.file).insert(key)?;
        self.wal.write_logs(done.logs)?;
        if let Some(header) = done.updated_header {
            self.header = header;
        }
        for (page_offset, page) in done.updated_pages {
            self.pages.insert(page_offset, page);
        }
        Ok(())
    }
    pub fn delete(&mut self, key: u128) -> Result<()> {
        todo!()
    }
    pub fn iter(&self) -> Result<impl Iterator<Item = u128>> {
        // TODO
        Ok(std::iter::empty())
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
            pages: HashMap::new(),
        })
    }
}

fn read_page_from_file(file: &mut File, page_offset: PageOffset) -> Result<Page> {
    file.seek(page_offset.file_pos())?;

    let page = unsafe {
        let mut page = MaybeUninit::<Page>::uninit();
        file.read_exact(std::slice::from_raw_parts_mut(
            page.as_mut_ptr() as *mut u8,
            std::mem::size_of::<Page>(),
        ))?;
        page.assume_init()
    };
    Ok(page)
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PageOffset {
    value: u32,
}
impl PageOffset {
    const NULL: Self = Self { value: 0 };
    fn new(value: u32) -> PageOffset {
        Self { value }
    }

    fn file_pos(&self) -> SeekFrom {
        SeekFrom::Start(self.value as u64 * 4096)
    }

    fn fetch_increase(&mut self) -> Self {
        let next = *self;
        self.value += 1;
        next
    }

    fn is_null(&self) -> bool {
        self == &Self::NULL
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
#[derive(Debug, Clone, Copy)]
struct Header {
    /// Would be null
    free_page_stack_top_page_offset: PageOffset,
    /// Root node would be a leaf node or an internal node.
    root_node_offset: PageOffset,
    /// Use this value to allocate new page.
    next_page_offset: PageOffset,
    _padding: [u32; 1021],
}
impl AsSlice for Header {}

#[repr(C)]
struct FreePageStackNode {
    next_page_offset: PageOffset,
    length: u32,
    /// would have dirty data.
    free_page_keys: [u32; 1022],
}
impl FreePageStackNode {
    fn pop(&mut self) -> PageOffset {
        assert_ne!(self.length, 0);
        self.length -= 1;
        let offset = self.free_page_keys[self.length as usize];
        assert_ne!(offset, 0);
        PageOffset::new(offset)
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
}
impl AsSlice for FreePageStackNode {}

const INTERNAL_NODE_KEY_LEN: usize = 203;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// right side child's key is greater or equal to key.
struct InternalNode {
    leaf_type: u8,
    _padding: [u8; 3],
    key_count: u32,
    keys: [u128; INTERNAL_NODE_KEY_LEN],
    child_offsets: [PageOffset; INTERNAL_NODE_KEY_LEN + 1],
    _padding_1: u32,
}
impl AsSlice for InternalNode {}

impl InternalNode {
    pub fn new(
        key: u128,
        left_side_child_offset: PageOffset,
        right_side_child_offset: PageOffset,
    ) -> Self {
        let mut keys = [0; INTERNAL_NODE_KEY_LEN];
        keys[0] = key;

        let mut child_offsets = [PageOffset::NULL; INTERNAL_NODE_KEY_LEN + 1];
        child_offsets[0] = left_side_child_offset;
        child_offsets[1] = right_side_child_offset;

        Self {
            leaf_type: 0,
            _padding: [0; 3],
            key_count: 1,
            keys,
            child_offsets,
            _padding_1: 0,
        }
    }
    fn key_index(&self, key: u128) -> Option<usize> {
        self.keys
            .iter()
            .take(self.key_count as usize)
            .enumerate()
            .find(|(_, &key1)| key < key1)
            .map(|(i, _)| i)
    }
    fn find_child_node_offset_for(&self, key: u128) -> PageOffset {
        self.key_index(key)
            .map(|i| self.child_offsets[i])
            .unwrap_or(self.child_offsets[self.key_count as usize])
    }
    fn is_full(&self) -> bool {
        self.key_count == self.keys.len() as u32
    }
    fn insert(&mut self, key: u128, right_side_child_offset: PageOffset) -> Option<InternalNode> {
        let key_index = self.key_index(key).unwrap_or(self.key_count as usize);

        if !self.is_full() {
            if key_index < self.key_count as usize {
                self.keys[key_index..].rotate_right(1);
                self.child_offsets[key_index + 1..].rotate_right(1);
            }
            self.keys[key_index] = key;
            self.child_offsets[key_index + 1] = right_side_child_offset;
            self.key_count += 1;
            return None;
        }

        let one_plus_keys = {
            let mut keys = [0; INTERNAL_NODE_KEY_LEN + 1];
            keys[..key_index].copy_from_slice(&self.keys[..key_index]);
            keys[key_index] = key;
            keys[key_index + 1..].copy_from_slice(&self.keys[key_index..]);
            keys
        };
        let one_plus_child_offsets = {
            let mut offsets = [PageOffset::NULL; INTERNAL_NODE_KEY_LEN + 2];
            offsets[..key_index + 1].copy_from_slice(&self.child_offsets[..key_index + 1]);
            offsets[key_index + 1] = right_side_child_offset;
            offsets[key_index + 2..].copy_from_slice(&self.child_offsets[key_index + 1..]);
            offsets
        };

        let floor = one_plus_keys.len() / 2;
        let ceil = one_plus_keys.len() - floor;

        let mut right_node = unsafe { std::mem::zeroed::<InternalNode>() };
        let key_count = floor - 1;
        let offset_count = key_count + 1;
        right_node.key_count = key_count as u32;
        right_node.keys[..key_count]
            .copy_from_slice(&one_plus_keys[(one_plus_keys.len() - key_count)..]);
        right_node.child_offsets[..offset_count].copy_from_slice(
            &one_plus_child_offsets[(one_plus_child_offsets.len() - offset_count)..],
        );

        let key_count = ceil;
        let offset_count = key_count + 1;
        self.key_count = key_count as u32;
        self.keys[..key_count].copy_from_slice(&one_plus_keys[..key_count]);
        self.child_offsets[..offset_count].copy_from_slice(&one_plus_child_offsets[..offset_count]);

        Some(right_node)
    }
}

const LEAF_NODE_KEYS_LEN: usize = 255;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct LeafNode {
    leaf_type: u8,
    _padding: [u8; 3],
    id_count: u32,
    keys: [u128; LEAF_NODE_KEYS_LEN],
}
impl AsSlice for LeafNode {}

impl LeafNode {
    fn new() -> Self {
        Self {
            leaf_type: 1,
            _padding: [0; 3],
            id_count: 0,
            keys: [0; LEAF_NODE_KEYS_LEN],
        }
    }

    fn is_full(&self) -> bool {
        self.id_count == self.keys.len() as u32
    }

    /// Return new splitted leaf node and new key if it's full.
    /// New leaf node will have half of the keys, bigger values.
    fn insert(&mut self, key: u128) -> Option<(LeafNode, u128)> {
        let offset = self
            .keys
            .iter()
            .take(self.id_count as usize)
            .enumerate()
            .find(|(_, &key_id)| key < key_id)
            .map(|(i, _)| i)
            .unwrap_or(self.id_count as usize);
        if self.is_full() {
            let one_plus_keys = {
                let mut one_plus_keys = [0; LEAF_NODE_KEYS_LEN + 1];
                one_plus_keys[..offset].copy_from_slice(&self.keys[..offset]);
                one_plus_keys[offset] = key;
                one_plus_keys[offset + 1..].copy_from_slice(&self.keys[offset..]);
                one_plus_keys
            };

            let floor = one_plus_keys.len() / 2;
            let ceil = one_plus_keys.len() - floor;

            self.keys[..ceil].copy_from_slice(&one_plus_keys[..ceil]);
            self.id_count = ceil as u32;

            let mut new_leaf_node = LeafNode::new();

            new_leaf_node.keys[..floor].copy_from_slice(&one_plus_keys[ceil..]);
            new_leaf_node.id_count = floor as u32;

            Some((new_leaf_node, self.keys[ceil - 1]))
        } else {
            if offset < self.id_count as usize {
                self.keys[offset..].rotate_right(1);
            }
            self.keys[offset] = key;
            self.id_count += 1;
            None
        }
    }

    fn into_node(self) -> Node {
        unsafe { std::mem::transmute(self) }
    }

    fn into_page(self) -> Page {
        unsafe { std::mem::transmute(self) }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Node {
    leaf_type: u8,
    _padding: [u8; 4095],
}
impl AsSlice for Node {}
impl Node {
    fn is_leaf(&self) -> bool {
        self.leaf_type != 0
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

    fn into_page(self) -> Page {
        unsafe { std::mem::transmute(self) }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Page {
    data: [u8; 4096],
}

impl Page {
    fn new() -> Self {
        Self { data: [0; 4096] }
    }

    fn into_node(self) -> Node {
        unsafe { std::mem::transmute(self) }
    }

    fn into_free_page_stack_node(self) -> FreePageStackNode {
        unsafe { std::mem::transmute(self) }
    }

    fn as_header(&self) -> &Header {
        unsafe { std::mem::transmute(self) }
    }

    fn as_free_page_stack_node_mut(&mut self) -> &mut FreePageStackNode {
        unsafe { std::mem::transmute(self) }
    }

    fn as_leaf_node_mut(&mut self) -> &mut LeafNode {
        unsafe { std::mem::transmute(self) }
    }

    fn as_leaf_node(&self) -> &LeafNode {
        unsafe { std::mem::transmute(self) }
    }

    fn into_leaf_node(self) -> LeafNode {
        unsafe { std::mem::transmute(self) }
    }

    fn as_internal_node(&self) -> &InternalNode {
        unsafe { std::mem::transmute(self) }
    }

    fn as_internal_node_mut(&mut self) -> &mut InternalNode {
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

    #[test]
    fn leaf_node_move_half() {
        let mut leaf_node = LeafNode::new();
        for i in (0..(255 * 2)).step_by(2) {
            assert!(leaf_node.insert(i).is_none());
        }

        let (new_leaf_node, key) = leaf_node.insert(3).unwrap();

        assert_eq!(leaf_node.id_count, 128);
        assert_eq!(new_leaf_node.id_count, 128);
        assert_eq!(key, 252);
        assert_eq!(key, leaf_node.keys[leaf_node.id_count as usize - 1]);

        let leaf_node_keys = {
            let mut keys = (0..=253).step_by(2).collect::<Vec<_>>();
            keys.push(3);
            keys.sort();
            keys
        };
        assert_eq!(leaf_node.keys[..128], leaf_node_keys);

        let new_leaf_node_keys = (254..510).step_by(2).collect::<Vec<_>>();
        assert_eq!(new_leaf_node.keys[..128], new_leaf_node_keys);
    }

    #[test]
    fn internal_node_insert() {
        {
            let mut internal_node = InternalNode::new(3, PageOffset::new(1), PageOffset::new(2));
            assert!(internal_node.insert(1, PageOffset::new(3)).is_none());
        }
    }

    #[test]
    fn internal_node_insert_split() {
        let mut internal_node = InternalNode::new(1, PageOffset::new(0), PageOffset::new(1));
        for i in 1..INTERNAL_NODE_KEY_LEN {
            assert!(internal_node
                .insert(i as u128 + 1, PageOffset::new(i as u32 + 1))
                .is_none());
        }
        let right_node = internal_node
            .insert(
                INTERNAL_NODE_KEY_LEN as u128 + 1,
                PageOffset::new(INTERNAL_NODE_KEY_LEN as u32 + 2),
            )
            .unwrap();

        let floor = INTERNAL_NODE_KEY_LEN / 2;
        let ceil = INTERNAL_NODE_KEY_LEN - floor;

        assert_eq!(internal_node.key_count, ceil as u32);
        assert_eq!(right_node.key_count, internal_node.key_count - 1);

        for i in 0..internal_node.key_count as usize {
            assert_eq!(internal_node.keys[i], i as u128 + 1);
            assert_eq!(internal_node.child_offsets[i].value, i as u32);
        }

        for i in 0..right_node.key_count as usize {
            assert_eq!(right_node.keys[i], i as u128 + 3 + floor as u128);
            assert_eq!(
                right_node.child_offsets[i].value,
                i as u32 + 2 + floor as u32
            );
        }
    }
}
