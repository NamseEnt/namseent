use super::*;
use std::collections::{btree_map::Entry, BTreeMap};

type Result<T> = std::io::Result<T>;

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
        let leaf_node = self.page_mut(leaf_node_offset).await?.as_leaf_node_mut();

        if !leaf_node.is_full() {
            leaf_node.insert(key, record_page_range);
            return Ok(());
        }

        let right_node_offset = self.new_page().await?;
        // ↓ I know this is duplicated code but I need this to pass mutable borrow check
        let leaf_node = self.page_mut(leaf_node_offset).await?.as_leaf_node_mut();
        let (right_half, center_key) =
            leaf_node.split_and_insert(key, record_page_range, leaf_node_offset, right_node_offset);

        *self.page_mut(right_node_offset).await?.as_leaf_node_mut() = right_half;

        if route.is_empty() {
            assert_eq!(leaf_node_offset, self.header().await.root_node_offset);

            let internal_node_offset = self.new_page().await?;

            let internal_node = self
                .page_mut(internal_node_offset)
                .await?
                .as_internal_node_mut();
            *internal_node =
                InternalNode::new(&[center_key], &[leaf_node_offset, right_node_offset]);

            self.header_mut().await.root_node_offset = internal_node_offset;
            return Ok(());
        }

        let mut center_key = center_key;
        let mut right_node_offset = right_node_offset;

        while let Some(node_offset) = route.pop() {
            let internal_node = self.page_mut(node_offset).await?.as_internal_node_mut();
            let Some((right_node, next_center_key)) =
                internal_node.insert(center_key, right_node_offset)
            else {
                return Ok(());
            };
            right_node_offset = self.new_page().await?;
            *self
                .page_mut(right_node_offset)
                .await?
                .as_internal_node_mut() = right_node;

            if node_offset != self.header().await.root_node_offset {
                center_key = next_center_key;
                continue;
            }

            assert!(route.is_empty());
            let new_root_node_offset = self.new_page().await?;
            let new_root_node =
                InternalNode::new(&[next_center_key], &[node_offset, right_node_offset]);
            *self
                .page_mut(new_root_node_offset)
                .await?
                .as_internal_node_mut() = new_root_node;
        }

        Ok(())
    }
    pub async fn delete(&mut self, key: Key) -> Result<()> {
        let leaf_node_offset = self.find_leaf_node_for(key).await?;
        let leaf_node = self.page(leaf_node_offset).await?.as_leaf_node();
        if leaf_node.contains(key) {
            let leaf_node = self.page_mut(leaf_node_offset).await?.as_leaf_node_mut();
            leaf_node.delete(key);
        }

        Ok(())
    }
    pub async fn contains(&mut self, key: Key) -> Result<bool> {
        let leaf_node_offset = self.find_leaf_node_for(key).await?;
        let leaf_node = self.page(leaf_node_offset).await?.as_leaf_node();
        let contains = leaf_node.contains(key);
        Ok(contains)
    }
    pub async fn get(&mut self, key: Key) -> Result<Option<Bytes>> {
        let leaf_node_offset = self.find_leaf_node_for(key).await?;
        let leaf_node = self.page(leaf_node_offset).await?.as_leaf_node();
        let Some(record_page_range) = leaf_node.get_record_page_range(key) else {
            return Ok(None);
        };
        let bytes = self.record(record_page_range).await?;
        Ok(Some(bytes))
    }
    pub async fn next(&mut self, exclusive_start_key: Option<Key>) -> Result<Option<Vec<Key>>> {
        todo!()
        // let mut leaf_node_offset = self
        //     .find_leaf_node_for(exclusive_start_key.unwrap_or_default())
        //     .await?;
        // loop {
        //     let leaf_node = self.page(leaf_node_offset).await?.as_leaf_node();
        //     match leaf_node.next(exclusive_start_key) {
        //         NextResult::Found { keys } => {
        //             return Ok(Some(keys));
        //         }
        //         NextResult::NoMoreKeys => {
        //             return Ok(None);
        //         }
        //         NextResult::CheckRightNode { right_node_offset } => {
        //             leaf_node_offset = right_node_offset;
        //             continue;
        //         }
        //     }
        // }
    }
    pub fn done(self) -> Done {
        Done {
            pages_read_from_file: self.blocks_read_from_file,
            updated_pages: self.blocks_updated,
        }
    }
    async fn find_route_for_insertion(&mut self, key: Key) -> Result<Vec<PageOffset>> {
        let mut node_offset = self.header().await.root_node_offset;
        let mut route = vec![];

        loop {
            route.push(node_offset);
            let page_block = self.page_block(PageRange::page(node_offset)).await?;
            match page_block.into_page().as_node().as_one_of() {
                NodeMatchRef::Internal { internal_node } => {
                    node_offset = internal_node.find_child_offset_for(key);
                }
                NodeMatchRef::Leaf { .. } => {
                    return Ok(route);
                }
            }
        }
    }
    async fn page_mut(&mut self, page_offset: PageOffset) -> Result<&mut Page> {
        let block_page_range = PageRange::page(page_offset);

        if let Entry::Vacant(e) = self.blocks_updated.entry(block_page_range) {
            let page_block = {
                if let Some(page) = self.cache.get(&block_page_range) {
                    page.as_ref().clone().into()
                } else {
                    if let Entry::Vacant(e) = self.blocks_read_from_file.entry(block_page_range) {
                        let page_block =
                            read_block_from_file(&self.file_read_fd, block_page_range).await?;
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
    async fn new_page(&mut self) -> Result<PageOffset> {
        if let Some(page_offset) = self.pop_free_page().await? {
            Ok(page_offset)
        } else {
            self.allocate_page().await
        }
    }
    async fn pop_free_page(&mut self) -> Result<Option<PageOffset>> {
        let free_page_stack_top_page_offset = self.header().await.free_page_stack_top_page_offset;
        if free_page_stack_top_page_offset.is_null() {
            return Ok(None);
        }

        let stack_node = self
            .page_mut(free_page_stack_top_page_offset)
            .await?
            .as_free_page_stack_node_mut();

        let page_offset = stack_node.pop();
        let next_page_offset = stack_node.next_page_offset;

        if stack_node.is_empty() {
            self.header_mut().await.free_page_stack_top_page_offset = next_page_offset;
        }

        Ok(Some(page_offset))
    }
    async fn header(&mut self) -> &Header {
        self.page(PageOffset::HEADER).await.unwrap().as_header()
    }

    async fn header_mut(&mut self) -> &mut Header {
        self.page_mut(PageOffset::HEADER)
            .await
            .unwrap()
            .as_header_mut()
    }
    async fn find_leaf_node_for(&mut self, key: Key) -> Result<PageOffset> {
        let mut node_offset = self.header().await.root_node_offset;

        loop {
            let page_block = self.page_block(PageRange::page(node_offset)).await?;
            match page_block.into_page().as_node().as_one_of() {
                NodeMatchRef::Internal { internal_node } => {
                    node_offset = internal_node.find_child_offset_for(key);
                }
                NodeMatchRef::Leaf { .. } => {
                    return Ok(node_offset);
                }
            };
        }
    }
    async fn record(&mut self, page_range: PageRange) -> Result<Bytes> {
        Ok(self.page_block(page_range).await?.into_record())
    }
    async fn page(&mut self, page_offset: PageOffset) -> Result<&Page> {
        Ok(self
            .page_block(PageRange::page(page_offset))
            .await?
            .into_page())
    }
    async fn page_block(&mut self, page_range: PageRange) -> Result<PageBlockRef> {
        if let Some(page_block) = self.blocks_updated.get(&page_range) {
            Ok(page_block.into())
        } else if let Some(page_block) = self.cache.get(&page_range) {
            Ok(page_block.as_ref().into())
        } else {
            if let Entry::Vacant(e) = self.blocks_read_from_file.entry(page_range) {
                let block = read_block_from_file(&self.file_read_fd, page_range).await?;
                e.insert(block);
            }

            Ok(self.blocks_read_from_file.get(&page_range).unwrap().into())
        }
    }
    async fn allocate_page(&mut self) -> Result<PageOffset> {
        let page_offset = self.header_mut().await.next_page_offset.fetch_increase(1);
        let page = Page::empty();

        let block = PageBlock::from_page(page);

        let block_page_range = PageRange::data(page_offset, 1);
        self.blocks_updated.insert(block_page_range, block);

        Ok(page_offset)
    }
    async fn allocate_record_pages(&mut self, value: Bytes) -> Result<PageRange> {
        // TODO: Get contiguous pages from free page stack

        let record_block = PageBlock::record(value);

        let page_count = record_block.page_count();

        let page_offset = self
            .header_mut()
            .await
            .next_page_offset
            .fetch_increase(page_count as usize);

        let block_page_range = PageRange::data(page_offset, page_count);

        self.blocks_updated.insert(block_page_range, record_block);

        Ok(block_page_range)
    }
}

async fn read_block_from_file(read_fd: &ReadFd, block_page_range: PageRange) -> Result<PageBlock> {
    let block = read_fd.read_block(block_page_range).await?;
    Ok(block)
}

pub struct Done {
    pub updated_pages: BTreeMap<PageRange, PageBlock>,
    pub pages_read_from_file: BTreeMap<PageRange, PageBlock>,
}
