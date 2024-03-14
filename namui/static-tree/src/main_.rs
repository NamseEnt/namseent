use std::{
    hash::BuildHasherDefault,
    sync::{Arc, RwLock},
};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    // IndexMap: 4.008µs
    // IndexMap Fx: 3.477µs
    // IndexMap Fx RwLock: 10.97µs
    // elsa::FrozenIndexMap: 7.123µs
    // DashMap: 13.815µs
    // DashMap FxHasher: 28.072µs
    // DashMap MT: 220.177µs
    // DashMap MT FxHasher: 1.520171ms
    // DashMap MT FxHasher, Single Iter: 71.331µs

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Data {
        value: [u8; 52],
    }
    impl Data {
        fn new(i: u8) -> Self {
            Self { value: [i; 52] }
        }
    }
    const CAPACITY: usize = 10;
    for _ in 0..10 {
        let now = std::time::Instant::now();

        {
            let map = elsa::FrozenIndexMap::new();
            for i in 0..CAPACITY {
                map.insert(i, Box::new(i));
            }

            let mut sum = 0;
            for entry in map.into_tuple_vec() {
                sum += *entry.1 as u32;
            }

            println!("sum: {}", sum);
        }

        println!("elsa::FrozenMap: {:?}", now.elapsed());

        let now = std::time::Instant::now();

        {
            let mut map = indexmap::IndexMap::with_capacity(CAPACITY);
            for i in 0..CAPACITY {
                map.insert(i, i);
            }

            let mut sum = 0;
            for entry in map {
                // sum += entry.1.value[0] as u32;
                sum += entry.1 as u32;
            }

            println!("sum: {}", sum);
        }

        println!("IndexMap: {:?}", now.elapsed());

        // let now = std::time::Instant::now();

        // {
        //     let mut map = indexmap::IndexMap::with_capacity_and_hasher(
        //         CAPACITY,
        //         BuildHasherDefault::<rustc_hash::FxHasher>::default(),
        //     );
        //     for i in 0..CAPACITY {
        //         map.insert(i, Data::new(i as u8));
        //     }

        //     let mut sum = 0;
        //     for entry in map {
        //         sum += entry.1.value[0] as u32;
        //     }

        //     println!("sum: {}", sum);
        // }

        // println!("IndexMap Fx: {:?}", now.elapsed());

        // {
        //     let map = Arc::new(RwLock::new(indexmap::IndexMap::with_capacity_and_hasher(
        //         CAPACITY,
        //         BuildHasherDefault::<rustc_hash::FxHasher>::default(),
        //     )));
        //     for i in 0..CAPACITY {
        //         map.write().unwrap().insert(i, Data::new(i as u8));
        //     }

        //     let mut sum = 0;
        //     {
        //         for entry in map.read().unwrap().iter() {
        //             sum += entry.1.value[0] as u32;
        //         }
        //     }

        //     println!("sum: {}", sum);
        // }

        // println!("IndexMap Fx RwLock: {:?}", now.elapsed());

        // let now = std::time::Instant::now();

        // {
        //     let map = dashmap::DashMap::with_capacity(CAPACITY);
        //     for i in 0..CAPACITY {
        //         map.insert(i, Data::new(i as u8));
        //     }

        //     let mut sum = 0;
        //     for entry in map {
        //         sum += entry.1.value[0] as u32;
        //     }

        //     println!("sum: {}", sum);
        // }

        // println!("DashMap: {:?}", now.elapsed());

        // {
        //     let map = dashmap::DashMap::with_capacity_and_hasher(
        //         CAPACITY,
        //         BuildHasherDefault::<rustc_hash::FxHasher>::default(),
        //     );
        //     for i in 0..CAPACITY {
        //         map.insert(i, Data::new(i as u8));
        //     }

        //     let mut sum = 0;
        //     for entry in map {
        //         sum += entry.1.value[0] as u32;
        //     }

        //     println!("sum: {}", sum);
        // }

        // println!("DashMap FxHasher: {:?}", now.elapsed());

        // let now = std::time::Instant::now();

        // {
        //     let map = dashmap::DashMap::with_capacity(CAPACITY);
        //     (0..CAPACITY).into_par_iter().for_each(|i| {
        //         map.insert(i, Data::new(i as u8));
        //     });

        //     let sum: u32 = map
        //         .into_iter()
        //         .par_bridge()
        //         .map(|entry| entry.1.value[0] as u32)
        //         .sum();

        //     println!("sum: {}", sum);
        // }

        // println!("DashMap MT: {:?}", now.elapsed());

        // let now = std::time::Instant::now();

        // {
        //     let map = dashmap::DashMap::with_capacity_and_hasher(
        //         CAPACITY,
        //         BuildHasherDefault::<rustc_hash::FxHasher>::default(),
        //     );
        //     (0..CAPACITY).into_par_iter().for_each(|i| {
        //         map.insert(i, Data::new(i as u8));
        //     });

        //     let sum: u32 = map
        //         .into_iter()
        //         .par_bridge()
        //         .map(|entry| entry.1.value[0] as u32)
        //         .sum();

        //     println!("sum: {}", sum);
        // }

        // println!("DashMap MT FxHasher: {:?}", now.elapsed());

        // let now = std::time::Instant::now();

        // {
        //     let map = dashmap::DashMap::with_capacity_and_hasher(
        //         CAPACITY,
        //         BuildHasherDefault::<rustc_hash::FxHasher>::default(),
        //     );
        //     (0..CAPACITY).into_par_iter().for_each(|i| {
        //         map.insert(i, Data::new(i as u8));
        //     });

        //     let mut sum = 0;
        //     for entry in map {
        //         sum += entry.1.value[0] as u32;
        //     }

        //     println!("sum: {}", sum);
        // }

        // println!("DashMap MT FxHasher, Single Iter: {:?}", now.elapsed());

        println!("----");
    }
}
