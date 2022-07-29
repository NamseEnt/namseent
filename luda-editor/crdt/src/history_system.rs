use crate::History;
use yrs::{updates::decoder::Decode, Doc, StateVector};

pub struct HistorySystem<State> {
    phantom: std::marker::PhantomData<State>,
    doc: Doc,
    state: Option<State>,
}
impl<State: History + Clone> HistorySystem<State> {
    pub fn new(state: State) -> Self {
        let mut doc = Doc::new();
        write_state_to_doc(&mut doc, state.clone());

        HistorySystem {
            phantom: std::marker::PhantomData,
            doc,
            state: Some(state),
        }
    }
    pub fn get_state(&self) -> State {
        match self.state.clone() {
            Some(state) => state,
            None => {
                let mut txn = self.doc.transact();
                let root = txn.get_map("root");
                State::from_map(&root)
            }
        }
    }
    pub fn export_doc(&self) -> Doc {
        let encoded = self.encode();

        let doc = Doc::new();
        let mut transact = doc.transact();
        transact.apply_update(yrs::Update::decode_v2(encoded.as_ref()).unwrap());
        doc
    }
    pub fn encode(&self) -> Box<[u8]> {
        self.doc
            .encode_state_as_update_v2(&StateVector::default())
            .into_boxed_slice()
    }
    pub fn decode(encoded: impl AsRef<[u8]>) -> Self {
        let doc = Doc::new();

        let mut transact = doc.transact();
        transact.apply_update(yrs::Update::decode_v2(encoded.as_ref()).unwrap());

        let version_of_state = State::get_version();

        match version_of_state {
            Some(version_of_state) => {
                let root = transact.get_map("root");
                let version_of_doc: u32 = root
                    .get("__version__")
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap();

                if version_of_doc < version_of_state {
                    Self::new(State::migrate(version_of_doc, doc))
                } else if version_of_doc > version_of_state {
                    panic!("The version of the state is higher than the version of the doc. Please update the code.")
                } else {
                    Self {
                        phantom: std::marker::PhantomData,
                        doc,
                        state: Some(State::from_map(&root)),
                    }
                }
            }
            None => Self {
                phantom: std::marker::PhantomData,
                doc,
                state: None,
            },
        }
    }
    pub fn mutate<F>(&mut self, f: F) -> Box<[u8]>
    where
        F: FnOnce(&mut State),
    {
        let mut state = self.get_state();
        f(&mut state);

        let mut txn = self.doc.transact();
        let mut root = txn.get_map("root");

        state.update_to_map(&mut txn, &mut root);

        let encoded_update = txn.encode_update_v2();
        self.state = Some(State::from_map(&root));

        encoded_update.into_boxed_slice()
    }
    pub fn merge(&mut self, encoded: impl AsRef<[u8]>) {
        let decoded = Self::decode(encoded);

        let mut transact = decoded.doc.transact();
        transact.apply_update(yrs::Update::decode_v2(self.encode().as_ref()).unwrap());

        *self = decoded;
        self.state = Some(State::from_map(&transact.get_map("root")));
    }
    pub fn merge_update(&mut self, encoded_update: impl AsRef<[u8]>) {}
}

fn write_state_to_doc(doc: &mut Doc, state: impl History) {
    let mut txn = doc.transact();

    state.insert_to_root(&mut txn);
}
