use super::*;
use std::{fmt::Debug, io::SeekFrom};

#[repr(C, align(1))]
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

    pub fn file_pos(&self) -> SeekFrom {
        SeekFrom::Start(self.value as u64 * 4096)
    }

    pub fn file_offset(&self) -> usize {
        self.value as usize * 4096
    }

    pub fn fetch_increase(&mut self) -> Self {
        let next = *self;
        self.value += 1;
        next
    }

    pub fn is_null(&self) -> bool {
        self == &Self::NULL
    }
}

pub(crate) trait AsSlice: Sized {
    fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self as *const _ as *const u8, std::mem::size_of::<Self>())
        }
    }
}

#[repr(C, align(1))]
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
    pub(crate) fn new(
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

#[repr(C, align(1))]
pub(crate) struct FreePageStackNode {
    pub next_page_offset: PageOffset,
    pub length: u32,
    /// would have dirty data.
    pub free_page_ids: [u32; 1022],
}
impl FreePageStackNode {
    pub fn pop(&mut self) -> PageOffset {
        assert_ne!(self.length, 0);
        self.length -= 1;
        let offset = self.free_page_ids[self.length as usize];
        assert_ne!(offset, 0);
        PageOffset::new(offset)
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
impl AsSlice for FreePageStackNode {}

const INTERNAL_NODE_ID_LEN: usize = 203;

#[repr(C, align(1))]
#[derive(Debug, Clone)]
/// right side child's id is greater or equal to id.
pub(crate) struct InternalNode {
    leaf_type: u8,
    _padding: [u8; 3],
    id_count: u32,
    ids: [Id; INTERNAL_NODE_ID_LEN],
    child_offsets: [PageOffset; INTERNAL_NODE_ID_LEN + 1],
    _padding_1: u32,
}
impl AsSlice for InternalNode {}

impl InternalNode {
    pub fn new(
        id: Id,
        left_side_child_offset: PageOffset,
        right_side_child_offset: PageOffset,
    ) -> Self {
        let mut ids = [0; INTERNAL_NODE_ID_LEN];
        ids[0] = id;

        let mut child_offsets = [PageOffset::NULL; INTERNAL_NODE_ID_LEN + 1];
        child_offsets[0] = left_side_child_offset;
        child_offsets[1] = right_side_child_offset;

        Self {
            leaf_type: 0,
            _padding: [0; 3],
            id_count: 1,
            ids,
            child_offsets,
            _padding_1: 0,
        }
    }
    fn id_index(&self, id: Id) -> Option<usize> {
        self.ids
            .into_iter()
            .take(self.id_count as usize)
            .enumerate()
            .find(|(_, id_)| id < *id_)
            .map(|(i, _)| i)
    }
    pub fn find_child_offset_for(&self, id: Id) -> PageOffset {
        self.id_index(id)
            .map(|i| self.child_offsets[i])
            .unwrap_or(self.child_offsets[self.id_count as usize])
    }
    pub fn is_full(&self) -> bool {
        self.id_count == self.ids.len() as u32
    }
    pub fn insert(
        &mut self,
        id: Id,
        right_side_child_offset: PageOffset,
    ) -> Option<(InternalNode, Id)> {
        let id_index = self.id_index(id).unwrap_or(self.id_count as usize);

        if !self.is_full() {
            if id_index < self.id_count as usize {
                self.ids[id_index..].rotate_right(1);
                self.child_offsets[id_index + 1..].rotate_right(1);
            }
            self.ids[id_index] = id;
            self.child_offsets[id_index + 1] = right_side_child_offset;
            self.id_count += 1;
            return None;
        }

        let one_plus_ids = {
            let mut ids = [0; INTERNAL_NODE_ID_LEN + 1];
            ids[..id_index].copy_from_slice(&self.ids[..id_index]);
            ids[id_index] = id;
            ids[id_index + 1..].copy_from_slice(&self.ids[id_index..]);
            ids
        };
        let one_plus_child_offsets = {
            let mut offsets = [PageOffset::NULL; INTERNAL_NODE_ID_LEN + 2];
            offsets[..id_index + 1].copy_from_slice(&self.child_offsets[..id_index + 1]);
            offsets[id_index + 1] = right_side_child_offset;
            offsets[id_index + 2..].copy_from_slice(&self.child_offsets[id_index + 1..]);
            offsets
        };

        let right_id_count = one_plus_ids.len() / 2;
        let left_id_count = one_plus_ids.len() - right_id_count - 1;
        assert_eq!(left_id_count + right_id_count + 1, one_plus_ids.len());
        let center_id_index = left_id_count;

        /*
            Before:
            |   id 0   |   id 1   |   id 2   |   id 3   |
        | offset 0 | offset 1 | offset 2 | offset 3 | offset 4 |

            After:

                            |   id 2   |
                        | offset ↙ | offset ↘ |


            |   id 0   |   id 1   |              |   id 3   |
        | offset 0 | offset 1 | offset 2 |   | offset 3 | offset 4 |
        */

        let mut right_node = unsafe { std::mem::zeroed::<InternalNode>() };
        right_node.id_count = right_id_count as u32;
        right_node.ids[..right_id_count].copy_from_slice(&one_plus_ids[center_id_index + 1..]);
        right_node.child_offsets[..right_id_count + 1]
            .copy_from_slice(&one_plus_child_offsets[center_id_index + 1..]);

        self.id_count = left_id_count as u32;
        self.ids[..left_id_count].copy_from_slice(&one_plus_ids[..center_id_index]);
        self.child_offsets[..left_id_count + 1]
            .copy_from_slice(&one_plus_child_offsets[..center_id_index + 1]);

        let center_id = one_plus_ids[center_id_index];

        Some((right_node, center_id))
    }
}

const LEAF_NODE_IDS_LEN: usize = 255;

#[repr(C, align(1))]
#[derive(Clone)]
pub(crate) struct LeafNode {
    leaf_type: u8,
    _padding: [u8; 3],
    id_count: u32,
    ids: [Id; LEAF_NODE_IDS_LEN],
}
impl AsSlice for LeafNode {}

impl LeafNode {
    pub fn new() -> Self {
        Self {
            leaf_type: 1,
            _padding: [0; 3],
            id_count: 0,
            ids: [0; LEAF_NODE_IDS_LEN],
        }
    }

    pub fn is_full(&self) -> bool {
        self.id_count == self.ids.len() as u32
    }

    /// Return new splitted leaf node and new id if it's full.
    /// New leaf node will have half of the ids, bigger values.
    pub fn insert(&mut self, id: Id) -> Option<(LeafNode, Id)> {
        let index = self
            .ids
            .into_iter()
            .take(self.id_count as usize)
            .enumerate()
            .find(|(_, id_)| id < *id_)
            .map(|(i, _)| i)
            .unwrap_or(self.id_count as usize);

        if !self.is_full() {
            if index + 1 < self.id_count as usize {
                self.ids
                    .copy_within(index..self.id_count as usize, index + 1);
            }
            self.ids[index] = id;
            self.id_count += 1;
            return None;
        }

        /*
            Before:
            |   id 0   |   id 1   |   id 2   |   id 3   |

            After:
                (left)                        (right)
            |   id 0   |   id 1   |           |   id 2   |   id 3   |
        */

        let one_plus_ids = {
            let mut one_plus_ids = [0; LEAF_NODE_IDS_LEN + 1];
            one_plus_ids[..index].copy_from_slice(&self.ids[..index]);
            one_plus_ids[index] = id;
            one_plus_ids[index + 1..].copy_from_slice(&self.ids[index..]);
            one_plus_ids
        };

        let right_count = one_plus_ids.len() / 2;
        let left_count = one_plus_ids.len() - right_count;

        self.ids[..left_count].copy_from_slice(&one_plus_ids[..left_count]);
        self.id_count = left_count as u32;

        let mut new_leaf_node = LeafNode::new();

        new_leaf_node.ids[..right_count].copy_from_slice(&one_plus_ids[left_count..]);
        new_leaf_node.id_count = right_count as u32;

        let center_id = new_leaf_node.ids[0];

        Some((new_leaf_node, center_id))
    }

    pub fn contains(&self, id: u128) -> bool {
        self.ids
            .into_iter()
            .take(self.id_count as usize)
            .any(|key| key == id)
    }

    /// # Panics
    ///
    /// Panics if id is not in the leaf node.
    pub fn delete(&mut self, id: u128) {
        let index = self
            .ids
            .into_iter()
            .take(self.id_count as usize)
            .enumerate()
            .find(|(_, key_id)| id == *key_id)
            .map(|(i, _)| i)
            .unwrap();

        if index + 1 < self.id_count as usize {
            self.ids
                .copy_within(index + 1..self.id_count as usize, index);
        }

        self.id_count -= 1;
    }

    pub fn ids(&self) -> &[Id] {
        &self.ids[..self.id_count as usize]
    }
}

impl Debug for LeafNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeafNode")
            .field("leaf_type", &self.leaf_type)
            .field("id_count", &self.id_count)
            .field("ids", &self.ids())
            .finish()
    }
}

#[repr(C, align(1))]
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
    pub(crate) fn as_internal_node(&self) -> &InternalNode {
        unsafe { std::mem::transmute(self) }
    }
    pub(crate) fn as_leaf_node(&self) -> &LeafNode {
        unsafe { std::mem::transmute(self) }
    }
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

#[repr(C, align(1))]
#[derive(Debug, Clone)]
pub(crate) struct Page {
    data: [u8; 4096],
}
impl AsSlice for Page {}
impl Page {
    pub fn new() -> Self {
        Self { data: [0; 4096] }
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

    pub(crate) fn as_node(&self) -> &Node {
        unsafe { std::mem::transmute(self) }
    }

    pub(crate) fn as_header_mut(&mut self) -> &mut Header {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
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

        let (new_leaf_node, id) = leaf_node.insert(3).unwrap();

        assert_eq!(leaf_node.id_count, 128);
        assert_eq!(new_leaf_node.id_count, 128);
        assert_eq!(id, 254);
        assert_eq!(id, new_leaf_node.ids[0]);

        let leaf_node_ids = {
            let mut ids = (0..=253).step_by(2).collect::<Vec<_>>();
            ids.push(3);
            ids.sort();
            ids
        };
        assert_eq!(leaf_node.ids[..128], leaf_node_ids);

        let new_leaf_node_ids = (254..510).step_by(2).collect::<Vec<_>>();
        assert_eq!(new_leaf_node.ids[..128], new_leaf_node_ids);
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
        for i in 1..INTERNAL_NODE_ID_LEN {
            assert!(internal_node
                .insert(i as Id + 1, PageOffset::new(i as u32 + 1))
                .is_none());
        }
        let (right_node, center_id) = internal_node
            .insert(
                INTERNAL_NODE_ID_LEN as Id + 1,
                PageOffset::new(INTERNAL_NODE_ID_LEN as u32 + 2),
            )
            .unwrap();

        assert_eq!(center_id, ((INTERNAL_NODE_ID_LEN + 1) / 2) as Id);

        assert_eq!(
            internal_node.id_count + right_node.id_count,
            INTERNAL_NODE_ID_LEN as u32
        );

        for i in 0..internal_node.id_count as usize {
            assert_eq!(internal_node.ids[i], i as Id + 1);
            assert_eq!(internal_node.child_offsets[i].value, i as u32);
        }

        for i in 0..right_node.id_count as usize {
            assert_eq!(right_node.ids[i], i as Id + 1 + center_id as Id);
            assert_eq!(
                right_node.child_offsets[i].value,
                i as u32 + center_id as u32
            );
        }
    }
}
