use super::*;
use std::collections::{BTreeMap, btree_map::Entry};

type Result<T> = std::io::Result<T>;

pub struct Operator {
    cache: CachedPages,
    pages_updated: BTreeMap<PageOffset, Page>,
    pages_read_from_file: BTreeMap<PageOffset, Page>,
    file_read_fd: ReadFd,
}

impl Operator {
    pub fn new(cache: CachedPages, file_read_fd: ReadFd) -> Operator {
        Operator {
            cache,
            pages_updated: Default::default(),
            pages_read_from_file: Default::default(),
            file_read_fd,
        }
    }
    pub async fn insert(&mut self, id: u128) -> Result<()> {
        let mut route = self.find_route_for_insertion(id).await?;
        let leaf_node_offset = route.pop().unwrap();
        let leaf_node = self.page_mut(leaf_node_offset).await?.as_leaf_node_mut();

        if !leaf_node.is_full() {
            leaf_node.insert(id);
            return Ok(());
        }

        let right_node_offset = self.new_page().await?;
        // ↓ I know this is duplicated code but I need this to pass mutable borrow check
        let leaf_node = self.page_mut(leaf_node_offset).await?.as_leaf_node_mut();
        let (right_half, center_id) =
            leaf_node.split_and_insert(id, leaf_node_offset, right_node_offset);

        *self.page_mut(right_node_offset).await?.as_leaf_node_mut() = right_half;

        if route.is_empty() {
            assert_eq!(leaf_node_offset, self.header().await.root_node_offset);

            let internal_node_offset = self.new_page().await?;

            let internal_node = self
                .page_mut(internal_node_offset)
                .await?
                .as_internal_node_mut();
            *internal_node =
                InternalNode::new(&[center_id], &[leaf_node_offset, right_node_offset]);

            self.header_mut().await.root_node_offset = internal_node_offset;
            return Ok(());
        }

        let mut center_id = center_id;
        let mut right_node_offset = right_node_offset;

        while let Some(node_offset) = route.pop() {
            let internal_node = self.page_mut(node_offset).await?.as_internal_node_mut();
            let Some((right_node, next_center_id)) =
                internal_node.insert(center_id, right_node_offset)
            else {
                return Ok(());
            };
            right_node_offset = self.new_page().await?;
            *self
                .page_mut(right_node_offset)
                .await?
                .as_internal_node_mut() = right_node;

            if node_offset != self.header().await.root_node_offset {
                center_id = next_center_id;
                continue;
            }

            assert!(route.is_empty());
            let new_root_node_offset = self.new_page().await?;
            let new_root_node =
                InternalNode::new(&[next_center_id], &[node_offset, right_node_offset]);
            *self
                .page_mut(new_root_node_offset)
                .await?
                .as_internal_node_mut() = new_root_node;
        }

        Ok(())
    }
    pub async fn delete(&mut self, id: u128) -> Result<()> {
        let leaf_node_offset = self.find_leaf_node_for(id).await?;
        let leaf_node = self.page(leaf_node_offset).await?.as_leaf_node();
        if leaf_node.contains(id) {
            let leaf_node = self.page_mut(leaf_node_offset).await?.as_leaf_node_mut();
            leaf_node.delete(id);
        }

        Ok(())
    }
    pub async fn contains(&mut self, id: u128) -> Result<bool> {
        let leaf_node_offset = self.find_leaf_node_for(id).await?;
        let leaf_node = self.page(leaf_node_offset).await?.as_leaf_node();
        let contains = leaf_node.contains(id);
        Ok(contains)
    }
    pub async fn next(&mut self, exclusive_start_id: Option<u128>) -> Result<Option<Vec<Id>>> {
        let mut leaf_node_offset = self
            .find_leaf_node_for(exclusive_start_id.unwrap_or_default())
            .await?;
        loop {
            let leaf_node = self.page(leaf_node_offset).await?.as_leaf_node();
            match leaf_node.next(exclusive_start_id) {
                NextResult::Found { ids } => {
                    return Ok(Some(ids));
                }
                NextResult::NoMoreIds => {
                    return Ok(None);
                }
                NextResult::CheckRightNode { right_node_offset } => {
                    leaf_node_offset = right_node_offset;
                    continue;
                }
            }
        }
    }
    pub fn done(self) -> Done {
        Done {
            pages_read_from_file: self.pages_read_from_file,
            updated_pages: self.pages_updated,
        }
    }
    async fn find_route_for_insertion(&mut self, id: u128) -> Result<Vec<PageOffset>> {
        let mut node_offset = self.header().await.root_node_offset;
        let mut route = vec![];

        loop {
            route.push(node_offset);
            match self.page(node_offset).await?.as_node().as_one_of() {
                NodeMatchRef::Internal { internal_node } => {
                    node_offset = internal_node.find_child_offset_for(id);
                }
                NodeMatchRef::Leaf { .. } => {
                    return Ok(route);
                }
            }
        }
    }
    async fn page(&mut self, page_offset: PageOffset) -> Result<&Page> {
        if let Some(page) = self.pages_updated.get(&page_offset) {
            Ok(page)
        } else if let Some(page) = self.cache.get(&page_offset) {
            Ok(page)
        } else {
            if let Entry::Vacant(e) = self.pages_read_from_file.entry(page_offset) {
                let page = read_page_from_file(&self.file_read_fd, page_offset).await?;
                e.insert(page);
            }

            Ok(self.pages_read_from_file.get(&page_offset).unwrap())
        }
    }
    async fn page_mut(&mut self, page_offset: PageOffset) -> Result<&mut Page> {
        if let Entry::Vacant(e) = self.pages_updated.entry(page_offset) {
            let page = {
                if let Some(page) = self.cache.get(&page_offset) {
                    page.as_ref().clone()
                } else {
                    if let Entry::Vacant(e) = self.pages_read_from_file.entry(page_offset) {
                        let page = read_page_from_file(&self.file_read_fd, page_offset).await?;
                        e.insert(page);
                    }

                    self.pages_read_from_file.get(&page_offset).unwrap().clone()
                }
            };
            e.insert(page);
        }

        Ok(self.pages_updated.get_mut(&page_offset).unwrap())
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
    async fn allocate_page(&mut self) -> Result<PageOffset> {
        let page_offset = self.header_mut().await.next_page_offset.fetch_increase();
        let page = Page::empty();

        self.pages_updated.insert(page_offset, page);

        Ok(page_offset)
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
    async fn find_leaf_node_for(&mut self, id: u128) -> Result<PageOffset> {
        let mut node_offset = self.header().await.root_node_offset;

        loop {
            match self.page(node_offset).await?.as_node().as_one_of() {
                NodeMatchRef::Internal { internal_node } => {
                    node_offset = internal_node.find_child_offset_for(id);
                }
                NodeMatchRef::Leaf { .. } => {
                    return Ok(node_offset);
                }
            };
        }
    }
}

async fn read_page_from_file(read_fd: &ReadFd, page_offset: PageOffset) -> Result<Page> {
    let bytes = read_fd.read_page(page_offset).await?;
    Ok(Page::new(bytes))
}

pub struct Done {
    pub updated_pages: BTreeMap<PageOffset, Page>,
    pub pages_read_from_file: BTreeMap<PageOffset, Page>,
}
