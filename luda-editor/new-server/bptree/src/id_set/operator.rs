use super::*;
use std::{
    collections::{btree_map, hash_map, BTreeMap, HashMap},
    fs::File,
};

pub struct Operator<'a> {
    pages_cached: &'a HashMap<PageOffset, Page>,
    pages_updated: BTreeMap<PageOffset, Page>,
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
            pages_updated: Default::default(),
            pages_read_from_file: HashMap::new(),
            updated_header: None,
            original_header: header,
            file,
        }
    }

    pub fn insert(&mut self, id: u128) -> Result<()> {
        let mut route = self.find_route_for_insertion(id)?;
        let leaf_node_offset = route.pop().unwrap();
        let leaf_node = self.page_mut(leaf_node_offset)?.as_leaf_node_mut();

        let Some((right_half, id)) = leaf_node.insert(id) else {
            return Ok(());
        };

        let right_node_offset = self.new_page()?;
        *self.page_mut(right_node_offset)?.as_leaf_node_mut() = right_half;

        if route.is_empty() {
            assert_eq!(leaf_node_offset, self.header().root_node_offset);

            let internal_node_offset = self.new_page()?;

            let internal_node = self.page_mut(internal_node_offset)?.as_internal_node_mut();
            *internal_node = InternalNode::new(id, leaf_node_offset, right_node_offset);

            self.header_mut().root_node_offset = internal_node_offset;
            return Ok(());
        }

        let mut id = id;
        let mut right_node_offset = right_node_offset;

        while let Some(node_offset) = route.pop() {
            let internal_node = self.page_mut(node_offset)?.as_internal_node_mut();
            let Some((right_node, center_id)) = internal_node.insert(id, right_node_offset) else {
                return Ok(());
            };
            right_node_offset = self.new_page()?;
            *self.page_mut(right_node_offset)?.as_internal_node_mut() = right_node;

            if node_offset != self.header().root_node_offset {
                id = center_id;
                continue;
            }

            assert!(route.is_empty());
            let new_root_node_offset = self.new_page()?;
            let new_root_node = InternalNode::new(center_id, node_offset, right_node_offset);
            *self.page_mut(new_root_node_offset)?.as_internal_node_mut() = new_root_node;
        }

        Ok(())
    }
    pub fn done(self) -> Done {
        Done {
            updated_header: self.updated_header,
            pages_read_from_file: self.pages_read_from_file,
            updated_pages: self.pages_updated,
        }
    }
    fn find_route_for_insertion(&mut self, id: u128) -> Result<Vec<PageOffset>> {
        let mut node_offset = self.header().root_node_offset;
        let mut route = vec![];

        loop {
            route.push(node_offset);
            let node = self.page(node_offset)?.into_node();
            if node.is_leaf() {
                return Ok(route);
            }
            let internal_node = node.into_internal_node();
            node_offset = internal_node.find_child_node_offset_for(id);
        }
    }
    fn page(&mut self, page_offset: PageOffset) -> Result<&Page> {
        if let Some(page) = self.pages_updated.get(&page_offset) {
            Ok(page)
        } else if let Some(page) = self.pages_cached.get(&page_offset) {
            Ok(page)
        } else {
            if let hash_map::Entry::Vacant(e) = self.pages_read_from_file.entry(page_offset) {
                let page = read_page_from_file(self.file, page_offset)?;
                e.insert(page);
            }

            Ok(self.pages_read_from_file.get(&page_offset).unwrap())
        }
    }
    fn page_mut(&mut self, page_offset: PageOffset) -> Result<&mut Page> {
        if let btree_map::Entry::Vacant(e) = self.pages_updated.entry(page_offset) {
            let page = {
                if let Some(page) = self.pages_cached.get(&page_offset) {
                    *page
                } else {
                    if let hash_map::Entry::Vacant(e) = self.pages_read_from_file.entry(page_offset)
                    {
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
    fn allocate_page(&mut self) -> Result<PageOffset> {
        let page_offset = self.header_mut().next_page_offset.fetch_increase();
        let page = Page::new();

        self.pages_updated.insert(page_offset, page);

        Ok(page_offset)
    }
    fn header(&self) -> &Header {
        self.updated_header.as_ref().unwrap_or(self.original_header)
    }

    fn header_mut(&mut self) -> &mut Header {
        self.updated_header.get_or_insert(*self.original_header)
    }
}

pub struct Done {
    pub updated_pages: BTreeMap<PageOffset, Page>,
    pub pages_read_from_file: HashMap<PageOffset, Page>,
    /// If header was updated, `updated_pages` will contain the updated header page too.
    pub updated_header: Option<Header>,
}
