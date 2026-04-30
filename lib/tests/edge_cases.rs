use brarchive::SerializeOptions;
use std::collections::BTreeMap;

// --- Programmatic edge cases ---

#[test]
fn round_trip_empty_archive() {
    let empty: Vec<(String, String)> = vec![];
    let bytes = brarchive::serialize(empty).unwrap();
    assert_eq!(bytes.len(), 16);
    let result: BTreeMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert!(result.is_empty());
}

#[test]
fn round_trip_zero_length_entry() {
    let data = vec![("stub.json".to_string(), String::new())];
    let bytes = brarchive::serialize(data).unwrap();
    let result: BTreeMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(result["stub.json"], "");
}

#[test]
fn dedup_round_trip_with_real_names() {
    let data = vec![
        (
            "entity/zombie.json".to_string(),
            r#"{"id":"zombie"}"#.to_string(),
        ),
        (
            "entity/skeleton.json".to_string(),
            r#"{"id":"zombie"}"#.to_string(),
        ),
        (
            "entity/creeper.json".to_string(),
            r#"{"id":"creeper"}"#.to_string(),
        ),
    ];
    let bytes =
        brarchive::serialize_with(data.clone(), SerializeOptions { dedup: true }).unwrap();
    let result: BTreeMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(result["entity/zombie.json"], r#"{"id":"zombie"}"#);
    assert_eq!(result["entity/skeleton.json"], r#"{"id":"zombie"}"#);
    assert_eq!(result["entity/creeper.json"], r#"{"id":"creeper"}"#);
}

#[test]
fn round_trip_single_entry() {
    let data = vec![("only.json".to_string(), "hello".to_string())];
    let bytes = brarchive::serialize(data).unwrap();
    let result: BTreeMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result["only.json"], "hello");
}

#[test]
fn dedup_produces_smaller_output_for_identical_values() {
    let data = vec![
        ("a.json".to_string(), "x".repeat(512)),
        ("b.json".to_string(), "x".repeat(512)),
        ("c.json".to_string(), "x".repeat(512)),
    ];
    let without = brarchive::serialize(data.clone()).unwrap();
    let with_dedup =
        brarchive::serialize_with(data, SerializeOptions { dedup: true }).unwrap();
    assert!(with_dedup.len() < without.len());
}

// --- Error-path tests ---

#[test]
fn deserialize_bad_magic_returns_error() {
    let mut bytes = brarchive::serialize([("k", "v")]).unwrap();
    bytes[0] ^= 0xFF;
    let result: Result<BTreeMap<String, String>, _> = brarchive::deserialize(&bytes);
    assert!(matches!(result, Err(brarchive::error::BrArchiveError::MagicMismatch(_))));
}

#[test]
fn deserialize_bad_version_returns_error() {
    let mut bytes = brarchive::serialize([("k", "v")]).unwrap();
    // Version is at bytes 12-15 (u32 LE). Set it to 999.
    bytes[12] = 0xE7;
    bytes[13] = 0x03;
    bytes[14] = 0x00;
    bytes[15] = 0x00;
    let result: Result<BTreeMap<String, String>, _> = brarchive::deserialize(&bytes);
    assert!(matches!(result, Err(brarchive::error::BrArchiveError::UnsupportedVersion(999))));
}

#[test]
fn serialize_name_too_long_returns_error() {
    let long_name = "a".repeat(248);
    let result = brarchive::serialize([(long_name.as_str(), "v")]);
    assert!(matches!(result, Err(brarchive::error::BrArchiveError::EntryNameTooLong(_))));
}

#[test]
fn serialize_name_at_max_length_succeeds() {
    let max_name = "a".repeat(247);
    let bytes = brarchive::serialize([(max_name.as_str(), "v")]).unwrap();
    let result: BTreeMap<String, String> = brarchive::deserialize(&bytes).unwrap();
    assert_eq!(result[&max_name], "v");
}

// --- Fixture-based tests ---

/// ddui.brarchive is 16 bytes — an empty archive (0 entries).
#[test]
fn fixture_ddui_is_empty_archive() {
    let bytes = include_bytes!("fixtures/ddui.brarchive");
    assert_eq!(bytes.len(), 16, "ddui.brarchive should be exactly 16 bytes");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert!(result.is_empty());
}

/// models.brarchive is 16 bytes — an empty archive (0 entries).
#[test]
fn fixture_models_is_empty_archive() {
    let bytes = include_bytes!("fixtures/models.brarchive");
    assert_eq!(bytes.len(), 16, "models.brarchive should be exactly 16 bytes");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert!(result.is_empty());
}

/// sounds.brarchive contains exactly 1 entry: sound_definitions.json.
#[test]
fn fixture_sounds_has_one_entry() {
    let bytes = include_bytes!("fixtures/sounds.brarchive");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result.contains_key("sound_definitions.json"));
}

/// textures.brarchive contains exactly 2 entries: item_texture.json and texture_list.json.
#[test]
fn fixture_textures_has_two_entries() {
    let bytes = include_bytes!("fixtures/textures.brarchive");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains_key("item_texture.json"));
    assert!(result.contains_key("texture_list.json"));
}

#[test]
fn fixture_animation_controllers_deserializes() {
    let bytes = include_bytes!("fixtures/animation_controllers.brarchive");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert_eq!(result.len(), 4);
    assert!(result.contains_key("humanoid.animation_controllers.json"));
}

#[test]
fn fixture_animations_deserializes() {
    let bytes = include_bytes!("fixtures/animations.brarchive");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert_eq!(result.len(), 5);
    assert!(result.contains_key("humanoid.animation.json"));
}

#[test]
fn fixture_attachables_deserializes() {
    let bytes = include_bytes!("fixtures/attachables.brarchive");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert_eq!(result.len(), 7);
    assert!(result.contains_key("copper_spear.entity.json"));
}

#[test]
fn fixture_entity_deserializes() {
    let bytes = include_bytes!("fixtures/entity.brarchive");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert_eq!(result.len(), 13);
    assert!(result.contains_key("camel_husk.entity.json"));
}

#[test]
fn fixture_particles_deserializes() {
    let bytes = include_bytes!("fixtures/particles.brarchive");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result.contains_key("nautilus_bubbles.json"));
}

#[test]
fn fixture_render_controllers_deserializes() {
    let bytes = include_bytes!("fixtures/render_controllers.brarchive");
    let result: BTreeMap<String, String> = brarchive::deserialize(bytes).unwrap();
    assert_eq!(result.len(), 4);
    assert!(result.contains_key("horse_v3.render_controllers.json"));
}
