use super::*;
use bytes::Buf;
use std::fmt::Debug;

pub(crate) const PAGE_LEN: usize = 4096;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PageOffset {
    value: u32,
}
impl PageOffset {
    pub const NULL: Self = Self { value: 0 };
    pub const HEADER: Self = Self { value: 0 };

    pub fn new(value: u32) -> PageOffset {
        Self { value }
    }

    pub fn file_offset(&self) -> usize {
        self.value as usize * PAGE_LEN
    }

    pub fn fetch_increase(&mut self, count: usize) -> Self {
        let next = *self;
        self.value += count as u32;
        next
    }

    pub fn is_null(&self) -> bool {
        self == &Self::NULL
    }

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

pub(crate) trait AsSlice: Sized {
    fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self as *const _ as *const u8, std::mem::size_of::<Self>())
        }
    }
}

#[repr(C, align(64))]
#[derive(Clone)]
pub(crate) struct Header {
    /// Would be null
    pub free_page_stack_top_page_offset: PageOffset,
    /// Root node would be a leaf node or an internal node.
    pub root_node_offset: PageOffset,
    /// Use this value to allocate new page.
    pub next_page_offset: PageOffset,
    _padding: [u32; 1021],
}
impl Header {
    pub fn new(
        free_page_stack_top_page_offset: PageOffset,
        root_node_offset: PageOffset,
        next_page_offset: PageOffset,
    ) -> Self {
        Self {
            free_page_stack_top_page_offset,
            root_node_offset,
            next_page_offset,
            _padding: [0; 1021],
        }
    }
}
impl AsSlice for Header {}
impl Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Header")
            .field(
                "free_page_stack_top_page_offset",
                &self.free_page_stack_top_page_offset,
            )
            .field("root_node_offset", &self.root_node_offset)
            .field("next_page_offset", &self.next_page_offset)
            .finish()
    }
}

#[repr(C, align(64))]
pub(crate) struct FreePageStackNode {
    pub next_page_offset: PageOffset,
    pub length: u32,
    /// would have dirty data.
    pub free_page_keys: [u32; 1022],
}
impl FreePageStackNode {
    pub fn pop(&mut self) -> PageOffset {
        assert_ne!(self.length, 0);
        self.length -= 1;
        let offset = self.free_page_keys[self.length as usize];
        assert_ne!(offset, 0);
        PageOffset::new(offset)
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
impl AsSlice for FreePageStackNode {}

const INTERNAL_NODE_KEY_LEN: usize = 203;

#[repr(C, align(64))]
#[derive(Clone)]
/// right skeye child's key is greater or equal to key.
pub(crate) struct InternalNode {
    leaf_type: u8,
    _padding: [u8; 3],
    key_count: u32,
    keys: [Key; INTERNAL_NODE_KEY_LEN],
    child_offsets: [PageOffset; INTERNAL_NODE_KEY_LEN + 1],
    _padding_1: u32,
}
impl AsSlice for InternalNode {}
impl Debug for InternalNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InternalNode")
            .field("leaf_type", &self.leaf_type)
            .field("key_count", &self.key_count)
            .field("keys", &&self.keys[..self.key_count as usize])
            .field(
                "child_offsets",
                &&self.child_offsets[..self.key_count as usize + 1],
            )
            .finish()
    }
}

impl InternalNode {
    pub fn new(keys: &[Key], child_offsets: &[PageOffset]) -> Self {
        assert!(!keys.is_empty());
        assert_eq!(keys.len() + 1, child_offsets.len());

        Self {
            leaf_type: 0,
            _padding: [0; 3],
            key_count: keys.len() as u32,
            keys: {
                let mut new_keys = [0; INTERNAL_NODE_KEY_LEN];
                new_keys[..keys.len()].copy_from_slice(keys);
                new_keys
            },
            child_offsets: {
                let mut new_offsets = [PageOffset::NULL; INTERNAL_NODE_KEY_LEN + 1];
                new_offsets[..child_offsets.len()].copy_from_slice(child_offsets);
                new_offsets
            },
            _padding_1: 0,
        }
    }
    fn key_index(&self, key: Key) -> usize {
        self.keys
            .into_iter()
            .take(self.key_count as usize)
            .enumerate()
            .find(|(_, key_)| key < *key_)
            .map(|(i, _)| i)
            .unwrap_or(self.key_count as usize)
    }
    pub fn find_child_offset_for(&self, key: Key) -> PageOffset {
        self.child_offsets[self.key_index(key)]
    }
    pub fn is_full(&self) -> bool {
        self.key_count == self.keys.len() as u32
    }
    pub fn insert(
        &mut self,
        key: Key,
        right_skeye_child_offset: PageOffset,
    ) -> Option<(InternalNode, Key)> {
        let key_index = self.key_index(key);

        if !self.is_full() {
            if key_index < self.key_count as usize {
                self.keys
                    .copy_within(key_index..self.key_count as usize, key_index + 1);
                self.child_offsets
                    .copy_within(key_index + 1..self.key_count as usize + 1, key_index + 2);
            }
            self.keys[key_index] = key;
            self.child_offsets[key_index + 1] = right_skeye_child_offset;
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
            offsets[key_index + 1] = right_skeye_child_offset;
            offsets[key_index + 2..].copy_from_slice(&self.child_offsets[key_index + 1..]);
            offsets
        };

        let right_key_count = one_plus_keys.len() / 2;
        let left_key_count = one_plus_keys.len() - right_key_count - 1;
        assert_eq!(left_key_count + right_key_count, INTERNAL_NODE_KEY_LEN);
        let center_key_index = left_key_count;

        /*
            Before:
            |   key 0   |   key 1   |   key 2   |   key 3   |
        | offset 0 | offset 1 | offset 2 | offset 3 | offset 4 |

            After:

                            |   key 2   |
                        | offset ↙ | offset ↘ |


            |   key 0   |   key 1   |              |   key 3   |
        | offset 0 | offset 1 | offset 2 |   | offset 3 | offset 4 |
        */

        let right_node = InternalNode::new(
            &one_plus_keys[center_key_index + 1..],
            &one_plus_child_offsets[center_key_index + 1..],
        );

        *self = InternalNode::new(
            &one_plus_keys[..center_key_index],
            &one_plus_child_offsets[..center_key_index + 1],
        );

        let center_key = one_plus_keys[center_key_index];

        Some((right_node, center_key))
    }
}

const LEAF_NODE_KEY_LEN: usize = 194;

#[repr(C, align(64))]
#[derive(Clone)]
pub(crate) struct LeafNode {
    leaf_type: u8,
    _padding: [u8; 3],
    key_count: u32,
    left_node_offset: PageOffset,
    right_node_offset: PageOffset,
    keys: [Key; LEAF_NODE_KEY_LEN],
    record_page_offsets: [PageOffset; LEAF_NODE_KEY_LEN],
    /// This size makes the limit of data size to 4KB * 2^8 = 1MB.
    record_page_count: [u8; LEAF_NODE_KEY_LEN],
}
impl AsSlice for LeafNode {}

impl LeafNode {
    pub fn new(left_node_offset: PageOffset, right_node_offset: PageOffset) -> Self {
        Self {
            leaf_type: 1,
            _padding: [0; 3],
            key_count: 0,
            left_node_offset,
            right_node_offset,
            keys: [0; LEAF_NODE_KEY_LEN],
            record_page_offsets: [PageOffset::NULL; LEAF_NODE_KEY_LEN],
            record_page_count: [0; LEAF_NODE_KEY_LEN],
        }
    }

    pub fn is_full(&self) -> bool {
        self.key_count == self.keys.len() as u32
    }

    fn index_to_insert(&self, key: Key) -> usize {
        self.keys
            .into_iter()
            .take(self.key_count as usize)
            .enumerate()
            .find(|(_, key_)| key < *key_)
            .map(|(i, _)| i)
            .unwrap_or(self.key_count as usize)
    }

    /// WARNING: Call this method only if the leaf node is **NOT FULL**.
    pub fn insert(&mut self, key: Key, record_page_range: PageRange) {
        assert!(!self.is_full());

        let index = self.index_to_insert(key);

        if index < self.key_count as usize {
            self.keys
                .copy_within(index..self.key_count as usize, index + 1);
            self.record_page_offsets
                .copy_within(index..self.key_count as usize, index + 1);
            self.record_page_count
                .copy_within(index..self.key_count as usize, index + 1);
        }
        self.keys[index] = key;
        self.record_page_offsets[index] = record_page_range.page_offset;
        self.record_page_count[index] = record_page_range.page_count;
        self.key_count += 1;
    }

    /// WARNING: Call this method only if the leaf node is **FULL".
    /// Return new splitted leaf node and new key if it's full.
    /// New leaf node will have half of the keys, bigger values.
    pub fn split_and_insert(
        &mut self,
        key: Key,
        record_page: PageRange,
        this_node_offset: PageOffset,
        right_node_offset: PageOffset,
    ) -> (LeafNode, Key) {
        assert!(self.is_full());
        let index = self.index_to_insert(key);

        /*
            Before:
            |   key 0   |   key 1   |   key 2   |   key 3   |

            After:
                (left)                        (right)
            |   key 0   |   key 1   |           |   key 2   |   key 3   |
        */

        let one_plus_keys = {
            let mut one_plus_keys = [0; LEAF_NODE_KEY_LEN + 1];
            one_plus_keys[..index].copy_from_slice(&self.keys[..index]);
            one_plus_keys[index] = key;
            one_plus_keys[index + 1..].copy_from_slice(&self.keys[index..]);
            one_plus_keys
        };

        let one_plus_record_page_offsets = {
            let mut one_plus_record_page_offsets = [PageOffset::NULL; LEAF_NODE_KEY_LEN + 1];
            one_plus_record_page_offsets[..index]
                .copy_from_slice(&self.record_page_offsets[..index]);
            one_plus_record_page_offsets[index] = record_page.page_offset;
            one_plus_record_page_offsets[index + 1..]
                .copy_from_slice(&self.record_page_offsets[index..]);
            one_plus_record_page_offsets
        };

        let one_plus_record_page_count = {
            let mut one_plus_record_page_count = [0; LEAF_NODE_KEY_LEN + 1];
            one_plus_record_page_count[..index].copy_from_slice(&self.record_page_count[..index]);
            one_plus_record_page_count[index] = record_page.page_count;
            one_plus_record_page_count[index + 1..]
                .copy_from_slice(&self.record_page_count[index..]);
            one_plus_record_page_count
        };

        let right_count = one_plus_keys.len() / 2;
        let left_count = one_plus_keys.len() - right_count;

        self.keys[..left_count].copy_from_slice(&one_plus_keys[..left_count]);
        self.record_page_offsets[..left_count]
            .copy_from_slice(&one_plus_record_page_offsets[..left_count]);
        self.record_page_count[..left_count]
            .copy_from_slice(&one_plus_record_page_count[..left_count]);
        self.key_count = left_count as u32;

        let mut right_leaf_node = LeafNode::new(this_node_offset, self.right_node_offset);

        right_leaf_node.keys[..right_count].copy_from_slice(&one_plus_keys[left_count..]);
        right_leaf_node.record_page_offsets[..right_count]
            .copy_from_slice(&one_plus_record_page_offsets[left_count..]);
        right_leaf_node.record_page_count[..right_count]
            .copy_from_slice(&one_plus_record_page_count[left_count..]);
        right_leaf_node.key_count = right_count as u32;

        self.right_node_offset = right_node_offset;

        let center_key = right_leaf_node.keys[0];

        (right_leaf_node, center_key)
    }

    pub fn contains(&self, key: u128) -> bool {
        self.keys
            .into_iter()
            .take(self.key_count as usize)
            .any(|key_| key_ == key)
    }

    /// # Panics
    ///
    /// Panics if key is not in the leaf node.
    pub fn delete(&mut self, key: u128) {
        let index = self
            .keys
            .into_iter()
            .take(self.key_count as usize)
            .enumerate()
            .find(|(_, key_key)| key == *key_key)
            .map(|(i, _)| i)
            .unwrap();

        if index + 1 < self.key_count as usize {
            self.keys
                .copy_within(index + 1..self.key_count as usize, index);
        }

        self.key_count -= 1;
    }

    pub fn keys(&self) -> &[Key] {
        &self.keys[..self.key_count as usize]
    }

    pub fn next(&self, exclusive_start_key: Option<Key>) -> NextResult {
        let start_index = exclusive_start_key
            .map(|key| {
                self.keys
                    .iter()
                    .take(self.key_count as usize)
                    .position(|&key_| key < key_)
                    .unwrap_or(self.key_count as usize)
            })
            .unwrap_or_default();

        if start_index == self.key_count as usize {
            if let Some(right_node_offset) = self.right_node_offset() {
                return NextResult::CheckRightNode { right_node_offset };
            } else {
                return NextResult::NoMoreEntries;
            }
        }

        let mut key_ranges = Vec::with_capacity(self.key_count as usize - start_index);
        for index in start_index..self.key_count as usize {
            key_ranges.push((
                self.keys[index],
                PageRange::new(
                    self.record_page_offsets[index],
                    self.record_page_count[index],
                ),
            ));
        }
        NextResult::Found { key_ranges }
    }

    pub fn right_node_offset(&self) -> Option<PageOffset> {
        if self.right_node_offset.is_null() {
            None
        } else {
            Some(self.right_node_offset)
        }
    }

    pub fn get_record_page_range(&self, key: Key) -> Option<PageRange> {
        let index = self
            .keys
            .iter()
            .take(self.key_count as usize)
            .position(|&key_| key == key_)?;

        Some(PageRange::new(
            self.record_page_offsets[index],
            self.record_page_count[index],
        ))
    }
}

impl Debug for LeafNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeafNode")
            .field("leaf_type", &self.leaf_type)
            .field("key_count", &self.key_count)
            .field("keys", &self.keys())
            .finish()
    }
}

pub(crate) enum NextResult {
    Found { key_ranges: Vec<(Key, PageRange)> },
    NoMoreEntries,
    CheckRightNode { right_node_offset: PageOffset },
}

#[repr(C, align(64))]
#[derive(Clone)]
pub(crate) struct Node {
    leaf_type: u8,
    _padding: [u8; 4095],
}
impl AsSlice for Node {}
impl Node {
    pub fn is_leaf(&self) -> bool {
        self.leaf_type != 0
    }
    pub fn as_internal_node(&self) -> Option<&InternalNode> {
        if !self.is_leaf() {
            Some(unsafe { std::mem::transmute::<&Node, &InternalNode>(self) })
        } else {
            None
        }
    }
    pub fn as_leaf_node(&self) -> Option<&LeafNode> {
        if self.is_leaf() {
            Some(unsafe { std::mem::transmute::<&Node, &LeafNode>(self) })
        } else {
            None
        }
    }
    pub fn as_one_of(&self) -> NodeMatchRef {
        if self.is_leaf() {
            NodeMatchRef::Leaf {
                leaf_node: unsafe { std::mem::transmute::<&Node, &LeafNode>(self) },
            }
        } else {
            NodeMatchRef::Internal {
                internal_node: unsafe { std::mem::transmute::<&Node, &InternalNode>(self) },
            }
        }
    }
}
pub(crate) enum NodeMatchRef<'a> {
    Internal { internal_node: &'a InternalNode },
    Leaf { leaf_node: &'a LeafNode },
}
impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_leaf() {
            self.as_leaf_node().fmt(f)
        } else {
            self.as_internal_node().fmt(f)
        }
    }
}

#[repr(C, align(64))]
#[derive(Debug, Clone)]
pub(crate) struct Page {
    data: [u8; PAGE_LEN],
}
impl AsSlice for Page {}
impl Page {
    pub fn empty() -> Self {
        Self {
            data: [0; PAGE_LEN],
        }
    }

    pub fn as_header(&self) -> &Header {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_free_page_stack_node_mut(&mut self) -> &mut FreePageStackNode {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_leaf_node_mut(&mut self) -> &mut LeafNode {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_leaf_node(&self) -> &LeafNode {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_internal_node_mut(&mut self) -> &mut InternalNode {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_node(&self) -> &Node {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_header_mut(&mut self) -> &mut Header {
        unsafe { std::mem::transmute(self) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PageRange {
    page_offset: PageOffset,
    page_count: u8,
}

impl PageRange {
    pub const HEADER: Self = Self {
        page_offset: PageOffset::HEADER,
        page_count: 1,
    };
    fn new(page_offset: PageOffset, page_count: u8) -> Self {
        Self {
            page_offset,
            page_count,
        }
    }

    pub fn page(page_offset: PageOffset) -> PageRange {
        Self::new(page_offset, 1)
    }

    pub fn data(page_offset: PageOffset, page_count: u8) -> PageRange {
        Self::new(page_offset, page_count)
    }

    pub fn byte_len(&self) -> usize {
        self.page_count as usize * PAGE_LEN
    }

    pub fn file_offset(&self) -> usize {
        self.page_offset.file_offset()
    }

    pub fn page_offset(&self) -> PageOffset {
        self.page_offset
    }

    pub fn page_count(&self) -> u8 {
        self.page_count
    }
}

pub(crate) enum PageBlockRef<'a> {
    Owned { page_block: &'a PageBlock },
    Shared { page_block: &'a PageBlockShared },
}

impl<'a> PageBlockRef<'a> {
    pub fn into_page(self) -> &'a Page {
        match self {
            Self::Owned { page_block } => page_block.as_page(),
            Self::Shared { page_block } => page_block.as_page(),
        }
    }

    pub fn into_record(self) -> Bytes {
        match self {
            Self::Owned { page_block } => page_block.as_record(),
            Self::Shared { page_block } => page_block.as_record(),
        }
    }
}

impl<'a> From<&'a PageBlock> for PageBlockRef<'a> {
    fn from(page_block: &'a PageBlock) -> Self {
        Self::Owned { page_block }
    }
}
impl<'a> From<&'a PageBlockShared> for PageBlockRef<'a> {
    fn from(page_block: &'a PageBlockShared) -> Self {
        Self::Shared { page_block }
    }
}

/// Page Block = contiguous pages
#[derive(Debug, Clone)]
pub(crate) struct PageBlock {
    page_count: u8,
    bytes: Vec<u8>,
}

impl PageBlock {
    pub fn new(page_count: u8) -> Self {
        Self {
            page_count,
            bytes: unsafe {
                let mut bytes = Vec::with_capacity(page_count as usize * PAGE_LEN);
                bytes.spare_capacity_mut();
                bytes.set_len(page_count as usize * PAGE_LEN);
                bytes
            },
        }
    }

    pub fn from_page(page: Page) -> PageBlock {
        Self {
            page_count: 1,
            bytes: page.as_slice().to_vec(),
        }
    }

    pub fn from_vec(buf: Vec<u8>, page_count: u8) -> PageBlock {
        Self {
            page_count,
            bytes: buf,
        }
    }

    pub fn as_page_mut(&mut self) -> &mut Page {
        unsafe { &mut *(self.bytes.as_mut_ptr() as *mut _) }
    }

    pub fn record(value: Bytes) -> Self {
        let record_byte_len = value.len() + size_of::<u32>();
        let page_count = (record_byte_len).div_ceil(PAGE_LEN);
        let mut page_block = Self::new(page_count as u8);

        page_block.bytes[0..4].copy_from_slice(&(value.len() as u32).to_le_bytes());
        page_block.bytes[4..(4 + value.len())].copy_from_slice(value.as_ref());

        page_block
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.page_count as usize * PAGE_LEN]
    }

    pub fn page_count(&self) -> u8 {
        self.page_count
    }

    fn as_page(&self) -> &Page {
        unsafe { &*(self.bytes.as_ptr() as *const _) }
    }

    fn as_record(&self) -> Bytes {
        let mut bytes = Bytes::from(self.bytes.clone());
        let byte_len = bytes.get_u32_le();
        bytes.slice(..byte_len as usize)
    }
}

impl From<PageBlock> for PageBlockShared {
    fn from(val: PageBlock) -> Self {
        PageBlockShared {
            page_count: val.page_count,
            bytes: Bytes::from(val.bytes),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PageBlockShared {
    page_count: u8,
    bytes: Bytes,
}

impl PageBlockShared {
    pub fn as_header(&self) -> &Header {
        unsafe { &*(self.as_slice().as_ptr() as *const _) }
    }

    pub fn as_node(&self) -> &Node {
        unsafe { &*(self.as_slice().as_ptr() as *const _) }
    }

    pub fn as_record(&self) -> Bytes {
        let mut bytes = self.bytes.clone();
        let byte_len = bytes.get_u32_le();
        bytes.slice(..byte_len as usize)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes[..self.page_count as usize * PAGE_LEN].as_ref()
    }

    fn as_page(&self) -> &Page {
        unsafe { &*(self.as_slice().as_ptr() as *const _) }
    }
}

impl From<PageBlockShared> for PageBlock {
    fn from(val: PageBlockShared) -> Self {
        PageBlock {
            page_count: val.page_count,
            bytes: val.bytes.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(std::mem::size_of::<Header>(), PAGE_LEN);
        assert_eq!(std::mem::size_of::<FreePageStackNode>(), PAGE_LEN);
        assert_eq!(std::mem::size_of::<InternalNode>(), PAGE_LEN);
        assert_eq!(std::mem::size_of::<LeafNode>(), PAGE_LEN);
        assert_eq!(std::mem::size_of::<Node>(), PAGE_LEN);

        assert_eq!(
            Header::new(PageOffset::NULL, PageOffset::NULL, PageOffset::NULL)
                .as_slice()
                .len(),
            PAGE_LEN
        );
        assert_eq!(
            FreePageStackNode {
                next_page_offset: PageOffset::NULL,
                length: 0,
                free_page_keys: [0; 1022]
            }
            .as_slice()
            .len(),
            PAGE_LEN
        );
        assert_eq!(
            InternalNode::new(
                &[0; INTERNAL_NODE_KEY_LEN],
                &[PageOffset::NULL; INTERNAL_NODE_KEY_LEN + 1]
            )
            .as_slice()
            .len(),
            PAGE_LEN
        );
        assert_eq!(
            LeafNode::new(PageOffset::NULL, PageOffset::NULL)
                .as_slice()
                .len(),
            PAGE_LEN
        );
        assert_eq!(
            Node {
                leaf_type: 0,
                _padding: [0; 4095]
            }
            .as_slice()
            .len(),
            PAGE_LEN
        );
    }

    #[test]
    fn leaf_node_move_half() {
        let mut inserted_keys = Vec::new();
        let mut leaf_node = LeafNode::new(PageOffset::NULL, PageOffset::NULL);
        for i in (0..(LEAF_NODE_KEY_LEN * 2)).step_by(2) {
            leaf_node.insert(i as _, PageRange::new(PageOffset::NULL, 0));
            inserted_keys.push(i as _);
        }

        assert!(leaf_node.is_full());
        assert_eq!(leaf_node.key_count, LEAF_NODE_KEY_LEN as u32);

        let (new_leaf_node, key) = leaf_node.split_and_insert(
            3,
            PageRange::new(PageOffset::NULL, 0),
            PageOffset::NULL,
            PageOffset::NULL,
        );
        inserted_keys.push(3);

        assert_eq!(new_leaf_node.key_count, new_leaf_node.keys().len() as u32);
        assert_eq!(leaf_node.key_count, leaf_node.keys().len() as u32);

        assert_eq!(
            new_leaf_node.key_count,
            (LEAF_NODE_KEY_LEN as u32 + 1 - leaf_node.key_count)
        );
        assert_eq!(key, new_leaf_node.keys[0]);

        assert!(leaf_node.contains(3));

        leaf_node
            .keys()
            .iter()
            .zip(leaf_node.keys().iter().skip(1))
            .for_each(|(a, b)| assert!(a < b, "{:?} < {:?}", a, b));

        new_leaf_node
            .keys()
            .iter()
            .zip(
                new_leaf_node
                    .keys()
                    .iter()
                    .skip(1)
                    .take(new_leaf_node.key_count as usize),
            )
            .for_each(|(a, b)| assert!(a < b, "{:?} < {:?}", a, b));

        for key in inserted_keys {
            assert!(leaf_node.contains(key) || new_leaf_node.contains(key));
        }
    }

    #[test]
    fn internal_node_insert() {
        {
            let mut internal_node =
                InternalNode::new(&[3], &[PageOffset::new(1), PageOffset::new(2)]);
            assert!(internal_node.insert(1, PageOffset::new(3)).is_none());
        }
    }

    #[test]
    fn internal_node_insert_split() {
        let mut internal_node = InternalNode::new(&[1], &[PageOffset::new(0), PageOffset::new(1)]);
        for i in 1..INTERNAL_NODE_KEY_LEN {
            assert!(internal_node
                .insert(i as Key + 1, PageOffset::new(i as u32 + 1))
                .is_none());
        }
        let (right_node, center_key) = internal_node
            .insert(
                INTERNAL_NODE_KEY_LEN as Key + 1,
                PageOffset::new(INTERNAL_NODE_KEY_LEN as u32 + 2),
            )
            .unwrap();

        assert_eq!(center_key, ((INTERNAL_NODE_KEY_LEN + 1) / 2) as Key);

        assert_eq!(
            internal_node.key_count + right_node.key_count,
            INTERNAL_NODE_KEY_LEN as u32
        );

        for i in 0..internal_node.key_count as usize {
            assert_eq!(internal_node.keys[i], i as Key + 1);
            assert_eq!(internal_node.child_offsets[i].value, i as u32);
        }

        for i in 0..right_node.key_count as usize {
            assert_eq!(right_node.keys[i], i as Key + 1 + center_key as Key);
            assert_eq!(
                right_node.child_offsets[i].value,
                i as u32 + center_key as u32
            );
        }
    }
}
