use std::collections::BTreeMap;

#[test]
fn brarchive_items() {
    let archive_result: BTreeMap<String, Vec<u8>> =
        brarchive::deserialize(include_bytes!("items.brarchive")).unwrap();
    let archive_result: BTreeMap<String, String> = archive_result
        .into_iter()
        .map(|(k, v)| (k, String::from_utf8(v).unwrap()))
        .collect();

    let archive_correct = BTreeMap::from([(
        "appleEnchanted.json".to_string(),
        r#"{"format_version":"1.10","minecraft:item":{"description":{"identifier":"minecraft:appleEnchanted"},"components":{"minecraft:hand_equipped":false,"minecraft:stacked_by_data":true,"minecraft:use_duration":32,"minecraft:foil":true,"minecraft:food":{"nutrition":4,"saturation_modifier":"supernatural","can_always_eat":true,"effects":[{"name":"regeneration","chance":1.0,"duration":30,"amplifier":1},{"name":"absorption","chance":1.0,"duration":120,"amplifier":3},{"name":"resistance","chance":1.0,"duration":300,"amplifier":0},{"name":"fire_resistance","chance":1.0,"duration":300,"amplifier":0}]}}}}"#.to_string(),
    )]);

    assert_eq!(archive_result, archive_correct);
}
