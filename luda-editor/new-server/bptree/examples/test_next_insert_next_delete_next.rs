use bptree::id_set::{Id, IdSet};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let path = std::env::temp_dir().join("test_next_insert_next_delete_next");
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }
    let wal_path = path.with_extension("wal");
    if wal_path.exists() {
        std::fs::remove_file(&wal_path).unwrap();
    }
    let shadow_path = path.with_extension("shadow");
    if shadow_path.exists() {
        std::fs::remove_file(&shadow_path).unwrap();
    }
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    let set = IdSet::new(&path, 5000).unwrap();

    assert!(set.next(None).await.unwrap().is_none());
    assert!(set.next(Some(5)).await.unwrap().is_none());

    let mut join_set = JoinSet::new();
    for i in 1..=300 {
        let set = set.clone();
        join_set.spawn(async move {
            set.insert(i as Id).await.unwrap();
        });
    }
    join_set.join_all().await;

    assert!(!set.contains(0).await.unwrap());

    let mut all_ids = vec![];
    let mut exclusive_start_id = None;
    while let Some(ids) = set.next(exclusive_start_id).await.unwrap() {
        assert!(!ids.contains(&0));
        exclusive_start_id = ids.last().cloned();
        all_ids.extend(ids);
    }
    assert!(set.contains(1).await.unwrap());

    assert_eq!(all_ids.len(), 300);
    for i in 1..=300 {
        assert_eq!(all_ids[i - 1], i as Id);
    }
}
