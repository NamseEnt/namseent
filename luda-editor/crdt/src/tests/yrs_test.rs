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
