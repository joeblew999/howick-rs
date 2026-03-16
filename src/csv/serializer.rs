use std::fmt::Write;

use crate::error::HowickError;
use crate::types::Frameset;

/// Serialize a [`Frameset`] to a Howick CSV string.
pub fn serialize(frameset: &Frameset) -> Result<String, HowickError> {
    let mut out = String::new();

    // Header
    writeln!(out, "UNIT,{}", frameset.unit.as_str())?;
    writeln!(
        out,
        "PROFILE,{},{}",
        frameset.profile.code, frameset.profile.description
    )?;
    writeln!(out, "FRAMESET,{}", frameset.name)?;

    // Components
    for component in &frameset.components {
        write!(
            out,
            "COMPONENT,{},{},{},{}",
            component.id,
            component.label.as_str(),
            component.quantity,
            format_f64(component.length_mm),
        )?;

        for op in &component.operations {
            write!(out, ",{},{}", op.name(), format_f64(op.position()))?;
        }

        writeln!(out)?;
    }

    Ok(out)
}

/// Format a float the way Howick CSVs do — minimal decimals, no trailing zeros beyond 2dp.
fn format_f64(v: f64) -> String {
    // Howick uses up to 2 decimal places in practice
    let s = format!("{:.2}", v);
    // Strip trailing zeros after decimal but keep at least one decimal place
    let s = s.trim_end_matches('0');
    let s = s.trim_end_matches('.');
    // Always ensure there's a decimal point for clarity
    if s.contains('.') {
        s.to_string()
    } else {
        format!("{}.0", s)
    }
}
