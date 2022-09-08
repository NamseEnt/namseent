use crate as crdt;
use crdt::*;

#[allow(unused_imports, dead_code)]
pub mod system_tree_0 {
    use crate as crdt;
    use crate::*;
    use serde::*;

    #[history(version = 0)]
    #[derive(PartialEq)]
    pub struct SystemTree {
        pub sequence: Single<Sequence>,
    }
    impl SystemTree {
        pub fn new(name: String) -> Self {
            Self {
                sequence: Single::new(Sequence::new(name)),
            }
        }
    }

    #[history]
    #[derive(PartialEq)]
    pub struct Sequence {
        id: String,
        pub name: String,
        pub cuts: List<Cut>,
    }
    impl Sequence {
        pub fn new(name: String) -> Self {
            Self {
                id: "0".to_string(),
                name,
                cuts: List::new([]),
            }
        }
        pub fn id(&self) -> &str {
            &self.id
        }
    }

    #[history]
    #[derive(PartialEq)]
    pub struct Cut {
        id: String,
        pub image_clips: List<ImageClip>,
        /// The text that the character speaks in this cut.
        pub line: String,
    }

    impl Cut {
        pub fn new() -> Self {
            Self {
                id: "0".to_string(),
                image_clips: List::new([]),
                line: String::new(),
            }
        }
        pub fn id(&self) -> &str {
            &self.id
        }
    }

    #[history]
    #[derive(PartialEq)]
    pub struct ImageClip {
        id: String,
        pub images: List<Image>,
    }

    #[allow(dead_code)]
    impl ImageClip {
        pub fn new() -> Self {
            Self {
                id: "0".to_string(),
                images: List::new([]),
            }
        }
        pub fn id(&self) -> &str {
            &self.id
        }
    }

    #[history]
    #[derive(PartialEq)]
    pub struct Image {
        id: String,
        pub image_path: Option<String>,
        /// against the screen size
        pub circumscribed: Circumscribed,
    }

    #[allow(dead_code)]
    impl Image {
        pub fn new(image_path: Option<String>, circumscribed: Circumscribed) -> Self {
            Self {
                id: "0".to_string(),
                image_path,
                circumscribed,
            }
        }
        pub fn id(&self) -> &str {
            &self.id
        }
    }

    #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
    pub struct Circumscribed {
        pub center: (f32, f32),
        pub radius: f32,
    }
}

#[test]
fn system_tree_case_0() {
    let history_system: HistorySystem<system_tree_0::SystemTree> =
        HistorySystem::new(system_tree_0::SystemTree {
            sequence: Single::new(system_tree_0::Sequence::new("0".to_string())),
        });

    let version_0_encode = history_system.encode();

    let history_system = HistorySystem::decode(&version_0_encode);

    assert_eq!(
        system_tree_0::SystemTree {
            sequence: Single::new(system_tree_0::Sequence::new("0".to_string())),
        },
        history_system.get_state()
    );
}
