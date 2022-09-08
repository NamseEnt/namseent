#[test]
fn yrs_decode_test() {
    use yrs::updates::decoder::Decode;

    let buffer = [
        0x00, 0x00, 0x06, 0xd0, 0x87, 0xf7, 0xf9, 0x0d, 0x02, 0x03, 0x02, 0x01, 0x00, 0x00, 0x07,
        0x28, 0x00, 0x27, 0x00, 0x28, 0x01, 0x27, 0x52, 0x47, 0x72, 0x6f, 0x6f, 0x74, 0x5f, 0x5f,
        0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x5f, 0x72, 0x6f, 0x6f, 0x74, 0x73, 0x65,
        0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x69, 0x64, 0x6e, 0x61, 0x6d, 0x65, 0x63, 0x75, 0x74,
        0x73, 0x72, 0x6f, 0x6f, 0x74, 0x63, 0x68, 0x61, 0x72, 0x61, 0x63, 0x74, 0x65, 0x72, 0x73,
        0x72, 0x6f, 0x6f, 0x74, 0x66, 0x61, 0x63, 0x65, 0x5f, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73,
        0x73, 0x69, 0x6f, 0x6e, 0x73, 0x04, 0x0b, 0x04, 0x08, 0x02, 0x44, 0x01, 0x0a, 0x04, 0x10,
        0x05, 0x01, 0x01, 0x00, 0x02, 0x01, 0x03, 0x01, 0x00, 0x01, 0x02, 0x41, 0x01, 0x01, 0x07,
        0x00, 0x7d, 0x01, 0x77, 0x15, 0x56, 0x39, 0x55, 0x6b, 0x39, 0x70, 0x78, 0x55, 0x4b, 0x5a,
        0x49, 0x72, 0x57, 0x36, 0x63, 0x4f, 0x6b, 0x43, 0x30, 0x52, 0x67, 0x77, 0x0c, 0x6e, 0x65,
        0x77, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x00,
    ];

    yrs::Update::decode_v2(&buffer).unwrap();
}

#[test]
fn yrs_decode_test_2() {
    use lib0::any::Any;
    use yrs::updates::decoder::Decode;

    let doc = yrs::Doc::new();
    let mut txn = doc.transact();
    let root = txn.get_map("root");

    root.insert(&mut txn, "sequence", yrs::PrelimMap::<bool>::new());
    let sequence = root.get("sequence").unwrap().to_ymap().unwrap();
    sequence.insert(&mut txn, "id", "V9Uk9pxUKZIrW6cOkC0Rg".to_string());
    sequence.insert(&mut txn, "cuts", yrs::PrelimArray::<_, Any>::from([]));
    sequence.insert(&mut txn, "name", "new sequence".to_string());

    root.insert(&mut txn, "__version__", 1);
    root.insert(
        &mut txn,
        "face_expressions",
        yrs::PrelimArray::<_, Any>::from([]),
    );
    root.insert(&mut txn, "characters", yrs::PrelimArray::<_, Any>::from([]));

    let buffer = doc.encode_state_as_update_v2(&yrs::StateVector::default());

    yrs::Update::decode_v2(&buffer).unwrap();
}

#[test]
fn no_error_1() {
    use lib0::any::Any;
    use yrs::updates::decoder::Decode;

    let doc = yrs::Doc::new();
    let mut txn = doc.transact();
    let root = txn.get_map("root");

    root.insert(&mut txn, "sequence", yrs::PrelimMap::<bool>::new());
    let sequence = root.get("sequence").unwrap().to_ymap().unwrap();
    sequence.insert(&mut txn, "id", "V9Uk9pxUKZIrW6cOkC0Rg".to_string());
    sequence.insert(&mut txn, "cuts", yrs::PrelimArray::<_, Any>::from([]));
    sequence.insert(&mut txn, "name", "new sequence".to_string());

    root.insert(&mut txn, "__version__", 1);
    root.insert(
        &mut txn,
        "face_expressions",
        yrs::PrelimArray::<_, Any>::from([]),
    );
    // root.insert(&mut txn, "characters", yrs::PrelimArray::<_, Any>::from([]));

    let buffer = doc.encode_state_as_update_v2(&yrs::StateVector::default());

    yrs::Update::decode_v2(&buffer).unwrap();
}

#[test]
fn no_error_2() {
    use lib0::any::Any;
    use yrs::updates::decoder::Decode;

    let doc = yrs::Doc::new();
    let mut txn = doc.transact();
    let root = txn.get_map("root");

    // root.insert(&mut txn, "sequence", yrs::PrelimMap::<bool>::new());
    // let sequence = root.get("sequence").unwrap().to_ymap().unwrap();
    // sequence.insert(&mut txn, "id", "V9Uk9pxUKZIrW6cOkC0Rg".to_string());
    // sequence.insert(&mut txn, "cuts", yrs::PrelimArray::<_, Any>::from([]));
    // sequence.insert(&mut txn, "name", "new sequence".to_string());

    // root.insert(&mut txn, "__version__", 1);
    // root.insert(
    //     &mut txn,
    //     "face_expressions",
    //     yrs::PrelimArray::<_, Any>::from([]),
    // );
    root.insert(&mut txn, "characters", yrs::PrelimArray::<_, Any>::from([]));

    let buffer = doc.encode_state_as_update_v2(&yrs::StateVector::default());

    yrs::Update::decode_v2(&buffer).unwrap();
}

#[test]
fn yrs_decode_test_3() {
    use lib0::any::Any;
    use yrs::updates::decoder::Decode;

    let doc = yrs::Doc::new();
    let mut txn = doc.transact();
    let root = txn.get_map("root");

    root.insert(&mut txn, "sequence", yrs::PrelimMap::<bool>::new());
    let sequence = root.get("sequence").unwrap().to_ymap().unwrap();
    sequence.insert(&mut txn, "id", "V9Uk9pxUKZIrW6cOkC0Rg".to_string());
    sequence.insert(&mut txn, "cuts", yrs::PrelimArray::<_, Any>::from([]));
    // sequence.insert(&mut txn, "name", "new sequence".to_string());

    root.insert(&mut txn, "__version__", 1);
    root.insert(
        &mut txn,
        "face_expressions",
        yrs::PrelimArray::<_, Any>::from([]),
    );
    root.insert(&mut txn, "characters", yrs::PrelimArray::<_, Any>::from([]));

    let buffer = doc.encode_state_as_update_v2(&yrs::StateVector::default());

    yrs::Update::decode_v2(&buffer).unwrap();
}

#[test]
fn yrs_test() {
    use yrs::updates::decoder::Decode;
    use yrs::updates::encoder::Encode;
    use yrs::{Doc, StateVector, Update};

    let remote_doc = Doc::new();
    let remote_encoded_state_vector = {
        let remote_txn = remote_doc.transact();
        remote_txn.state_vector().encode_v2()
    };

    let doc = Doc::new();

    {
        let mut txn = doc.transact();
        let root = txn.get_text("root-type-name");
        root.push(&mut txn, "hello world");
    };
    let encoded_update_1 = doc.encode_state_as_update_v2(
        &StateVector::decode_v2(&remote_encoded_state_vector.as_slice()).unwrap(),
    );
    println!("{:?}", encoded_update_1);

    {
        let mut txn = doc.transact();
        let root = txn.get_text("root-type-name");
        root.push(&mut txn, "my name");
    };
    let encoded_update_2 = doc.encode_state_as_update_v2(
        &StateVector::decode_v2(&remote_encoded_state_vector.as_slice()).unwrap(),
    );
    println!("{:?}", encoded_update_2);

    assert_eq!(
        "hello worldmy name",
        doc.transact()
            .get_text("root-type-name")
            .to_string()
            .as_str()
    );

    {
        let mut remote_txn = remote_doc.transact();
        remote_txn.apply_update(Update::decode_v2(encoded_update_1.as_slice()).unwrap());

        assert_eq!(
            "hello world",
            remote_txn.get_text("root-type-name").to_string().as_str()
        );
    }

    {
        let mut remote_txn = remote_doc.transact();
        remote_txn.apply_update(Update::decode_v2(encoded_update_2.as_slice()).unwrap());

        assert_eq!(
            "hello worldmy name",
            remote_txn.get_text("root-type-name").to_string().as_str()
        );
    }

    {
        let remote_doc = Doc::new();
        let mut remote_txn = remote_doc.transact();
        remote_txn.apply_update(Update::decode_v2(encoded_update_2.as_slice()).unwrap());

        assert_eq!(
            "hello worldmy name",
            remote_txn.get_text("root-type-name").to_string().as_str()
        );
    }
}
