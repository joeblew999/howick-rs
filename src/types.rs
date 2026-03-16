/// Unit of measurement. Howick machines work in millimetres.
#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Millimetre,
}

impl Unit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Unit::Millimetre => "MILLIMETRE",
        }
    }
}

/// Steel profile specification.
/// e.g. S8908 = Standard C-section, 89mm web, 0.8mm gauge (approximate).
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub code: String,
    pub description: String,
}

/// Label orientation on the printed inkjet label.
/// LABEL_INV = inverted (C-section facing one direction).
/// LABEL_NRM = normal (C-section facing the other direction).
/// Pairs of INV/NRM members are mirror images — both required for each position in a truss or wall.
#[derive(Debug, Clone, PartialEq)]
pub enum LabelOrientation {
    Normal,
    Inverted,
}

impl LabelOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            LabelOrientation::Normal => "LABEL_NRM",
            LabelOrientation::Inverted => "LABEL_INV",
        }
    }
}

/// A machine operation with its position along the member in mm.
/// All positions are measured from the start (feed end) of the member.
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    /// Dimple punch — raised bump for screw location.
    /// Dimples come in pairs ~50mm apart at each stud connection point.
    Dimple(f64),

    /// Lip cut — cuts the lip of the C-section at a connection notch.
    /// Comes in pairs ~0.16mm apart defining the slot width.
    LipCut(f64),

    /// Swage — crimp/deformation of the steel at chord-to-web joints in trusses.
    Swage(f64),

    /// Web punch — hole through the web for services (pipes, cables, bolts).
    Web(f64),

    /// End truss cut — marks the truss end cut angle position.
    /// Always comes in pairs: (length, 0.0) for the two ends.
    EndTruss(f64),

    /// Notch — a rectangular cutout in the flange, used where members intersect.
    /// Seen on wall tracks and studs at opening jambs and panel ends.
    /// Comes in pairs: (position_far, position_near) defining the notch extents.
    Notch(f64),

    /// Service hole — larger punched hole for building services (electrical conduit,
    /// plumbing, HVAC). Distinct from WEB holes: SERVICE_HOLE is for services,
    /// WEB is for structural bolts/connections.
    ServiceHole(f64),
}

impl Operation {
    pub fn name(&self) -> &'static str {
        match self {
            Operation::Dimple(_) => "DIMPLE",
            Operation::LipCut(_) => "LIP_CUT",
            Operation::Swage(_) => "SWAGE",
            Operation::Web(_) => "WEB",
            Operation::EndTruss(_) => "END_TRUSS",
            Operation::Notch(_) => "NOTCH",
            Operation::ServiceHole(_) => "SERVICE_HOLE",
        }
    }

    pub fn position(&self) -> f64 {
        match self {
            Operation::Dimple(p) => *p,
            Operation::LipCut(p) => *p,
            Operation::Swage(p) => *p,
            Operation::Web(p) => *p,
            Operation::EndTruss(p) => *p,
            Operation::Notch(p) => *p,
            Operation::ServiceHole(p) => *p,
        }
    }
}

/// A single steel member to be produced by the machine.
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    /// Unique identifier within the frameset. e.g. "T1-1"
    pub id: String,

    /// Label orientation printed on the member.
    pub label: LabelOrientation,

    /// Number of identical pieces required.
    pub quantity: u32,

    /// Cut length in mm.
    pub length_mm: f64,

    /// Ordered list of machine operations along the member.
    /// Operations are interleaved in the CSV as: OP_NAME,position,OP_NAME,position,...
    pub operations: Vec<Operation>,
}

/// A complete frameset — all components for one truss, wall panel, or floor panel.
#[derive(Debug, Clone, PartialEq)]
pub struct Frameset {
    /// Name of this frameset. e.g. "T1" (Truss 1), "W1" (Wall 1).
    pub name: String,

    /// Unit of measurement.
    pub unit: Unit,

    /// Steel profile specification.
    pub profile: Profile,

    /// All components in this frameset.
    pub components: Vec<Component>,
}
