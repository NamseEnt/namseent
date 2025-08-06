use super::*;
use bytes::{Buf, BufMut};
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

    pub fn decrease(&mut self) {
        self.value -= 1;
    }
}

pub(crate) trait Serialize {
    fn to_vec(&self) -> Vec<u8>;
}
pub(crate) trait Deserialize {
    fn from_slice(slice: &[u8]) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Header {
    /// Would be null
    pub free_stack_top_page_offset: PageOffset,
    /// Root node would be a leaf node or an internal node.
    pub root_node_offset: PageOffset,
    /// Use this value to allocate new page.
    pub next_page_offset: PageOffset,
}
impl Header {
    pub fn new(
        free_stack_top_page_offset: PageOffset,
        root_node_offset: PageOffset,
        next_page_offset: PageOffset,
    ) -> Self {
        Self {
            free_stack_top_page_offset,
            root_node_offset,
            next_page_offset,
        }
    }

    pub fn file_size(&self) -> usize {
        self.next_page_offset.file_offset()
    }
}
impl Serialize for Header {
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PAGE_LEN);

        bytes.put_u32_le(self.free_stack_top_page_offset.as_u32());
        bytes.put_u32_le(self.root_node_offset.as_u32());
        bytes.put_u32_le(self.next_page_offset.as_u32());

        bytes.put_bytes(0, PAGE_LEN - bytes.len());

        assert_eq!(bytes.len(), PAGE_LEN);
        bytes
    }
}
impl Deserialize for Header {
    fn from_slice(mut slice: &[u8]) -> Self {
        assert_eq!(slice.len(), PAGE_LEN);

        let free_stack_top_page_offset = PageOffset::new(slice.get_u32_le());
        let root_node_offset = PageOffset::new(slice.get_u32_le());
        let next_page_offset = PageOffset::new(slice.get_u32_le());
        Self {
            free_stack_top_page_offset,
            root_node_offset,
            next_page_offset,
        }
    }
}

const FREE_STACK_MAX_PAGE_RANGE_COUNT: usize = 818;

#[derive(Debug, Clone)]
pub(crate) struct FreeStackNode {
    pub next_page_offset: PageOffset,
    page_ranges: Vec<PageRange>,
}
impl FreeStackNode {
    pub fn new() -> Self {
        Self {
            next_page_offset: PageOffset::NULL,
            page_ranges: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.page_ranges.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.page_ranges.len() == FREE_STACK_MAX_PAGE_RANGE_COUNT
    }
    pub fn try_pop(&mut self, page_count: u8) -> Option<PageRange> {
        if self.page_ranges.is_empty() {
            return None;
        }
        for index in 0..self.page_ranges.len() {
            let page_range = &self.page_ranges[index];
            if page_count <= page_range.page_count {
                return Some(self.page_ranges.remove(index));
            }
        }
        None
    }
    pub fn push(&mut self, page_range: PageRange) {
        assert!(self.page_ranges.len() < FREE_STACK_MAX_PAGE_RANGE_COUNT);
        self.page_ranges.push(page_range);
    }
}
impl Serialize for FreeStackNode {
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PAGE_LEN);

        bytes.put_u32_le(self.next_page_offset.as_u32());
        bytes.put_u16_le(self.page_ranges.len() as u16);
        for key in &self.page_ranges {
            bytes.put_u32_le(key.page_offset.as_u32());
            bytes.put_u8(key.page_count);
        }
        bytes.put_bytes(0, PAGE_LEN - bytes.len());

        assert_eq!(bytes.len(), PAGE_LEN);
        bytes
    }
}
impl Deserialize for FreeStackNode {
    fn from_slice(mut slice: &[u8]) -> Self {
        assert_eq!(slice.len(), PAGE_LEN);

        let next_page_offset = PageOffset::new(slice.get_u32_le());
        let page_range_count = slice.get_u16_le() as usize;
        let mut page_ranges = Vec::with_capacity(page_range_count);
        for _ in 0..page_range_count {
            let page_offset = PageOffset::new(slice.get_u32_le());
            let page_count = slice.get_u8();
            page_ranges.push(PageRange::new(page_offset, page_count));
        }

        Self {
            next_page_offset,
            page_ranges,
        }
    }
}

const INTERNAL_NODE_KEY_LEN: usize = 204;

#[derive(Debug, Clone)]
/// right child's key is greater or equal to key.
pub(crate) struct InternalNode {
    keys: Vec<Key>,
    child_offsets: Vec<PageOffset>,
}
impl Serialize for InternalNode {
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PAGE_LEN);

        bytes.put_u8(NodeType::INTERNAL);

        bytes.put_u32_le(self.keys.len() as u32);

        if !self.keys.is_empty() {
            bytes.put_u32_le(self.child_offsets[0].as_u32());
        }

        for i in 0..self.keys.len() {
            bytes.put_u128_le(self.keys[i]);
            bytes.put_u32_le(self.child_offsets[i + 1].as_u32());
        }

        bytes.put_bytes(0, PAGE_LEN - bytes.len());

        assert_eq!(bytes.len(), PAGE_LEN);
        bytes
    }
}
impl Deserialize for InternalNode {
    fn from_slice(mut slice: &[u8]) -> Self {
        assert_eq!(slice.len(), PAGE_LEN);

        let node_type = slice.get_u8();
        assert_eq!(node_type, NodeType::INTERNAL);

        let key_count = slice.get_u32_le();
        let mut keys = Vec::with_capacity(key_count as usize);
        let mut child_offsets = Vec::with_capacity(key_count as usize + 1);
        if key_count > 0 {
            child_offsets.push(PageOffset::new(slice.get_u32_le()));
        }

        for _ in 0..key_count {
            keys.push(slice.get_u128_le());
            child_offsets.push(PageOffset::new(slice.get_u32_le()));
        }

        Self {
            keys,
            child_offsets,
        }
    }
}

impl InternalNode {
    pub fn new(keys: &[Key], child_offsets: &[PageOffset]) -> Self {
        assert!(!keys.is_empty());
        assert_eq!(keys.len() + 1, child_offsets.len());

        Self {
            keys: keys.to_vec(),
            child_offsets: child_offsets.to_vec(),
        }
    }
    fn key_index(&self, key: Key) -> usize {
        self.keys
            .iter()
            .position(|&key_| key < key_)
            .unwrap_or(self.keys.len())
    }
    pub fn find_child_offset_for(&self, key: Key) -> PageOffset {
        self.child_offsets[self.key_index(key)]
    }
    pub fn is_full(&self) -> bool {
        self.keys.len() == INTERNAL_NODE_KEY_LEN
    }
    pub fn insert(
        &mut self,
        key: Key,
        right_child_offset: PageOffset,
    ) -> Option<(InternalNode, Key)> {
        let index = self.key_index(key);
        let was_full = self.is_full();

        self.keys.insert(index, key);
        self.child_offsets.insert(index + 1, right_child_offset);

        if !was_full {
            return None;
        }

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

        let right_keys = self.keys.split_off(INTERNAL_NODE_KEY_LEN / 2);
        let center_key = self.keys.pop().unwrap();
        let right_child_offsets = self.child_offsets.split_off(INTERNAL_NODE_KEY_LEN / 2);

        let right_node = InternalNode::new(&right_keys, &right_child_offsets);

        Some((right_node, center_key))
    }
}

struct NodeType;
impl NodeType {
    const INTERNAL: u8 = 0;
    const LEAF: u8 = 1;
}

const LEAF_NODE_KEY_LEN: usize = 194;

#[derive(Debug, Clone)]
pub(crate) struct LeafNode {
    right_node_offset: PageOffset,
    entries: Vec<LeafNodeEntry>,
}
impl Serialize for LeafNode {
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PAGE_LEN);

        bytes.put_u8(NodeType::LEAF);
        bytes.put_u32_le(self.right_node_offset.as_u32());

        assert!(self.entries.len() <= u8::MAX as usize);
        bytes.put_u8(self.entries.len() as u8);

        for entry in &self.entries {
            bytes.put_u128_le(entry.key);
            bytes.put_u32_le(entry.record_page_range.page_offset.as_u32());
            bytes.put_u8(entry.record_page_range.page_count);
        }

        bytes.put_bytes(0, PAGE_LEN - bytes.len());

        assert_eq!(bytes.len(), PAGE_LEN);
        bytes
    }
}
impl Deserialize for LeafNode {
    fn from_slice(mut slice: &[u8]) -> Self {
        assert_eq!(slice.len(), PAGE_LEN);

        let node_type = slice.get_u8();
        assert_eq!(node_type, NodeType::LEAF);

        let right_node_offset = PageOffset::new(slice.get_u32_le());

        let key_count = slice.get_u8();
        let mut entries = Vec::with_capacity(key_count as usize);

        for _ in 0..key_count {
            let key = slice.get_u128_le();
            let page_offset = PageOffset::new(slice.get_u32_le());
            let page_count = slice.get_u8();
            entries.push(LeafNodeEntry {
                key,
                record_page_range: PageRange::new(page_offset, page_count),
            });
        }

        Self {
            right_node_offset,
            entries,
        }
    }
}

impl LeafNode {
    pub fn new(right_node_offset: PageOffset) -> Self {
        Self {
            right_node_offset,
            entries: Vec::new(),
        }
    }

    fn new_with_entries(right_node_offset: PageOffset, entries: Vec<LeafNodeEntry>) -> LeafNode {
        assert!(entries.len() <= LEAF_NODE_KEY_LEN);
        Self {
            right_node_offset,
            entries,
        }
    }

    pub fn is_full(&self) -> bool {
        self.entries.len() == LEAF_NODE_KEY_LEN
    }

    fn index_to_insert(&self, key: Key) -> usize {
        self.keys()
            .position(|key_| key < key_)
            .unwrap_or(self.entries.len())
    }

    /// WARNING: Call this method only if the leaf node is **NOT FULL**.
    pub fn insert(&mut self, key: Key, record_page_range: PageRange) {
        assert!(!self.is_full());

        let index = self.index_to_insert(key);

        self.entries.insert(
            index,
            LeafNodeEntry {
                key,
                record_page_range,
            },
        );
    }

    /// WARNING: Call this method only if the leaf node is **FULL".
    /// Return new splitted leaf node and new key if it's full.
    /// New leaf node will have half of the keys, bigger values.
    pub fn split_and_insert(
        &mut self,
        key: Key,
        record_page_range: PageRange,
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

        self.entries.insert(
            index,
            LeafNodeEntry {
                key,
                record_page_range,
            },
        );

        let left_count = self.entries.len() / 2;
        let right_entries = self.entries.split_off(left_count);

        let right_leaf_node = LeafNode::new_with_entries(self.right_node_offset, right_entries);
        self.right_node_offset = right_node_offset;

        let center_key = right_leaf_node.keys().next().unwrap();

        (right_leaf_node, center_key)
    }

    pub fn contains(&self, key: u128) -> bool {
        self.keys().any(|key_| key == key_)
    }

    /// # Panics
    ///
    /// Panics if key is not in the leaf node.
    pub fn delete(&mut self, key: u128) -> LeafNodeEntry {
        let index = self.keys().position(|key_| key == key_).unwrap();
        self.entries.remove(index)
    }

    pub fn keys(&self) -> impl ExactSizeIterator<Item = Key> + '_ {
        self.entries.iter().map(|entry| entry.key)
    }

    pub fn next(&self, exclusive_start_key: Option<Key>) -> NextResult {
        let start_index = exclusive_start_key
            .map(|key| {
                self.keys()
                    .position(|key_| key < key_)
                    .unwrap_or(self.entries.len())
            })
            .unwrap_or_default();

        if start_index == self.entries.len() {
            if let Some(right_node_offset) = self.right_node_offset() {
                return NextResult::CheckRightNode { right_node_offset };
            } else {
                return NextResult::NoMoreEntries;
            }
        }

        let mut key_ranges = Vec::with_capacity(self.entries.len() - start_index);
        for index in start_index..self.entries.len() {
            let entry = &self.entries[index];
            key_ranges.push((entry.key, entry.record_page_range));
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
        let index = self.keys().position(|key_| key == key_)?;

        Some(self.entries[index].record_page_range)
    }
}

#[derive(Debug, Clone)]
pub struct LeafNodeEntry {
    pub key: Key,
    pub record_page_range: PageRange,
}

pub(crate) enum NextResult {
    Found { key_ranges: Vec<(Key, PageRange)> },
    NoMoreEntries,
    CheckRightNode { right_node_offset: PageOffset },
}

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Internal(InternalNode),
    Leaf(LeafNode),
}
impl Serialize for Node {
    fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Internal(internal_node) => internal_node.to_vec(),
            Self::Leaf(leaf_node) => leaf_node.to_vec(),
        }
    }
}
impl Deserialize for Node {
    fn from_slice(slice: &[u8]) -> Self {
        if slice[0] == 0 {
            Self::Internal(InternalNode::from_slice(slice))
        } else {
            Self::Leaf(LeafNode::from_slice(slice))
        }
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

#[derive(Debug, Clone)]
pub(crate) enum Page {
    Header(Header),
    FreeStackNode(FreeStackNode),
    InternalNode(InternalNode),
    LeafNode(LeafNode),
}

impl Page {
    pub fn as_header(&self) -> &Header {
        match self {
            Self::Header(header) => header,
            x => unreachable!("expect header but {:?}", x),
        }
    }
    pub fn as_header_mut(&mut self) -> &mut Header {
        match self {
            Self::Header(header) => header,
            x => unreachable!("expect header but {:?}", x),
        }
    }
    pub fn as_leaf_node(&self) -> &LeafNode {
        match self {
            Self::LeafNode(leaf_node) => leaf_node,
            x => unreachable!("expect leaf_node but {:?}", x),
        }
    }
    pub fn as_leaf_node_mut(&mut self) -> &mut LeafNode {
        match self {
            Self::LeafNode(leaf_node) => leaf_node,
            x => unreachable!("expect leaf_node but {:?}", x),
        }
    }
    pub fn as_internal_node_mut(&mut self) -> &mut InternalNode {
        match self {
            Self::InternalNode(internal_node) => internal_node,
            x => unreachable!("expect internal_node but {:?}", x),
        }
    }
    pub fn as_free_stack_mut(&mut self) -> &mut FreeStackNode {
        match self {
            Self::FreeStackNode(free_page_stack_node) => free_page_stack_node,
            x => unreachable!("expect free_stack but {:?}", x),
        }
    }
}

impl Serialize for Page {
    fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Header(header) => header.to_vec(),
            Self::FreeStackNode(free_page_stack_node) => free_page_stack_node.to_vec(),
            Self::InternalNode(internal_node) => internal_node.to_vec(),
            Self::LeafNode(leaf_node) => leaf_node.to_vec(),
        }
    }
}
impl Deserialize for Page {
    fn from_slice(slice: &[u8]) -> Self {
        if slice.len() == PAGE_LEN {
            Self::Header(Header::from_slice(slice))
        } else {
            match slice[0] {
                0 => Self::FreeStackNode(FreeStackNode::from_slice(slice)),
                1 => Self::LeafNode(LeafNode::from_slice(slice)),
                _ => Self::InternalNode(InternalNode::from_slice(slice)),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Record {
    page_count: u8,
    content: Bytes,
}

impl Record {
    pub fn new(content: Bytes) -> Self {
        let page_count = (content.len() + size_of::<u32>()).div_ceil(PAGE_LEN) as u8;

        Self {
            page_count,
            content,
        }
    }

    pub fn content(&self) -> Bytes {
        self.content.clone()
    }

    pub fn page_count(&self) -> u8 {
        self.page_count
    }
}

impl Serialize for Record {
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PAGE_LEN * self.page_count as usize);

        bytes.put_u32_le(self.content.len() as u32);
        bytes.put_slice(self.content.as_ref());
        bytes.put_bytes(0, PAGE_LEN - (self.content.len() + 4) % PAGE_LEN);

        assert_eq!(bytes.len(), self.page_count as usize * PAGE_LEN);

        bytes
    }
}
impl Deserialize for Record {
    fn from_slice(mut slice: &[u8]) -> Self {
        assert_eq!(slice.len() % PAGE_LEN, 0);

        let page_count = (slice.len() / PAGE_LEN) as u8;

        let content_len = slice.get_u32_le() as usize;
        let content = Bytes::from(slice[..content_len].to_vec());

        Self {
            page_count,
            content,
        }
    }
}

/// Page Block = contiguous pages
#[derive(Debug, Clone)]
pub(crate) enum PageBlock {
    Page(Page),
    Record(Record),
}

impl PageBlock {
    pub fn as_page_mut(&mut self) -> &mut Page {
        match self {
            Self::Page(page) => page,
            _ => panic!("Not a page"),
        }
    }

    pub fn as_page(&self) -> &Page {
        match self {
            Self::Page(page) => page,
            _ => panic!("Not a page"),
        }
    }

    pub fn as_record(&self) -> &Record {
        match self {
            Self::Record(record) => record,
            _ => panic!("Not a record"),
        }
    }
}

impl Serialize for PageBlock {
    fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Page(page) => page.to_vec(),
            Self::Record(record) => record.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(
            Header::new(PageOffset::NULL, PageOffset::NULL, PageOffset::NULL)
                .to_vec()
                .len(),
            PAGE_LEN
        );
        assert_eq!(FreeStackNode::new().to_vec().len(), PAGE_LEN);
        assert_eq!(
            InternalNode::new(
                &[0; INTERNAL_NODE_KEY_LEN],
                &[PageOffset::NULL; INTERNAL_NODE_KEY_LEN + 1]
            )
            .to_vec()
            .len(),
            PAGE_LEN
        );
        assert_eq!(LeafNode::new(PageOffset::NULL).to_vec().len(), PAGE_LEN);

        let mut node = LeafNode::new(PageOffset::NULL);
        for i in 0..LEAF_NODE_KEY_LEN {
            node.insert(i as _, PageRange::new(PageOffset::NULL, 0));
        }
        assert_eq!(node.to_vec().len(), PAGE_LEN);

        let mut node = FreeStackNode::new();
        for _ in 0..FREE_STACK_MAX_PAGE_RANGE_COUNT {
            node.push(PageRange::new(PageOffset::NULL, 0));
        }
        assert_eq!(node.to_vec().len(), PAGE_LEN);
    }

    #[test]
    fn leaf_node_move_half() {
        let mut inserted_keys = Vec::new();
        let mut leaf_node = LeafNode::new(PageOffset::NULL);
        for i in (0..(LEAF_NODE_KEY_LEN * 2)).step_by(2) {
            leaf_node.insert(i as _, PageRange::new(PageOffset::NULL, 0));
            inserted_keys.push(i as _);
        }

        assert!(leaf_node.is_full());
        assert_eq!(leaf_node.entries.len(), LEAF_NODE_KEY_LEN);

        let (new_leaf_node, key) =
            leaf_node.split_and_insert(3, PageRange::new(PageOffset::NULL, 0), PageOffset::NULL);
        inserted_keys.push(3);

        assert_eq!(new_leaf_node.entries.len(), new_leaf_node.keys().len());
        assert_eq!(leaf_node.entries.len(), leaf_node.keys().len());

        assert_eq!(
            new_leaf_node.entries.len(),
            (LEAF_NODE_KEY_LEN + 1 - leaf_node.entries.len())
        );
        assert_eq!(key, new_leaf_node.keys().next().unwrap());

        assert!(leaf_node.contains(3));

        leaf_node
            .keys()
            .zip(leaf_node.keys().skip(1))
            .for_each(|(a, b)| assert!(a < b, "{a:?} < {b:?}"));

        new_leaf_node
            .keys()
            .zip(new_leaf_node.keys().skip(1))
            .for_each(|(a, b)| assert!(a < b, "{a:?} < {b:?}"));

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
            assert!(
                internal_node
                    .insert(i as Key + 1, PageOffset::new(i as u32 + 1))
                    .is_none()
            );
        }
        let (right_node, center_key) = internal_node
            .insert(
                INTERNAL_NODE_KEY_LEN as Key + 1,
                PageOffset::new(INTERNAL_NODE_KEY_LEN as u32 + 2),
            )
            .unwrap();

        assert_eq!(
            internal_node.keys.len() + right_node.keys.len(),
            INTERNAL_NODE_KEY_LEN
        );

        for i in 0..internal_node.keys.len() {
            assert_eq!(internal_node.keys[i], i as Key + 1);
            assert_eq!(internal_node.child_offsets[i].value, i as u32);
        }

        for i in 0..right_node.keys.len() {
            assert_eq!(right_node.keys[i], i as Key + 1 + center_key as Key);
            assert_eq!(
                right_node.child_offsets[i].value,
                i as u32 + center_key as u32
            );
        }
    }
}
