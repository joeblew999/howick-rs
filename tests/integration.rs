use howick_rs::csv;
use howick_rs::types::{LabelOrientation, Operation, Unit};

// The actual sample CSV from the factory
const SAMPLE_CSV: &str = r#"UNIT,MILLIMETRE
PROFILE,S8908,Standard Profile
FRAMESET,T1
COMPONENT,T1-1,LABEL_INV,1,3945.0,DIMPLE,20.65,DIMPLE,212.52,DIMPLE,262.51,DIMPLE,646.25,DIMPLE,696.26,DIMPLE,1080.01,DIMPLE,1130.0,DIMPLE,1513.76,DIMPLE,1563.75,DIMPLE,1947.49,DIMPLE,1997.51,DIMPLE,2381.25,DIMPLE,2431.24,DIMPLE,2815.01,DIMPLE,2864.99,DIMPLE,3248.74,DIMPLE,3298.75,DIMPLE,3682.49,DIMPLE,3732.48,DIMPLE,3924.35,LIP_CUT,23.0,LIP_CUT,205.68,LIP_CUT,270.55,LIP_CUT,270.71,LIP_CUT,638.21,LIP_CUT,638.37,LIP_CUT,704.31,LIP_CUT,704.47,LIP_CUT,1071.97,LIP_CUT,1072.13,LIP_CUT,1138.04,LIP_CUT,1138.2,LIP_CUT,1505.72,LIP_CUT,1505.88,LIP_CUT,1571.79,LIP_CUT,1571.95,LIP_CUT,1939.45,LIP_CUT,1939.61,LIP_CUT,2005.55,LIP_CUT,2005.71,LIP_CUT,2373.21,LIP_CUT,2373.37,LIP_CUT,2439.28,LIP_CUT,2439.44,LIP_CUT,2806.96,LIP_CUT,2807.12,LIP_CUT,2873.03,LIP_CUT,2873.19,LIP_CUT,3240.69,LIP_CUT,3240.85,LIP_CUT,3306.79,LIP_CUT,3306.95,LIP_CUT,3674.45,LIP_CUT,3674.61,LIP_CUT,3739.32,LIP_CUT,3922.0,WEB,2634.7,WEB,1755.6,WEB,454.4,WEB,3493.5
COMPONENT,T1-2,LABEL_NRM,1,3945.0,DIMPLE,20.65,DIMPLE,70.64,DIMPLE,429.39,DIMPLE,479.37,DIMPLE,863.14,DIMPLE,913.13,DIMPLE,1296.87,DIMPLE,1346.89,DIMPLE,1730.63,DIMPLE,1780.62,DIMPLE,2164.38,DIMPLE,2214.37,DIMPLE,2598.12,DIMPLE,2648.13,DIMPLE,3031.87,DIMPLE,3081.86,DIMPLE,3465.63,DIMPLE,3515.61,DIMPLE,3874.36,DIMPLE,3924.35,LIP_CUT,23.0,LIP_CUT,77.48,LIP_CUT,421.35,LIP_CUT,421.51,LIP_CUT,487.42,LIP_CUT,487.57,LIP_CUT,855.1,LIP_CUT,855.26,LIP_CUT,921.17,LIP_CUT,921.33,LIP_CUT,1288.83,LIP_CUT,1288.99,LIP_CUT,1354.93,LIP_CUT,1355.09,LIP_CUT,1722.59,LIP_CUT,1722.75,LIP_CUT,1788.66,LIP_CUT,1788.82,LIP_CUT,2156.34,LIP_CUT,2156.5,LIP_CUT,2222.41,LIP_CUT,2222.57,LIP_CUT,2590.07,LIP_CUT,2590.23,LIP_CUT,2656.17,LIP_CUT,2656.33,LIP_CUT,3023.83,LIP_CUT,3023.99,LIP_CUT,3089.9,LIP_CUT,3090.06,LIP_CUT,3457.59,LIP_CUT,3457.75,LIP_CUT,3523.66,LIP_CUT,3523.81,LIP_CUT,3867.52,LIP_CUT,3922.0
COMPONENT,T1-3,LABEL_INV,1,466.0,DIMPLE,447.34,DIMPLE,18.64,SWAGE,438.5,SWAGE,27.5,WEB,178.3,WEB,386.3
COMPONENT,T1-4,LABEL_NRM,1,466.0,DIMPLE,447.34,DIMPLE,18.64,SWAGE,438.5,SWAGE,27.5,WEB,239.3,WEB,404.4
COMPONENT,T1-5,LABEL_INV,1,483.95,DIMPLE,467.74,DIMPLE,16.18,SWAGE,456.45,SWAGE,27.5,END_TRUSS,483.95,END_TRUSS,0.0
COMPONENT,T1-6,LABEL_NRM,1,491.98,DIMPLE,476.0,DIMPLE,15.98,SWAGE,464.48,SWAGE,27.5,END_TRUSS,491.98,END_TRUSS,0.0
COMPONENT,T1-22,LABEL_NRM,1,483.95,DIMPLE,467.74,DIMPLE,16.18,SWAGE,456.45,SWAGE,27.5,END_TRUSS,483.95,END_TRUSS,0.0"#;

#[test]
fn test_parse_header() {
    let frameset = csv::parse(SAMPLE_CSV).unwrap();
    assert_eq!(frameset.name, "T1");
    assert_eq!(frameset.unit, Unit::Millimetre);
    assert_eq!(frameset.profile.code, "S8908");
    assert_eq!(frameset.profile.description, "Standard Profile");
}

#[test]
fn test_parse_component_count() {
    let frameset = csv::parse(SAMPLE_CSV).unwrap();
    assert_eq!(frameset.components.len(), 7);
}

#[test]
fn test_parse_chord_member() {
    let frameset = csv::parse(SAMPLE_CSV).unwrap();
    let t1_1 = &frameset.components[0];

    assert_eq!(t1_1.id, "T1-1");
    assert_eq!(t1_1.label, LabelOrientation::Inverted);
    assert_eq!(t1_1.quantity, 1);
    assert_eq!(t1_1.length_mm, 3945.0);

    // Count operations by type
    let dimples = t1_1.operations.iter().filter(|op| matches!(op, Operation::Dimple(_))).count();
    let lip_cuts = t1_1.operations.iter().filter(|op| matches!(op, Operation::LipCut(_))).count();
    let webs = t1_1.operations.iter().filter(|op| matches!(op, Operation::Web(_))).count();

    assert_eq!(dimples, 20);
    assert_eq!(lip_cuts, 36);
    assert_eq!(webs, 4);
}

#[test]
fn test_parse_web_member_with_swage() {
    let frameset = csv::parse(SAMPLE_CSV).unwrap();
    let t1_3 = &frameset.components[2];

    assert_eq!(t1_3.id, "T1-3");
    assert_eq!(t1_3.label, LabelOrientation::Inverted);
    assert_eq!(t1_3.length_mm, 466.0);

    let swages = t1_3.operations.iter().filter(|op| matches!(op, Operation::Swage(_))).count();
    assert_eq!(swages, 2);
}

#[test]
fn test_parse_end_truss_member() {
    let frameset = csv::parse(SAMPLE_CSV).unwrap();
    let t1_5 = &frameset.components[4];

    assert_eq!(t1_5.id, "T1-5");
    assert_eq!(t1_5.length_mm, 483.95);

    let end_trusses: Vec<_> = t1_5.operations.iter()
        .filter_map(|op| if let Operation::EndTruss(p) = op { Some(p) } else { None })
        .collect();

    assert_eq!(end_trusses.len(), 2);
    assert_eq!(*end_trusses[0], 483.95);
    assert_eq!(*end_trusses[1], 0.0);
}

#[test]
fn test_roundtrip() {
    // Parse → serialize → parse again, check equality
    let original = csv::parse(SAMPLE_CSV).unwrap();
    let serialized = csv::serialize(&original).unwrap();
    let reparsed = csv::parse(&serialized).unwrap();

    assert_eq!(original.name, reparsed.name);
    assert_eq!(original.unit, reparsed.unit);
    assert_eq!(original.profile.code, reparsed.profile.code);
    assert_eq!(original.components.len(), reparsed.components.len());

    for (orig, reparsed) in original.components.iter().zip(reparsed.components.iter()) {
        assert_eq!(orig.id, reparsed.id);
        assert_eq!(orig.length_mm, reparsed.length_mm);
        assert_eq!(orig.operations.len(), reparsed.operations.len());
    }
}

#[test]
fn test_label_pair_symmetry() {
    // Use the full 22-component CSV to verify INV/NRM symmetry
    let full_csv = include_str!("fixtures/T1_full.csv");
    let frameset = csv::parse(full_csv).unwrap();
    let inv_count = frameset.components.iter()
        .filter(|c| c.label == LabelOrientation::Inverted)
        .count();
    let nrm_count = frameset.components.iter()
        .filter(|c| c.label == LabelOrientation::Normal)
        .count();
    assert_eq!(inv_count, nrm_count,
        "Expected equal INV/NRM pairs, got {} INV and {} NRM", inv_count, nrm_count);
}

#[test]
fn test_full_csv_component_count() {
    let full_csv = include_str!("fixtures/T1_full.csv");
    let frameset = csv::parse(full_csv).unwrap();
    assert_eq!(frameset.components.len(), 22);
}
