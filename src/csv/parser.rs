use crate::error::HowickError;
use crate::types::{Component, Frameset, LabelOrientation, Operation, Profile, Unit};

/// Parse a Howick CSV string into a [`Frameset`].
///
/// # Format
/// ```text
/// UNIT,MILLIMETRE
/// PROFILE,S8908,Standard Profile
/// FRAMESET,T1
/// COMPONENT,T1-1,LABEL_INV,1,3945.0,DIMPLE,20.65,...
/// ```
pub fn parse(input: &str) -> Result<Frameset, HowickError> {
    let mut unit: Option<Unit> = None;
    let mut profile: Option<Profile> = None;
    let mut frameset_name: Option<String> = None;
    let mut components: Vec<Component> = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "UNIT" => {
                let u = parts.get(1).ok_or_else(|| HowickError::MissingField("UNIT value".into()))?;
                unit = Some(parse_unit(u)?);
            }
            "PROFILE" => {
                let code = parts.get(1).ok_or_else(|| HowickError::MissingField("PROFILE code".into()))?;
                let description = parts.get(2).unwrap_or(&"").to_string();
                profile = Some(Profile {
                    code: code.to_string(),
                    description,
                });
            }
            "FRAMESET" => {
                let name = parts.get(1).ok_or_else(|| HowickError::MissingField("FRAMESET name".into()))?;
                frameset_name = Some(name.to_string());
            }
            "COMPONENT" => {
                let component = parse_component(&parts, line_num + 1)?;
                components.push(component);
            }
            _ => {
                // Unknown row types are silently ignored for forward compatibility
            }
        }
    }

    Ok(Frameset {
        name: frameset_name.ok_or_else(|| HowickError::MissingField("FRAMESET".into()))?,
        unit: unit.ok_or_else(|| HowickError::MissingField("UNIT".into()))?,
        profile: profile.ok_or_else(|| HowickError::MissingField("PROFILE".into()))?,
        components,
    })
}

fn parse_unit(s: &str) -> Result<Unit, HowickError> {
    match s {
        "MILLIMETRE" => Ok(Unit::Millimetre),
        other => Err(HowickError::UnknownUnit(other.to_string())),
    }
}

fn parse_label(s: &str) -> Result<LabelOrientation, HowickError> {
    match s {
        "LABEL_NRM" => Ok(LabelOrientation::Normal),
        "LABEL_INV" => Ok(LabelOrientation::Inverted),
        other => Err(HowickError::UnknownLabel(other.to_string())),
    }
}

fn parse_f64(s: &str) -> Result<f64, HowickError> {
    s.parse::<f64>().map_err(|e| HowickError::InvalidNumber {
        value: s.to_string(),
        source: e,
    })
}

fn parse_u32(s: &str) -> Result<u32, HowickError> {
    s.parse::<u32>().map_err(|e| HowickError::InvalidInteger {
        value: s.to_string(),
        source: e,
    })
}

fn parse_component(parts: &[&str], line_num: usize) -> Result<Component, HowickError> {
    // COMPONENT, id, label, qty, length, [op, pos, ...]
    if parts.len() < 5 {
        return Err(HowickError::ParseError {
            line: line_num,
            message: format!("COMPONENT needs at least 5 fields, got {}", parts.len()),
        });
    }

    let id = parts[1].to_string();
    let label = parse_label(parts[2])?;
    let quantity = parse_u32(parts[3])?;
    let length_mm = parse_f64(parts[4])?;

    let mut operations = Vec::new();
    let mut i = 5;
    while i + 1 < parts.len() {
        let op_name = parts[i];
        let position = parse_f64(parts[i + 1])?;
        let op = match op_name {
            "DIMPLE" => Operation::Dimple(position),
            "LIP_CUT" => Operation::LipCut(position),
            "SWAGE" => Operation::Swage(position),
            "WEB" => Operation::Web(position),
            "END_TRUSS" => Operation::EndTruss(position),
            other => return Err(HowickError::UnknownOperation(other.to_string())),
        };
        operations.push(op);
        i += 2;
    }

    Ok(Component {
        id,
        label,
        quantity,
        length_mm,
        operations,
    })
}
