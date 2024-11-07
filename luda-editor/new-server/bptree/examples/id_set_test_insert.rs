use bptree::id_set::{Id, IdSet};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let path = std::env::temp_dir().join("id_set_test_insert");
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }
    let wal_path = path.with_extension("wal");
    if wal_path.exists() {
        std::fs::remove_file(&wal_path).unwrap();
    }
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    let set = IdSet::new(path, 5000).unwrap();
    let mut join_set = JoinSet::new();
    for i in 1..=10000 {
        let set = set.clone();
        join_set.spawn(async move { set.insert(i as Id).await });
    }

    join_set.join_all().await;
}
