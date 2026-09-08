use std::ops::Deref;

pub(crate) struct HeadedRawCoreState {
    session: td_core::CoreSession,
}

impl Clone for HeadedRawCoreState {
    fn clone(&self) -> Self {
        Self::new(self.state().clone())
    }
}

impl HeadedRawCoreState {
    pub(crate) fn new(state: td_core::CoreState) -> Self {
        Self {
            session: td_core::CoreSession::from_unvalidated_state(state),
        }
    }

    pub(crate) fn state(&self) -> &td_core::CoreState {
        self.session.raw_state()
    }

    #[cfg(test)]
    pub(crate) fn snapshot(
        &self,
    ) -> Result<td_core::CoreSnapshot, td_core::SnapshotValidationError> {
        self.session.snapshot()
    }

    pub(crate) fn apply(
        &mut self,
        command: td_core::PlayerCommand,
    ) -> Result<td_core::CommandReceipt, td_core::CommandError> {
        self.assert_valid();
        self.session.apply(command)
    }

    pub(crate) fn advance_tick_with_events(&mut self) -> td_core::TickEventsOutput {
        self.assert_valid();
        let output = self.session.advance_tick_with_events();
        self.session.extend_events(output.events.clone());
        output
    }

    #[cfg(test)]
    pub(crate) fn record_compatibility_command(
        &mut self,
        command: td_core::PlayerCommand,
    ) -> td_core::CommandReceipt {
        self.assert_valid();
        self.session.record_compatibility_command(command)
    }

    pub(crate) fn drain_events(&mut self) -> std::vec::Drain<'_, td_core::CoreEvent> {
        self.session.drain_events()
    }

    pub(crate) fn extend_events(&mut self, events: impl IntoIterator<Item = td_core::CoreEvent>) {
        self.session.extend_events(events);
    }

    #[cfg(test)]
    pub(crate) fn edit_snapshot(
        &mut self,
        edit: impl FnOnce(&mut td_core::CoreSnapshotParts),
    ) -> Result<(), td_core::SnapshotValidationError> {
        self.session.edit_snapshot(edit)
    }

    fn assert_valid(&self) {
        assert!(
            self.state().validate_snapshot(),
            "headed raw core state must be valid"
        );
    }
}

impl Deref for HeadedRawCoreState {
    type Target = td_core::CoreState;

    fn deref(&self) -> &Self::Target {
        self.state()
    }
}

impl namui::bincode::Encode for HeadedRawCoreState {
    fn encode<__E: namui::bincode::enc::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> Result<(), namui::bincode::error::EncodeError> {
        serde_json::to_vec(self.state())
            .expect("headed raw core state must be serializable")
            .encode(encoder)
    }
}

impl namui::bincode::Decode<()> for HeadedRawCoreState {
    fn decode<__D: namui::bincode::de::Decoder<Context = ()>>(
        decoder: &mut __D,
    ) -> Result<Self, namui::bincode::error::DecodeError> {
        let bytes = Vec::<u8>::decode(decoder)?;
        serde_json::from_slice(&bytes)
            .map(Self::new)
            .map_err(|error| namui::bincode::error::DecodeError::OtherString(error.to_string()))
    }
}

impl namui::Serialize for HeadedRawCoreState {
    fn serialize(&self, buf: &mut Vec<u8>) {
        serde_json::to_vec(self.state())
            .expect("headed raw core state must be serializable")
            .serialize(buf);
    }

    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        serde_json::to_vec(self.state())
            .expect("headed raw core state must be serializable")
            .serialize_without_name(buf);
    }
}

impl namui::Deserialize for HeadedRawCoreState {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, namui::DeserializeError> {
        let bytes = Vec::<u8>::deserialize(buf)?;
        serde_json::from_slice(&bytes)
            .map(Self::new)
            .map_err(|error| namui::DeserializeError::InvalidEnumVariant {
                expected: "valid headed raw core state".to_string(),
                actual: error.to_string(),
            })
    }

    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, namui::DeserializeError> {
        let bytes = Vec::<u8>::deserialize_without_name(buf)?;
        serde_json::from_slice(&bytes)
            .map(Self::new)
            .map_err(|error| namui::DeserializeError::InvalidEnumVariant {
                expected: "valid headed raw core state".to_string(),
                actual: error.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_core_codec_round_trips_authoritative_hash_and_bytes() {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        let original = game_state.raw_core.clone();
        let original_hash = td_core::authoritative_hash(original.state());
        assert_eq!(
            original.snapshot().expect("raw core snapshot").as_state(),
            original.state()
        );
        let bytes = namui::bincode::encode_to_vec(&original, namui::bincode::config::standard())
            .expect("raw core encoding");
        let (restored, consumed): (HeadedRawCoreState, usize) =
            namui::bincode::decode_from_slice(&bytes, namui::bincode::config::standard())
                .expect("raw core decoding");
        let restored_bytes =
            namui::bincode::encode_to_vec(&restored, namui::bincode::config::standard())
                .expect("restored raw core encoding");
        let mut named_bytes = Vec::new();
        namui::Serialize::serialize(&original, &mut named_bytes);
        let mut named_input = named_bytes.as_slice();
        let named_restored: HeadedRawCoreState =
            namui::Deserialize::deserialize(&mut named_input).expect("raw core named decoding");
        let mut unnamed_bytes = Vec::new();
        namui::Serialize::serialize_without_name(&original, &mut unnamed_bytes);
        let mut unnamed_input = unnamed_bytes.as_slice();
        let unnamed_restored: HeadedRawCoreState =
            namui::Deserialize::deserialize_without_name(&mut unnamed_input)
                .expect("raw core unnamed decoding");

        assert_eq!(consumed, bytes.len());
        assert_eq!(td_core::authoritative_hash(restored.state()), original_hash);
        assert_eq!(restored_bytes, bytes);
        assert!(named_input.is_empty());
        assert!(unnamed_input.is_empty());
        assert_eq!(
            td_core::authoritative_hash(named_restored.state()),
            original_hash
        );
        assert_eq!(
            td_core::authoritative_hash(unnamed_restored.state()),
            original_hash
        );
    }

    #[test]
    fn headed_adapter_clone_session_baseline() {
        let initial = crate::game_state::create_game_state_with_seed(7)
            .raw_core
            .state()
            .clone();
        let mut replay_heavy =
            td_core::CoreSession::from_state(initial.clone()).expect("initial state must be valid");
        for _ in 0..128 {
            replay_heavy.record_compatibility_command(td_core::PlayerCommand::StartDefense);
        }
        let replay_heavy = replay_heavy.into_state();
        let mut event_heavy = initial.clone();
        event_heavy.extend_events((0..256).map(|id| td_core::CoreEvent::DamageApplied {
            target_id: id,
            amount: 100,
            position: [id as i64, id as i64],
        }));
        let cases = [
            ("initial", initial),
            ("replay_128_commands", replay_heavy),
            ("event_queue_256", event_heavy),
        ];
        let iterations = 128;

        for (label, state) in cases {
            let state_bytes = serde_json::to_vec(&state)
                .expect("baseline state serialization")
                .len();
            let started = std::time::Instant::now();
            let mut clone_tick_sum = 0u64;
            for _ in 0..iterations {
                let cloned = std::hint::black_box(state.clone());
                clone_tick_sum = clone_tick_sum.wrapping_add(cloned.sim_tick().ticks());
            }
            let clone_elapsed = started.elapsed();

            let started = std::time::Instant::now();
            let mut recreation_tick_sum = 0u64;
            for _ in 0..iterations {
                let session = td_core::CoreSession::from_state(std::hint::black_box(state.clone()))
                    .expect("baseline state must be valid");
                recreation_tick_sum = recreation_tick_sum.wrapping_add(session.sim_tick().ticks());
            }
            let recreation_elapsed = started.elapsed();

            assert_eq!(clone_tick_sum, recreation_tick_sum);
            eprintln!(
                "headed_raw_core_baseline label={label} state_bytes={state_bytes} \
                 iterations={iterations} clone_ns_per_iter={} \
                 clone_plus_session_ns_per_iter={} session_validation_ns_per_iter={}",
                clone_elapsed.as_nanos() / iterations,
                recreation_elapsed.as_nanos() / iterations,
                recreation_elapsed.saturating_sub(clone_elapsed).as_nanos() / iterations,
            );
        }
    }
}
