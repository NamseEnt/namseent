use super::*;
use std::collections::{BTreeMap, btree_map};

type Result<T> = std::io::Result<T>;

/// Assume that operator will be used while before making error.
/// If error occurs, state would be corrupted, so don't use that.
pub struct Operator {
    cache: CachedPages,
    blocks_updated: BTreeMap<PageRange, PageBlock>,
    blocks_read_from_file: BTreeMap<PageRange, PageBlock>,
    file_read_fd: ReadFd,
}

impl Operator {
    pub fn new(cache: CachedPages, file_read_fd: ReadFd) -> Operator {
        Operator {
            cache,
            blocks_updated: Default::default(),
            blocks_read_from_file: Default::default(),
            file_read_fd,
        }
    }
    pub async fn insert(&mut self, key: Key, value: Bytes) -> Result<()> {
        let record_page_range = self.allocate_record_pages(value).await?;

        let mut route = self.find_route_for_insertion(key).await?;
        let leaf_node_offset = route.pop().unwrap();

        let right_node_offset = self.reserve_page_offset().await?;

        let leaf_node = self
            .page_mut(leaf_node_offset, PageBlockTypeHint::Node)
            .await?
            .as_leaf_node_mut();

        if !leaf_node.is_full() {
            leaf_node.insert(key, record_page_range);
            self.rollback_reserve_page_offset(right_node_offset).await?;
            return Ok(());
        }

        let (right_half, center_key) =
            leaf_node.split_and_insert(key, record_page_range, right_node_offset);
        self.blocks_updated.insert(
            PageRange::page(right_node_offset),
            PageBlock::Page(Page::LeafNode(right_half)),
        );

        if route.is_empty() {
            assert_eq!(leaf_node_offset, self.header().await?.root_node_offset);

            let internal_node =
                InternalNode::new(&[center_key], &[leaf_node_offset, right_node_offset]);
            let internal_node_offset = self.reserve_page_offset().await?;
            self.blocks_updated.insert(
                PageRange::page(internal_node_offset),
                PageBlock::Page(Page::InternalNode(internal_node)),
            );

            self.header_mut().await?.root_node_offset = internal_node_offset;
            return Ok(());
        }

        let mut center_key = center_key;
        let mut right_node_offset = right_node_offset;

        while let Some(node_offset) = route.pop() {
            let internal_node = self
                .page_mut(node_offset, PageBlockTypeHint::Node)
                .await?
                .as_internal_node_mut();
            let Some((right_node, next_center_key)) =
                internal_node.insert(center_key, right_node_offset)
            else {
                return Ok(());
            };
            right_node_offset = self.reserve_page_offset().await?;
            self.blocks_updated.insert(
                PageRange::page(right_node_offset),
                PageBlock::Page(Page::InternalNode(right_node)),
            );

            if node_offset != self.header().await?.root_node_offset {
                center_key = next_center_key;
                continue;
            }

            assert!(route.is_empty());
            let new_root_node_offset = self.reserve_page_offset().await?;
            let new_root_node =
                InternalNode::new(&[next_center_key], &[node_offset, right_node_offset]);
            self.blocks_updated.insert(
                PageRange::page(new_root_node_offset),
                PageBlock::Page(Page::InternalNode(new_root_node)),
            );
        }

        Ok(())
    }
    pub async fn delete(&mut self, key: Key) -> Result<()> {
        let leaf_node_offset = self.find_leaf_node_for(key).await?;
        let leaf_node = self
            .page(leaf_node_offset, PageBlockTypeHint::Node)
            .await?
            .as_leaf_node();
        if leaf_node.contains(key) {
            let leaf_node = self
                .page_mut(leaf_node_offset, PageBlockTypeHint::Node)
                .await?
                .as_leaf_node_mut();
            let entry = leaf_node.delete(key);
            assert!(!leaf_node.contains(key));

            self.push_free_block(entry.record_page_range).await?;
        }

        Ok(())
    }
    pub async fn contains(&mut self, key: Key) -> Result<bool> {
        let leaf_node_offset = self.find_leaf_node_for(key).await?;
        let leaf_node = self
            .page(leaf_node_offset, PageBlockTypeHint::Node)
            .await?
            .as_leaf_node();
        let contains = leaf_node.contains(key);
        Ok(contains)
    }
    pub async fn get(&mut self, key: Key) -> Result<Option<Bytes>> {
        let leaf_node_offset = self.find_leaf_node_for(key).await?;
        let leaf_node = self
            .page(leaf_node_offset, PageBlockTypeHint::Node)
            .await?
            .as_leaf_node();
        let Some(record_page_range) = leaf_node.get_record_page_range(key) else {
            return Ok(None);
        };
        let bytes = self.record(record_page_range).await?;
        Ok(Some(bytes))
    }
    pub async fn next(&mut self, exclusive_start_key: Option<Key>) -> Result<Option<Vec<Entry>>> {
        let mut leaf_node_offset = self
            .find_leaf_node_for(exclusive_start_key.unwrap_or_default())
            .await?;
        loop {
            let leaf_node = self
                .page(leaf_node_offset, PageBlockTypeHint::Node)
                .await?
                .as_leaf_node();
            match leaf_node.next(exclusive_start_key) {
                NextResult::Found { key_ranges } => {
                    let mut entries = vec![];

                    for (key, page_range) in key_ranges {
                        let bytes = self.record(page_range).await?;
                        entries.push(Entry { key, value: bytes });
                    }

                    return Ok(Some(entries));
                }
                NextResult::NoMoreEntries => {
                    return Ok(None);
                }
                NextResult::CheckRightNode { right_node_offset } => {
                    leaf_node_offset = right_node_offset;
                    continue;
                }
            }
        }
    }
    pub async fn file_size(&mut self) -> Result<usize> {
        Ok(self.header().await?.file_size())
    }
    pub fn done(self) -> Done {
        Done {
            pages_read_from_file: self.blocks_read_from_file,
            updated_pages: self.blocks_updated,
        }
    }
    async fn find_route_for_insertion(&mut self, key: Key) -> Result<Vec<PageOffset>> {
        let mut node_offset = self.header().await?.root_node_offset;
        let mut route = vec![];

        loop {
            route.push(node_offset);
            let page_block = self
                .page_block(PageRange::page(node_offset), PageBlockTypeHint::Node)
                .await?;
            match page_block.as_page() {
                Page::InternalNode(internal_node) => {
                    node_offset = internal_node.find_child_offset_for(key);
                }
                Page::LeafNode(..) => {
                    return Ok(route);
                }
                x => unreachable!("{:?}", x),
            }
        }
    }
    async fn page_mut(
        &mut self,
        page_offset: PageOffset,
        hint: PageBlockTypeHint,
    ) -> Result<&mut Page> {
        let block_page_range = PageRange::page(page_offset);

        if let btree_map::Entry::Vacant(e) = self.blocks_updated.entry(block_page_range) {
            let page_block = {
                if let Some(page) = self.cache.get(&block_page_range) {
                    page.as_ref().clone()
                } else {
                    if let btree_map::Entry::Vacant(e) =
                        self.blocks_read_from_file.entry(block_page_range)
                    {
                        let page_block =
                            read_block_from_file(&self.file_read_fd, block_page_range, hint)
                                .await?;
                        e.insert(page_block);
                    }

                    self.blocks_read_from_file
                        .get(&block_page_range)
                        .unwrap()
                        .clone()
                }
            };
            e.insert(page_block);
        }

        Ok(self
            .blocks_updated
            .get_mut(&block_page_range)
            .unwrap()
            .as_page_mut())
    }
    async fn reserve_page_offset(&mut self) -> Result<PageOffset> {
        Ok(self.header_mut().await?.next_page_offset.fetch_increase(1))
    }
    async fn header(&mut self) -> Result<&Header> {
        Ok(self
            .page(PageOffset::HEADER, PageBlockTypeHint::Header)
            .await?
            .as_header())
    }

    async fn header_mut(&mut self) -> Result<&mut Header> {
        Ok(self
            .page_mut(PageOffset::HEADER, PageBlockTypeHint::Header)
            .await?
            .as_header_mut())
    }
    async fn find_leaf_node_for(&mut self, key: Key) -> Result<PageOffset> {
        let mut node_offset = self.header().await?.root_node_offset;

        loop {
            let page_block = self
                .page_block(PageRange::page(node_offset), PageBlockTypeHint::Node)
                .await?;
            match page_block.as_page() {
                Page::InternalNode(internal_node) => {
                    node_offset = internal_node.find_child_offset_for(key);
                }
                Page::LeafNode(..) => {
                    return Ok(node_offset);
                }
                x => unreachable!("{:?}", x),
            };
        }
    }
    async fn record(&mut self, page_range: PageRange) -> Result<Bytes> {
        Ok(self
            .page_block(page_range, PageBlockTypeHint::Record)
            .await?
            .as_record()
            .content())
    }
    async fn page(&mut self, page_offset: PageOffset, hint: PageBlockTypeHint) -> Result<&Page> {
        Ok(self
            .page_block(PageRange::page(page_offset), hint)
            .await?
            .as_page())
    }
    async fn page_block(
        &mut self,
        page_range: PageRange,
        hint: PageBlockTypeHint,
    ) -> Result<&PageBlock> {
        if let Some(page_block) = self.blocks_updated.get(&page_range) {
            Ok(page_block)
        } else if let Some(page_block) = self.cache.get(&page_range) {
            Ok(page_block.as_ref())
        } else {
            if let btree_map::Entry::Vacant(e) = self.blocks_read_from_file.entry(page_range) {
                let block = read_block_from_file(&self.file_read_fd, page_range, hint).await?;
                e.insert(block);
            }

            Ok(self.blocks_read_from_file.get(&page_range).unwrap())
        }
    }
    async fn allocate_record_pages(&mut self, value: Bytes) -> Result<PageRange> {
        let record = Record::new(value);

        let page_count = record.page_count();

        let page_range = if let Some(page_range) = self.try_pop_free_block(page_count).await? {
            page_range
        } else {
            let page_offset = self
                .header_mut()
                .await?
                .next_page_offset
                .fetch_increase(page_count as usize);

            PageRange::data(page_offset, page_count)
        };

        self.blocks_updated
            .insert(page_range, PageBlock::Record(record));

        Ok(page_range)
    }
    async fn rollback_reserve_page_offset(&mut self, right_node_offset: PageOffset) -> Result<()> {
        assert_eq!(
            self.header().await?.next_page_offset.as_u32(),
            right_node_offset.as_u32() + 1
        );
        self.header_mut().await?.next_page_offset.decrease();
        Ok(())
    }
    async fn free_stack_mut(&mut self, page_offset: PageOffset) -> Result<&mut FreeStackNode> {
        Ok(self
            .page_mut(page_offset, PageBlockTypeHint::FreeStackNode)
            .await?
            .as_free_stack_mut())
    }
    async fn try_pop_free_block(&mut self, page_count: u8) -> Result<Option<PageRange>> {
        let mut parent_page_offset = PageOffset::NULL;
        let mut page_offset = self.header().await?.free_stack_top_page_offset;

        loop {
            if page_offset.is_null() {
                return Ok(None);
            }
            let free_stack = self.free_stack_mut(page_offset).await?;

            if let Some(range) = free_stack.try_pop(page_count) {
                if free_stack.is_empty() && !parent_page_offset.is_null() {
                    let next_page_offset = free_stack.next_page_offset;
                    let parent_node = self.free_stack_mut(parent_page_offset).await?;
                    if !parent_node.is_full() {
                        parent_node.next_page_offset = next_page_offset;
                        parent_node.push(PageRange::page(page_offset));
                    }
                }

                return Ok(Some(range));
            }

            parent_page_offset = page_offset;
            page_offset = free_stack.next_page_offset;
        }
    }

    async fn push_free_block(&mut self, page_range: PageRange) -> Result<()> {
        enum Parent {
            Header,
            FreeStack(PageOffset),
        }
        let mut parent = Parent::Header;
        let mut page_offset = self.header().await?.free_stack_top_page_offset;
        loop {
            if page_offset.is_null() {
                let new_free_stack_offset = self.reserve_page_offset().await?;
                let new_free_stack = FreeStackNode::new();
                self.blocks_updated.insert(
                    PageRange::page(new_free_stack_offset),
                    PageBlock::Page(Page::FreeStackNode(new_free_stack)),
                );
                match parent {
                    Parent::Header => {
                        self.header_mut().await?.free_stack_top_page_offset = new_free_stack_offset;
                    }
                    Parent::FreeStack(page_offset) => {
                        self.free_stack_mut(page_offset).await?.next_page_offset =
                            new_free_stack_offset;
                    }
                }
                page_offset = new_free_stack_offset;
                continue;
            }

            let free_stack = self.free_stack_mut(page_offset).await?;
            if !free_stack.is_full() {
                free_stack.push(page_range);
                return Ok(());
            }
            parent = Parent::FreeStack(page_offset);
            page_offset = free_stack.next_page_offset;
        }
    }
}

async fn read_block_from_file(
    read_fd: &ReadFd,
    block_page_range: PageRange,
    hint: PageBlockTypeHint,
) -> Result<PageBlock> {
    let bytes = read_fd
        .read_exact(block_page_range.file_offset(), block_page_range.byte_len())
        .await?;

    match hint {
        PageBlockTypeHint::Header => {
            assert_eq!(block_page_range.page_count(), 1);
            assert_eq!(bytes.len(), PAGE_LEN);
            let header = Header::from_slice(&bytes);
            Ok(PageBlock::Page(Page::Header(header)))
        }
        PageBlockTypeHint::Record => {
            let record = Record::from_slice(&bytes);
            Ok(PageBlock::Record(record))
        }
        PageBlockTypeHint::Node => {
            assert_eq!(bytes.len(), PAGE_LEN);
            assert_eq!(block_page_range.page_count(), 1);
            let node = Node::from_slice(&bytes);
            Ok(PageBlock::Page(match node {
                Node::Internal(internal_node) => Page::InternalNode(internal_node),
                Node::Leaf(leaf_node) => Page::LeafNode(leaf_node),
            }))
        }
        PageBlockTypeHint::FreeStackNode => {
            assert_eq!(block_page_range.page_count(), 1);
            assert_eq!(bytes.len(), PAGE_LEN);
            let free_stack_node = FreeStackNode::from_slice(&bytes);
            Ok(PageBlock::Page(Page::FreeStackNode(free_stack_node)))
        }
    }
}

pub struct Done {
    pub updated_pages: BTreeMap<PageRange, PageBlock>,
    pub pages_read_from_file: BTreeMap<PageRange, PageBlock>,
}

// TODO: Wrap this hint by functions, not passing every time, because code is too verbose
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PageBlockTypeHint {
    Header,
    FreeStackNode,
    Record,
    Node,
}
