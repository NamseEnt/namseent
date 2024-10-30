use criterion::{criterion_group, criterion_main, Criterion};
use vvv::*;

fn std_vec_string(c: &mut Criterion) {
    c.bench_function("std_vec<String>", |b| {
        b.iter(|| {
            let vec = vec![
                "abcdefg".to_string(),
                "hrsturv".to_string(),
                "안녕하세요".to_string(),
            ];
            let mut bytes = vec.to_bytes();
            let mut vec = Vec::<String>::from_bytes(&mut bytes).unwrap();
            vec.push("Hello".to_string());
        })
    });
}

fn std_vec_str(c: &mut Criterion) {
    c.bench_function("std_vec<Str>", |b| {
        b.iter(|| {
            let vec = vec![
                "abcdefg".to_str(),
                "hrsturv".to_str(),
                "안녕하세요".to_str(),
            ];
            let mut bytes = vec.to_bytes();
            let mut vec = Vec::<Str>::from_bytes(&mut bytes).unwrap();
            vec.push("Hello".to_str());
        })
    });
}

fn list_string(c: &mut Criterion) {
    c.bench_function("List<String>", |b| {
        b.iter(|| {
            let list = list![
                "abcdefg".to_string(),
                "hrsturv".to_string(),
                "안녕하세요".to_string(),
            ];
            let mut bytes = list.to_bytes();
            let mut list = List::<String>::from_bytes(&mut bytes).unwrap();
            list.push("Hello".to_string());
        })
    });
}

fn list_str(c: &mut Criterion) {
    c.bench_function("List<Str>", |b| {
        b.iter(|| {
            let list = list![
                "abcdefg".to_str(),
                "hrsturv".to_str(),
                "안녕하세요".to_str(),
            ];
            let mut bytes = list.to_bytes();
            let mut list = List::<Str>::from_bytes(&mut bytes).unwrap();
            list.push("Hello".to_str());
        })
    });
}

criterion_group!(benches, std_vec_string, std_vec_str, list_string, list_str);
criterion_main!(benches);
