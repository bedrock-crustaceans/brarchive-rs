use std::collections::BTreeMap;

#[test]
fn brarchive_recipe() {
    let archive_result = brarchive::deserialize(include_bytes!("recipes.brarchive"));

    let archive_correct = Ok(BTreeMap::from([(
        "acacia_boat.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:acacia_boat"},"tags":["crafting_table"],"pattern":["# #","###"],"key":{"#":{"item":"minecraft:acacia_planks"}},"unlock":{"context":"PlayerInWater"},"result":{"item":"minecraft:acacia_boat"}}}"####.to_string()
    ), (
        "acacia_door.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:acacia_door"},"tags":["crafting_table"],"group":"wooden_door","pattern":["##","##","##"],"key":{"#":{"item":"minecraft:acacia_planks"}},"unlock":[{"item":"minecraft:acacia_planks"}],"result":{"item":"minecraft:acacia_door","count":3}}}"####.to_string()
    ), (
        "acacia_fence.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:acacia_fence"},"tags":["crafting_table"],"pattern":["W#W","W#W"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:acacia_planks"}},"unlock":[{"item":"minecraft:acacia_planks"}],"result":{"item":"minecraft:acacia_fence","count":3}}}"####.to_string()
    ), (
        "acacia_fence_gate.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:acacia_fence_gate"},"tags":["crafting_table"],"group":"wooden_fence_gate","pattern":["#W#","#W#"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:acacia_planks"}},"unlock":[{"item":"minecraft:acacia_planks"}],"result":{"item":"minecraft:acacia_fence_gate"}}}"####.to_string()
    ), (
        "acacia_planks.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:acacia_planks"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:acacia_log"}},"unlock":[{"item":"minecraft:acacia_log"}],"result":{"item":"minecraft:acacia_planks","count":4}}}"####.to_string()
    ), (
        "acacia_planks_from_stripped.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:acacia_planks_from_stripped"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:stripped_acacia_log"}},"unlock":[{"item":"minecraft:stripped_acacia_log"}],"result":{"item":"minecraft:acacia_planks","count":4}}}"####.to_string()
    ), (
        "acacia_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:acacia_stairs"},"tags":["crafting_table"],"group":"wooden_stairs","pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:acacia_planks"}},"unlock":[{"item":"minecraft:acacia_planks"}],"result":{"item":"minecraft:acacia_stairs","count":4}}}"####.to_string()
    ), (
        "acacia_wooden_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:acacia_wooden_slab"},"tags":["crafting_table"],"group":"wooden_slab","pattern":["###"],"key":{"#":{"item":"minecraft:acacia_planks"}},"unlock":[{"item":"minecraft:acacia_planks"}],"result":{"item":"minecraft:acacia_slab","count":6}}}"####.to_string()
    ), (
        "andesite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:andesite"},"tags":["crafting_table"],"ingredients":[{"item":"minecraft:diorite"},{"item":"minecraft:cobblestone"}],"unlock":[{"item":"minecraft:diorite"}],"result":{"item":"minecraft:andesite","count":2}}}"####.to_string()
    ), (
        "andesite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:andesite_stairs"},"tags":["crafting_table"],"pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:andesite"}},"unlock":[{"item":"minecraft:andesite"}],"result":{"item":"minecraft:andesite_stairs","count":4}}}"####.to_string()
    ), (
        "birch_boat.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:birch_boat"},"tags":["crafting_table"],"pattern":["# #","###"],"key":{"#":{"item":"minecraft:birch_planks"}},"unlock":{"context":"PlayerInWater"},"result":{"item":"minecraft:birch_boat"}}}"####.to_string()
    ), (
        "birch_door.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:birch_door"},"tags":["crafting_table"],"group":"wooden_door","pattern":["##","##","##"],"key":{"#":{"item":"minecraft:birch_planks"}},"unlock":[{"item":"minecraft:birch_planks"}],"result":{"item":"minecraft:birch_door","count":3}}}"####.to_string()
    ), (
        "birch_fence.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:birch_fence"},"tags":["crafting_table"],"pattern":["W#W","W#W"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:birch_planks"}},"unlock":[{"item":"minecraft:birch_planks"}],"result":{"item":"minecraft:birch_fence","count":3}}}"####.to_string()
    ), (
        "birch_fence_gate.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:birch_fence_gate"},"tags":["crafting_table"],"group":"wooden_fence_gate","pattern":["#W#","#W#"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:birch_planks"}},"unlock":[{"item":"minecraft:birch_planks"}],"result":{"item":"minecraft:birch_fence_gate"}}}"####.to_string()
    ), (
        "birch_planks.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:birch_planks"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:birch_log"}},"unlock":[{"item":"minecraft:birch_log"}],"result":{"item":"minecraft:birch_planks","count":4}}}"####.to_string()
    ), (
        "birch_planks_from_stripped.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:birch_planks_from_stripped"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:stripped_birch_log"}},"unlock":[{"item":"minecraft:stripped_birch_log"}],"result":{"item":"minecraft:birch_planks","count":4}}}"####.to_string()
    ), (
        "birch_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:birch_stairs"},"tags":["crafting_table"],"group":"wooden_stairs","pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:birch_planks"}},"unlock":[{"item":"minecraft:birch_planks"}],"result":{"item":"minecraft:birch_stairs","count":4}}}"####.to_string()
    ), (
        "birch_wooden_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:birch_wooden_slab"},"tags":["crafting_table"],"group":"wooden_slab","pattern":["###"],"key":{"#":{"item":"minecraft:birch_planks"}},"unlock":[{"item":"minecraft:birch_planks"}],"result":{"item":"minecraft:birch_slab","count":6}}}"####.to_string()
    ), (
        "boat.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:boat"},"tags":["crafting_table"],"pattern":["# #","###"],"key":{"#":{"item":"minecraft:oak_planks"}},"unlock":{"context":"PlayerInWater"},"result":{"item":"minecraft:oak_boat"}}}"####.to_string()
    ), (
        "dark_oak_boat.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_oak_boat"},"tags":["crafting_table"],"pattern":["# #","###"],"key":{"#":{"item":"minecraft:dark_oak_planks"}},"unlock":{"context":"PlayerInWater"},"result":{"item":"minecraft:dark_oak_boat"}}}"####.to_string()
    ), (
        "dark_oak_door.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_oak_door"},"tags":["crafting_table"],"group":"wooden_door","pattern":["##","##","##"],"key":{"#":{"item":"minecraft:dark_oak_planks"}},"unlock":[{"item":"minecraft:dark_oak_planks"}],"result":{"item":"minecraft:dark_oak_door","count":3}}}"####.to_string()
    ), (
        "dark_oak_fence.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_oak_fence"},"tags":["crafting_table"],"pattern":["W#W","W#W"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:dark_oak_planks"}},"unlock":[{"item":"minecraft:dark_oak_planks"}],"result":{"item":"minecraft:dark_oak_fence","count":3}}}"####.to_string()
    ), (
        "dark_oak_fence_gate.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_oak_fence_gate"},"tags":["crafting_table"],"group":"wooden_fence_gate","pattern":["#W#","#W#"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:dark_oak_planks"}},"unlock":[{"item":"minecraft:dark_oak_planks"}],"result":{"item":"minecraft:dark_oak_fence_gate"}}}"####.to_string()
    ), (
        "dark_oak_planks.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_oak_planks"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:dark_oak_log"}},"unlock":[{"item":"minecraft:dark_oak_log"}],"result":{"item":"minecraft:dark_oak_planks","count":4}}}"####.to_string()
    ), (
        "dark_oak_planks_from_stripped.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_oak_planks_from_stripped"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:stripped_dark_oak_log"}},"unlock":[{"item":"minecraft:stripped_dark_oak_log"}],"result":{"item":"minecraft:dark_oak_planks","count":4}}}"####.to_string()
    ), (
        "dark_oak_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_oak_stairs"},"tags":["crafting_table"],"group":"wooden_stairs","pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:dark_oak_planks"}},"unlock":[{"item":"minecraft:dark_oak_planks"}],"result":{"item":"minecraft:dark_oak_stairs","count":4}}}"####.to_string()
    ), (
        "dark_oak_wooden_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_oak_wooden_slab"},"tags":["crafting_table"],"group":"wooden_slab","pattern":["###"],"key":{"#":{"item":"minecraft:dark_oak_planks"}},"unlock":[{"item":"minecraft:dark_oak_planks"}],"result":{"item":"minecraft:dark_oak_slab","count":6}}}"####.to_string()
    ), (
        "dark_prismarine.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_prismarine"},"tags":["crafting_table"],"pattern":["SSS","SIS","SSS"],"key":{"S":{"item":"minecraft:prismarine_shard"},"I":{"item":"minecraft:black_dye"}},"unlock":[{"item":"minecraft:prismarine_shard"}],"result":{"item":"minecraft:dark_prismarine"}}}"####.to_string()
    ), (
        "dark_prismarine_from_ink_sac.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:dark_prismarine_from_ink_sac"},"tags":["crafting_table"],"priority":1,"pattern":["SSS","SIS","SSS"],"key":{"S":{"item":"minecraft:prismarine_shard"},"I":{"item":"minecraft:ink_sac"}},"unlock":[{"item":"minecraft:prismarine_shard"}],"result":{"item":"minecraft:dark_prismarine"}}}"####.to_string()
    ), (
        "diorite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:diorite"},"tags":["crafting_table"],"pattern":["CQ","QC"],"key":{"Q":{"item":"minecraft:quartz"},"C":{"item":"minecraft:cobblestone"}},"unlock":[{"item":"minecraft:quartz"}],"result":{"item":"minecraft:diorite","count":2}}}"####.to_string()
    ), (
        "diorite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:diorite_stairs"},"tags":["crafting_table"],"pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:diorite"}},"unlock":[{"item":"minecraft:diorite"}],"result":{"item":"minecraft:diorite_stairs","count":4}}}"####.to_string()
    ), (
        "fence.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:fence"},"tags":["crafting_table"],"pattern":["W#W","W#W"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:oak_planks"}},"unlock":[{"item":"minecraft:oak_planks"}],"result":{"item":"minecraft:oak_fence","count":3}}}"####.to_string()
    ), (
        "fence_gate.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:fence_gate"},"tags":["crafting_table"],"group":"wooden_fence_gate","pattern":["#W#","#W#"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:oak_planks"}},"unlock":[{"item":"minecraft:oak_planks"}],"result":{"item":"minecraft:fence_gate"}}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay0.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay0"},"unlock":[{"item":"minecraft:white_terracotta"}],"tags":["furnace"],"input":"minecraft:white_terracotta","output":"minecraft:white_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay1.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay1"},"unlock":[{"item":"minecraft:orange_terracotta"}],"tags":["furnace"],"input":"minecraft:orange_terracotta","output":"minecraft:orange_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay10.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay10"},"unlock":[{"item":"minecraft:purple_terracotta"}],"tags":["furnace"],"input":"minecraft:purple_terracotta","output":"minecraft:purple_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay11.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay11"},"unlock":[{"item":"minecraft:blue_terracotta"}],"tags":["furnace"],"input":"minecraft:blue_terracotta","output":"minecraft:blue_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay12.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay12"},"unlock":[{"item":"minecraft:brown_terracotta"}],"tags":["furnace"],"input":"minecraft:brown_terracotta","output":"minecraft:brown_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay13.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay13"},"unlock":[{"item":"minecraft:green_terracotta"}],"tags":["furnace"],"input":"minecraft:green_terracotta","output":"minecraft:green_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay14.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay14"},"unlock":[{"item":"minecraft:red_terracotta"}],"tags":["furnace"],"input":"minecraft:red_terracotta","output":"minecraft:red_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay15.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay15"},"unlock":[{"item":"minecraft:black_terracotta"}],"tags":["furnace"],"input":"minecraft:black_terracotta","output":"minecraft:black_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay2.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay2"},"unlock":[{"item":"minecraft:magenta_terracotta"}],"tags":["furnace"],"input":"minecraft:magenta_terracotta","output":"minecraft:magenta_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay3.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay3"},"unlock":[{"item":"minecraft:light_blue_terracotta"}],"tags":["furnace"],"input":"minecraft:light_blue_terracotta","output":"minecraft:light_blue_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay4.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay4"},"unlock":[{"item":"minecraft:yellow_terracotta"}],"tags":["furnace"],"input":"minecraft:yellow_terracotta","output":"minecraft:yellow_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay5.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay5"},"unlock":[{"item":"minecraft:lime_terracotta"}],"tags":["furnace"],"input":"minecraft:lime_terracotta","output":"minecraft:lime_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay6.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay6"},"unlock":[{"item":"minecraft:pink_terracotta"}],"tags":["furnace"],"input":"minecraft:pink_terracotta","output":"minecraft:pink_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay7.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay7"},"unlock":[{"item":"minecraft:gray_terracotta"}],"tags":["furnace"],"input":"minecraft:gray_terracotta","output":"minecraft:gray_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay8.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay8"},"unlock":[{"item":"minecraft:light_gray_terracotta"}],"tags":["furnace"],"input":"minecraft:light_gray_terracotta","output":"minecraft:silver_glazed_terracotta"}}"####.to_string()
    ), (
        "furnace_stained_hardened_clay9.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_furnace":{"description":{"identifier":"minecraft:furnace_stained_hardened_clay9"},"unlock":[{"item":"minecraft:cyan_terracotta"}],"tags":["furnace"],"input":"minecraft:cyan_terracotta","output":"minecraft:cyan_glazed_terracotta"}}"####.to_string()
    ), (
        "granite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:granite"},"tags":["crafting_table"],"ingredients":[{"item":"minecraft:diorite"},{"item":"minecraft:quartz"}],"unlock":[{"item":"minecraft:quartz"}],"result":{"item":"minecraft:granite"}}}"####.to_string()
    ), (
        "granite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:granite_stairs"},"tags":["crafting_table"],"pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:granite"}},"unlock":[{"item":"minecraft:granite"}],"result":{"item":"minecraft:granite_stairs","count":4}}}"####.to_string()
    ), (
        "jungle_boat.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:jungle_boat"},"tags":["crafting_table"],"pattern":["# #","###"],"key":{"#":{"item":"minecraft:jungle_planks"}},"unlock":{"context":"PlayerInWater"},"result":{"item":"minecraft:jungle_boat"}}}"####.to_string()
    ), (
        "jungle_door.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:jungle_door"},"tags":["crafting_table"],"group":"wooden_door","pattern":["##","##","##"],"key":{"#":{"item":"minecraft:jungle_planks"}},"unlock":[{"item":"minecraft:jungle_planks"}],"result":{"item":"minecraft:jungle_door","count":3}}}"####.to_string()
    ), (
        "jungle_fence.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:jungle_fence"},"tags":["crafting_table"],"pattern":["W#W","W#W"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:jungle_planks"}},"unlock":[{"item":"minecraft:jungle_planks"}],"result":{"item":"minecraft:jungle_fence","count":3}}}"####.to_string()
    ), (
        "jungle_fence_gate.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:jungle_fence_gate"},"tags":["crafting_table"],"group":"wooden_fence_gate","pattern":["#W#","#W#"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:jungle_planks"}},"unlock":[{"item":"minecraft:jungle_planks"}],"result":{"item":"minecraft:jungle_fence_gate"}}}"####.to_string()
    ), (
        "jungle_planks.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:jungle_planks"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:jungle_log"}},"unlock":[{"item":"minecraft:jungle_log"}],"result":{"item":"minecraft:jungle_planks","count":4}}}"####.to_string()
    ), (
        "jungle_planks_from_stripped.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:jungle_planks_from_stripped"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:stripped_jungle_log"}},"unlock":[{"item":"minecraft:stripped_jungle_log"}],"result":{"item":"minecraft:jungle_planks","count":4}}}"####.to_string()
    ), (
        "jungle_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:jungle_stairs"},"tags":["crafting_table"],"group":"wooden_stairs","pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:jungle_planks"}},"unlock":[{"item":"minecraft:jungle_planks"}],"result":{"item":"minecraft:jungle_stairs","count":4}}}"####.to_string()
    ), (
        "jungle_wooden_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:jungle_wooden_slab"},"tags":["crafting_table"],"group":"wooden_slab","pattern":["###"],"key":{"#":{"item":"minecraft:jungle_planks"}},"unlock":[{"item":"minecraft:jungle_planks"}],"result":{"item":"minecraft:jungle_slab","count":6}}}"####.to_string()
    ), (
        "oak_fence.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:oak_fence"},"tags":["crafting_table"],"pattern":["W#W","W#W"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:oak_planks"}},"unlock":[{"item":"minecraft:oak_planks"}],"result":{"item":"minecraft:oak_fence","count":3}}}"####.to_string()
    ), (
        "oak_planks.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:oak_planks"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:oak_log"}},"unlock":[{"item":"minecraft:oak_log"}],"result":{"item":"minecraft:oak_planks","count":4}}}"####.to_string()
    ), (
        "oak_planks_from_stripped.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:oak_planks_from_stripped"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:stripped_oak_log"}},"unlock":[{"item":"minecraft:stripped_oak_log"}],"result":{"item":"minecraft:oak_planks","count":4}}}"####.to_string()
    ), (
        "oak_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:oak_stairs"},"tags":["crafting_table"],"group":"wooden_stairs","pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:oak_planks"}},"unlock":[{"item":"minecraft:oak_planks"}],"result":{"item":"minecraft:oak_stairs","count":4}}}"####.to_string()
    ), (
        "oak_wooden_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:oak_wooden_slab"},"tags":["crafting_table"],"group":"wooden_slab","pattern":["###"],"key":{"#":{"item":"minecraft:oak_planks"}},"unlock":[{"item":"minecraft:oak_planks"}],"result":{"item":"minecraft:oak_slab","count":6}}}"####.to_string()
    ), (
        "painting.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:painting"},"tags":["crafting_table"],"pattern":["AAA","ABA","AAA"],"key":{"A":{"item":"minecraft:stick"},"B":{"tag":"minecraft:wool"}},"unlock":[{"item":"minecraft:white_wool"},{"item":"minecraft:orange_wool"},{"item":"minecraft:magenta_wool"},{"item":"minecraft:light_blue_wool"},{"item":"minecraft:yellow_wool"},{"item":"minecraft:lime_wool"},{"item":"minecraft:pink_wool"},{"item":"minecraft:gray_wool"},{"item":"minecraft:light_gray_wool"},{"item":"minecraft:cyan_wool"},{"item":"minecraft:purple_wool"},{"item":"minecraft:blue_wool"},{"item":"minecraft:brown_wool"},{"item":"minecraft:green_wool"},{"item":"minecraft:red_wool"},{"item":"minecraft:black_wool"}],"result":{"item":"painting","count":1}}}"####.to_string()
    ), (
        "polished_andesite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:polished_andesite"},"tags":["crafting_table"],"pattern":["SS","SS"],"key":{"S":{"item":"minecraft:andesite"}},"unlock":[{"item":"minecraft:andesite"}],"result":{"item":"minecraft:polished_andesite","count":4}}}"####.to_string()
    ), (
        "polished_andesite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:polished_andesite_stairs"},"tags":["crafting_table"],"pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:polished_andesite"}},"unlock":[{"item":"minecraft:polished_andesite"}],"result":{"item":"minecraft:polished_andesite_stairs","count":4}}}"####.to_string()
    ), (
        "polished_diorite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:polished_diorite"},"tags":["crafting_table"],"pattern":["SS","SS"],"key":{"S":{"item":"minecraft:diorite"}},"unlock":[{"item":"minecraft:diorite"}],"result":{"item":"minecraft:polished_diorite","count":4}}}"####.to_string()
    ), (
        "polished_diorite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:polished_diorite_stairs"},"tags":["crafting_table"],"pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:polished_diorite"}},"unlock":[{"item":"minecraft:polished_diorite"}],"result":{"item":"minecraft:polished_diorite_stairs","count":4}}}"####.to_string()
    ), (
        "polished_granite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:polished_granite"},"tags":["crafting_table"],"pattern":["SS","SS"],"key":{"S":{"item":"minecraft:granite"}},"unlock":[{"item":"minecraft:granite"}],"result":{"item":"minecraft:polished_granite","count":4}}}"####.to_string()
    ), (
        "polished_granite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:polished_granite_stairs"},"tags":["crafting_table"],"pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:polished_granite"}},"unlock":[{"item":"minecraft:polished_granite"}],"result":{"item":"minecraft:polished_granite_stairs","count":4}}}"####.to_string()
    ), (
        "prismarine_bricks.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:prismarine_bricks"},"tags":["crafting_table"],"pattern":["SSS","SSS","SSS"],"key":{"S":{"item":"minecraft:prismarine_shard"}},"unlock":[{"item":"minecraft:prismarine_shard"}],"result":{"item":"minecraft:prismarine_bricks"}}}"####.to_string()
    ), (
        "prismarine_stairs_bricks.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:prismarine_stairs_bricks"},"tags":["crafting_table"],"pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:prismarine_bricks"}},"unlock":[{"item":"minecraft:prismarine_bricks"}],"result":{"item":"minecraft:prismarine_bricks_stairs","count":4}}}"####.to_string()
    ), (
        "prismarine_stairs_dark.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:prismarine_stairs_dark"},"tags":["crafting_table"],"pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:dark_prismarine"}},"unlock":[{"item":"minecraft:dark_prismarine"}],"result":{"item":"minecraft:dark_prismarine_stairs","count":4}}}"####.to_string()
    ), (
        "sign_acacia.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:sign_acacia"},"tags":["crafting_table"],"group":"sign","pattern":["###","###"," | "],"key":{"#":{"item":"minecraft:acacia_planks"},"|":{"item":"minecraft:stick"}},"unlock":[{"item":"minecraft:acacia_planks"}],"result":{"item":"minecraft:acacia_sign","count":3}}}"####.to_string()
    ), (
        "sign_birch.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:sign_birch"},"tags":["crafting_table"],"group":"sign","pattern":["###","###"," | "],"key":{"#":{"item":"minecraft:birch_planks"},"|":{"item":"minecraft:stick"}},"unlock":[{"item":"minecraft:birch_planks"}],"result":{"item":"minecraft:birch_sign","count":3}}}"####.to_string()
    ), (
        "sign_darkoak.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:sign_darkoak"},"tags":["crafting_table"],"group":"sign","pattern":["###","###"," | "],"key":{"#":{"item":"minecraft:dark_oak_planks"},"|":{"item":"minecraft:stick"}},"unlock":[{"item":"minecraft:dark_oak_planks"}],"result":{"item":"minecraft:darkoak_sign","count":3}}}"####.to_string()
    ), (
        "sign_jungle.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:sign_jungle"},"tags":["crafting_table"],"group":"sign","pattern":["###","###"," | "],"key":{"#":{"item":"minecraft:jungle_planks"},"|":{"item":"minecraft:stick"}},"unlock":[{"item":"minecraft:jungle_planks"}],"result":{"item":"minecraft:jungle_sign","count":3}}}"####.to_string()
    ), (
        "sign_oak.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:sign_oak"},"tags":["crafting_table"],"group":"sign","pattern":["###","###"," | "],"key":{"#":{"item":"minecraft:oak_planks"},"|":{"item":"minecraft:stick"}},"unlock":[{"item":"minecraft:oak_planks"}],"result":{"item":"minecraft:sign","count":3}}}"####.to_string()
    ), (
        "sign_spruce.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:sign_spruce"},"tags":["crafting_table"],"group":"sign","pattern":["###","###"," | "],"key":{"#":{"item":"minecraft:spruce_planks"},"|":{"item":"minecraft:stick"}},"unlock":[{"item":"minecraft:spruce_planks"}],"result":{"item":"minecraft:spruce_sign","count":3}}}"####.to_string()
    ), (
        "spruce_boat.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:spruce_boat"},"tags":["crafting_table"],"pattern":["# #","###"],"key":{"#":{"item":"minecraft:spruce_planks"}},"unlock":{"context":"PlayerInWater"},"result":{"item":"minecraft:spruce_boat"}}}"####.to_string()
    ), (
        "spruce_door.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:spruce_door"},"tags":["crafting_table"],"group":"wooden_door","pattern":["##","##","##"],"key":{"#":{"item":"minecraft:spruce_planks"}},"unlock":[{"item":"minecraft:spruce_planks"}],"result":{"item":"minecraft:spruce_door","count":3}}}"####.to_string()
    ), (
        "spruce_fence.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:spruce_fence"},"tags":["crafting_table"],"pattern":["W#W","W#W"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:spruce_planks"}},"unlock":[{"item":"minecraft:spruce_planks"}],"result":{"item":"minecraft:spruce_fence","count":3}}}"####.to_string()
    ), (
        "spruce_fence_gate.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:spruce_fence_gate"},"tags":["crafting_table"],"group":"wooden_fence_gate","pattern":["#W#","#W#"],"key":{"#":{"item":"minecraft:stick"},"W":{"item":"minecraft:spruce_planks"}},"unlock":[{"item":"minecraft:spruce_planks"}],"result":{"item":"minecraft:spruce_fence_gate"}}}"####.to_string()
    ), (
        "spruce_planks.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:spruce_planks"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:spruce_log"}},"unlock":[{"item":"minecraft:spruce_log"}],"result":{"item":"minecraft:spruce_planks","count":4}}}"####.to_string()
    ), (
        "spruce_planks_from_stripped.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:spruce_planks_from_stripped"},"tags":["crafting_table"],"group":"planks","pattern":["#"],"key":{"#":{"item":"minecraft:stripped_spruce_log"}},"unlock":[{"item":"minecraft:stripped_spruce_log"}],"result":{"item":"minecraft:spruce_planks","count":4}}}"####.to_string()
    ), (
        "spruce_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:spruce_stairs"},"tags":["crafting_table"],"group":"wooden_stairs","pattern":["#  ","## ","###"],"key":{"#":{"item":"minecraft:spruce_planks"}},"unlock":[{"item":"minecraft:spruce_planks"}],"result":{"item":"minecraft:spruce_stairs","count":4}}}"####.to_string()
    ), (
        "spruce_wooden_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:spruce_wooden_slab"},"tags":["crafting_table"],"group":"wooden_slab","pattern":["###"],"key":{"#":{"item":"minecraft:spruce_planks"}},"unlock":[{"item":"minecraft:spruce_planks"}],"result":{"item":"minecraft:spruce_slab","count":6}}}"####.to_string()
    ), (
        "stonecutter_andesite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_andesite_stairs"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:andesite"}],"unlock":[{"item":"minecraft:andesite"}],"result":{"item":"minecraft:andesite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_dark_prismarine_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_dark_prismarine_slab"},"tags":["stonecutter"],"priority":0,"ingredients":[{"item":"minecraft:dark_prismarine"}],"unlock":[{"item":"minecraft:dark_prismarine"}],"result":{"item":"minecraft:dark_prismarine_slab","count":2}}}"####.to_string()
    ), (
        "stonecutter_dark_prismarine_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_dark_prismarine_stairs"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:dark_prismarine"}],"unlock":[{"item":"minecraft:dark_prismarine"}],"result":{"item":"minecraft:dark_prismarine_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_diorite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_diorite_stairs"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:diorite"}],"unlock":[{"item":"minecraft:diorite"}],"result":{"item":"minecraft:diorite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_granite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_granite_stairs"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:granite"}],"unlock":[{"item":"minecraft:granite"}],"result":{"item":"minecraft:granite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_mossy_cobblestone_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_mossy_cobbledouble_stone_slab"},"tags":["stonecutter"],"priority":0,"ingredients":[{"item":"minecraft:mossy_cobblestone"}],"unlock":[{"item":"minecraft:mossy_cobblestone"}],"result":{"item":"minecraft:mossy_cobblestone_slab","count":2}}}"####.to_string()
    ), (
        "stonecutter_polished_andesite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_andesite"},"tags":["stonecutter"],"priority":3,"ingredients":[{"item":"minecraft:andesite"}],"unlock":[{"item":"minecraft:andesite"}],"result":{"item":"minecraft:polished_andesite","count":1}}}"####.to_string()
    ), (
        "stonecutter_polished_andesite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_andesite_stairs"},"tags":["stonecutter"],"priority":5,"ingredients":[{"item":"minecraft:andesite"}],"unlock":[{"item":"minecraft:andesite"}],"result":{"item":"minecraft:polished_andesite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_polished_andesite_stairs2.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_andesite_stairs2"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:polished_andesite"}],"unlock":[{"item":"minecraft:polished_andesite"}],"result":{"item":"minecraft:polished_andesite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_polished_diorite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_diorite"},"tags":["stonecutter"],"priority":3,"ingredients":[{"item":"minecraft:diorite"}],"unlock":[{"item":"minecraft:diorite"}],"result":{"item":"minecraft:polished_diorite","count":1}}}"####.to_string()
    ), (
        "stonecutter_polished_diorite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_diorite_stairs"},"tags":["stonecutter"],"priority":5,"ingredients":[{"item":"minecraft:diorite"}],"unlock":[{"item":"minecraft:diorite"}],"result":{"item":"minecraft:polished_diorite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_polished_diorite_stairs2.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_diorite_stairs2"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:polished_diorite"}],"unlock":[{"item":"minecraft:polished_diorite"}],"result":{"item":"minecraft:polished_diorite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_polished_granite.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_granite"},"tags":["stonecutter"],"priority":3,"ingredients":[{"item":"minecraft:granite"}],"unlock":[{"item":"minecraft:granite"}],"result":{"item":"minecraft:polished_granite","count":1}}}"####.to_string()
    ), (
        "stonecutter_polished_granite_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_granite_stairs"},"tags":["stonecutter"],"priority":5,"ingredients":[{"item":"minecraft:granite"}],"unlock":[{"item":"minecraft:granite"}],"result":{"item":"minecraft:polished_granite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_polished_granite_stairs2.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_polished_granite_stairs2"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:polished_granite"}],"unlock":[{"item":"minecraft:polished_granite"}],"result":{"item":"minecraft:polished_granite_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_prismarine_brick_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_prismarine_brick_slab"},"tags":["stonecutter"],"priority":0,"ingredients":[{"item":"minecraft:prismarine_bricks"}],"unlock":[{"item":"minecraft:prismarine_bricks"}],"result":{"item":"minecraft:prismarine_brick_slab","count":2}}}"####.to_string()
    ), (
        "stonecutter_prismarine_brick_stairs.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_prismarine_brick_stairs"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:prismarine_bricks"}],"unlock":[{"item":"minecraft:prismarine_bricks"}],"result":{"item":"minecraft:prismarine_bricks_stairs","data":0,"count":1}}}"####.to_string()
    ), (
        "stonecutter_prismarine_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_prismarine_slab"},"tags":["stonecutter"],"priority":0,"ingredients":[{"item":"minecraft:prismarine","data":0}],"unlock":[{"item":"minecraft:prismarine","data":0}],"result":{"item":"minecraft:prismarine_slab","count":2}}}"####.to_string()
    ), (
        "stonecutter_purpur_pillar.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_purpur_lines"},"tags":["stonecutter"],"priority":0,"ingredients":[{"item":"minecraft:purpur_block","data":0}],"unlock":[{"item":"minecraft:purpur_block","data":0}],"result":{"item":"minecraft:purpur_pillar","count":1}}}"####.to_string()
    ), (
        "stonecutter_purpur_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_purpur_slab"},"tags":["stonecutter"],"priority":1,"ingredients":[{"item":"minecraft:purpur_block","data":0}],"unlock":[{"item":"minecraft:purpur_block","data":0}],"result":{"item":"minecraft:purpur_slab","count":2}}}"####.to_string()
    ), (
        "stonecutter_red_nether_brick_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_red_nether_brick_slab"},"tags":["stonecutter"],"priority":0,"ingredients":[{"item":"minecraft:red_nether_brick"}],"unlock":[{"item":"minecraft:red_nether_brick"}],"result":{"item":"minecraft:red_nether_brick_slab","count":2}}}"####.to_string()
    ), (
        "stonecutter_smooth_sandstone_slab.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shapeless":{"description":{"identifier":"minecraft:stonecutter_smooth_sanddouble_stone_slab"},"tags":["stonecutter"],"priority":0,"ingredients":[{"item":"minecraft:smooth_sandstone"}],"unlock":[{"item":"minecraft:smooth_sandstone"}],"result":{"item":"minecraft:smooth_sandstone_slab","count":2}}}"####.to_string()
    ), (
        "string_to_wool.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:string_to_wool"},"tags":["crafting_table"],"pattern":["##","##"],"key":{"#":{"item":"minecraft:string"}},"unlock":[{"item":"minecraft:string"}],"result":{"item":"minecraft:white_wool"}}}"####.to_string()
    ), (
        "wooden_door.json".to_string(),
        r####"{"format_version":"1.20.10","minecraft:recipe_shaped":{"description":{"identifier":"minecraft:wooden_door"},"tags":["crafting_table"],"group":"wooden_door","pattern":["##","##","##"],"key":{"#":{"item":"minecraft:oak_planks"}},"unlock":[{"item":"minecraft:oak_planks"}],"result":{"item":"minecraft:wooden_door","count":3}}}"####.to_string()
    ),
    ]));

    assert_eq!(archive_result, archive_correct);
}
