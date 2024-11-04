use super::*;
use std::collections::{hash_map::Entry, HashMap};

pub struct Operator<'a> {
    pages_cached: &'a HashMap<PageOffset, Page>,
    pages_updated: HashMap<PageOffset, Page>,
    pages_read_from_file: HashMap<PageOffset, Page>,
    updated_header: Option<Header>,
    original_header: &'a Header,
    file: &'a mut File,
}

impl<'a> Operator<'a> {
    pub fn new(
        header: &'a Header,
        pages_cached: &'a HashMap<PageOffset, Page>,
        file: &'a mut File,
    ) -> Operator<'a> {
        Operator {
            pages_cached,
            pages_updated: HashMap::new(),
            pages_read_from_file: HashMap::new(),
            updated_header: None,
            original_header: header,
            file,
        }
    }

    pub fn insert(mut self, key: u128) -> Result<Done> {
        let mut route = self.find_route_for_insertion(key)?;
        let leaf_node_offset = route.pop().unwrap();
        let leaf_node = self.page_mut(leaf_node_offset)?.as_leaf_node_mut();

        let Some((right_half, key)) = leaf_node.insert(key) else {
            return Ok(self.done());
        };

        let right_node_offset = self.new_page()?;
        *self.page_mut(right_node_offset)?.as_leaf_node_mut() = right_half;

        if route.is_empty() {
            let internal_node_offset = self.new_page()?;
            let internal_node = self.page_mut(internal_node_offset)?.as_internal_node_mut();
            *internal_node = InternalNode::new(key, leaf_node_offset, right_node_offset);

            self.header_mut().root_node_offset = internal_node_offset;
            return Ok(self.done());
        }

        for parent_offset in route.iter().rev().cloned() {
            let parent_node = self.page_mut(parent_offset)?.as_internal_node_mut();
            parent_node.insert(key, right_node_offset);

            // let Some((half, key, key)) = parent_node.insert(new_leaf_node_offset, key) else {
            //     return Ok(self.done());
            // };
        }

        todo!()

        // - Otherwise, before inserting the new record
        //     - Split the node.
        //         - original node has ⌈(K+1)/2⌉ items
        //         - new node has ⌊(K+1)/2⌋ items
        //     - Copy ⌈(K+1)/2⌉-th key to the parent, and insert the new node to the parent.
        //     - Repeat until a parent is found that need not split.
        //     - Insert the new record into the new node.
        // - If the root splits, treat it as if it has an empty parent and split as outline above.
    }
    fn done(self) -> Done {
        todo!()
        // Done {
        //     updated_pages: self.pages_updated,
        //     logs: self.logs,
        //     updated_header: if self.is_header_updated {
        //         Some(self.original_header)
        //     } else {
        //         None
        //     },
        // }
    }
    fn find_route_for_insertion(&mut self, key: u128) -> Result<Vec<PageOffset>> {
        let mut node_offset = self.original_header.root_node_offset;
        let mut route = vec![];

        loop {
            route.push(node_offset);
            let node = self.page(node_offset)?.into_node();
            if node.is_leaf() {
                return Ok(route);
            }
            let internal_node = node.into_internal_node();
            node_offset = internal_node.find_child_node_offset_for(key);
        }
    }
    fn page(&mut self, page_offset: PageOffset) -> Result<&Page> {
        if let Some(page) = self.pages_updated.get(&page_offset) {
            Ok(page)
        } else if let Some(page) = self.pages_cached.get(&page_offset) {
            Ok(page)
        } else {
            if let Entry::Vacant(e) = self.pages_read_from_file.entry(page_offset) {
                let page = read_page_from_file(self.file, page_offset)?;
                e.insert(page);
            }

            Ok(self.pages_read_from_file.get(&page_offset).unwrap())
        }
    }
    fn page_mut(&mut self, page_offset: PageOffset) -> Result<&mut Page> {
        if let Entry::Vacant(e) = self.pages_updated.entry(page_offset) {
            let page = {
                if let Some(page) = self.pages_cached.get(&page_offset) {
                    *page
                } else {
                    if let Entry::Vacant(e) = self.pages_read_from_file.entry(page_offset) {
                        let page = read_page_from_file(self.file, page_offset)?;
                        e.insert(page);
                    }

                    *self.pages_read_from_file.get(&page_offset).unwrap()
                }
            };
            e.insert(page);
        }

        Ok(self.pages_updated.get_mut(&page_offset).unwrap())
    }
    fn new_page(&mut self) -> Result<PageOffset> {
        if let Some(page_offset) = self.pop_free_page()? {
            Ok(page_offset)
        } else {
            self.allocate_page()
        }
    }
    fn pop_free_page(&mut self) -> Result<Option<PageOffset>> {
        let free_page_stack_top_page_offset = self.header().free_page_stack_top_page_offset;
        if free_page_stack_top_page_offset.is_null() {
            return Ok(None);
        }

        let stack_node = self
            .page_mut(free_page_stack_top_page_offset)?
            .as_free_page_stack_node_mut();

        let page_offset = stack_node.pop();
        let next_page_offset = stack_node.next_page_offset;

        if stack_node.is_empty() {
            self.header_mut().free_page_stack_top_page_offset = next_page_offset;
        }

        Ok(Some(page_offset))
    }
    fn push_free_page(&mut self) -> Result<()> {
        if self.header().free_page_stack_top_page_offset.is_null() {
            // let page_offset = self.allocate_page()?;
            // self.wal.update_free_page_stack_top_page_offset(page_offset)?;
            // self.header.free_page_stack_top_page_offset = page_offset;

            // let stack_top = page.into_free_page_stack_node();
        }

        todo!()
    }
    fn allocate_page(&mut self) -> Result<PageOffset> {
        let page_offset = self.header_mut().next_page_offset.fetch_increase();
        let page = Page::new();

        self.pages_updated.insert(page_offset, page);

        Ok(page_offset)
    }
    fn header(&self) -> &Header {
        self.updated_header
            .as_ref()
            .unwrap_or_else(|| &self.original_header)
    }

    fn header_mut(&mut self) -> &mut Header {
        self.updated_header
            .get_or_insert_with(|| self.original_header.clone())
    }
}

pub enum OperationLog {
    UpdatePage { page_offset: PageOffset, page: Page },
}

pub struct Done {
    pub updated_pages: HashMap<PageOffset, Page>,
    pub logs: Vec<OperationLog>,
    pub updated_header: Option<Header>,
}
