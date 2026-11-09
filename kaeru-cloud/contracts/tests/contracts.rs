use contracts::{Action, ActionMask, SecretToken};

#[test]
fn action_ids_and_unknown_masks_are_checked_on_the_wire() {
    for action in Action::ALL {
        let mut bytes = Vec::new();
        ciborium::into_writer(&action, &mut bytes).unwrap();
        let id: u8 = ciborium::from_reader(bytes.as_slice()).unwrap();
        assert_eq!(id, action as u8);
        let decoded: Action = ciborium::from_reader(bytes.as_slice()).unwrap();
        assert_eq!(decoded, action);
    }
    let mut unknown = Vec::new();
    ciborium::into_writer(&63u64, &mut unknown).unwrap();
    assert!(ciborium::from_reader::<Action, _>(unknown.as_slice()).is_err());
    let mut bits = Vec::new();
    ciborium::into_writer(&(1u64 << 63), &mut bits).unwrap();
    assert!(ciborium::from_reader::<ActionMask, _>(bits.as_slice()).is_err());
}

#[test]
fn credentials_are_redacted_in_debug_output() {
    let value = SecretToken::new("synthetic-secret".into());
    assert_eq!(format!("{value:?}"), "[REDACTED]");
}
