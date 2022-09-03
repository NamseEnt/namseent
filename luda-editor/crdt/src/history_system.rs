use crate::History;
use yrs::{
    updates::{decoder::Decode, encoder::Encode},
    Doc, StateVector,
};

pub struct HistorySystem<State> {
    phantom: std::marker::PhantomData<State>,
    pub doc: Doc,
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
    pub fn encode(&self) -> impl AsRef<[u8]> {
        self.doc.encode_state_as_update_v2(&StateVector::default())
    }
    pub fn encode_against_state_vector(&self, state_vector: impl AsRef<[u8]>) -> impl AsRef<[u8]> {
        self.doc
            .encode_state_as_update_v2(&StateVector::decode_v2(state_vector.as_ref()).unwrap())
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

    /// NOTE: This code is unstable so would make bug.
    pub fn merge(&mut self, encoded: impl AsRef<[u8]>) {
        let version_before_merge = self.get_version();

        let mut transact = self.doc.transact();
        transact.apply_update(yrs::Update::decode_v2(encoded.as_ref()).unwrap());

        let version_after_merge = self.get_version();

        if version_before_merge < version_after_merge {
            let doc = std::mem::take(&mut self.doc);
            *self = Self::new(State::migrate(version_before_merge, doc));
            self.merge(encoded);
        } else if version_before_merge > version_after_merge {
            panic!("The version before merge is higher than the version after merge. Please update the code.")
        } else {
            self.state = Some(State::from_map(&transact.get_map("root")));
        }
    }
    pub fn state_vector(&self) -> Vec<u8> {
        self.doc.transact().state_vector().encode_v2()
    }
    pub fn default_state_vector() -> Vec<u8> {
        StateVector::default().encode_v2()
    }
    fn get_version(&self) -> u32 {
        let root = self.doc.transact().get_map("root");
        let version_of_doc: u32 = root
            .get("__version__")
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        version_of_doc
    }
}

fn write_state_to_doc(doc: &mut Doc, state: impl History) {
    let mut txn = doc.transact();

    state.insert_to_root(&mut txn);
}
