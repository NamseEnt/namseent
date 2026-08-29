#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EntityIdAllocator {
    next: u64,
}

impl Default for EntityIdAllocator {
    fn default() -> Self {
        Self { next: 1 }
    }
}

impl EntityIdAllocator {
    pub fn from_next_id(next: u64) -> Self {
        Self { next }
    }

    pub fn next_id(self) -> u64 {
        self.next
    }

    pub fn allocate_raw(&mut self) -> u64 {
        let id = self.next;
        self.next = self.next.checked_add(1).expect("entity ID space exhausted");
        id
    }
}
