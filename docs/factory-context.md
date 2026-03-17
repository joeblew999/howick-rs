# Factory Context & Discovery

## The Customer

**CEO:** Prin  
**Location:** Si Racha / Laem Chabang area, Eastern Seaboard, Thailand  
**Business:** Steel framing factory producing light gauge steel (LGS) frames for housing construction

---

## Machines in the Factory

### Howick FRAMA (roll-forming machine)
- Made by **Howick Ltd**, Auckland, New Zealand
- Produces C-section steel studs, tracks, trusses, and joists from coiled steel
- Accuracy: ±0.5mm
- **File format: Open CSV** (V1/V2) — no lock-in, any software can feed it
- Interface: USB stick or network file share (to be confirmed)
- More info: https://www.howickltd.com

### FRAMECAD Machine (roll-forming machine)
- Made by **FRAMECAD**, New Zealand
- Similar roll-forming machine but proprietary file format
- **File format: `.RFY` or `.XML/ACNC`** — proprietary
- Gateway: **FRAMECAD Nexa** production management platform has an API (Builder tier)
- More info: https://www.framecad.com

---

## Current Software Workflow

**Design:** SketchUp (mesh-based, no fabrication precision — the pain point)

**CSV generation for Howick machine:** `FrameBuilderMRD`  
- Confirmed by Prin directly (March 2026 factory visit)
- FrameBuilderMRD is a Howick software partner
- It takes structural design input and outputs Howick-compatible CSV
- Website: https://www.framebuilder.com.au (to be confirmed)

**Gap:** SketchUp → FrameBuilderMRD is a manual, imprecise handoff.  
The CEO is keen on a better solution. plat-trunk replacing this pipeline is the opportunity.

---

## The CSV Format — Decoded

From the sample file `tests/fixtures/T1_full.csv` (a real roof truss job):

```
UNIT,MILLIMETRE
PROFILE,S8908,Standard Profile
FRAMESET,T1
COMPONENT,T1-1,LABEL_INV,1,3945.0,DIMPLE,20.65,...
```

### File structure
| Row | Meaning |
|-----|---------|
| `UNIT,MILLIMETRE` | All dimensions in millimetres |
| `PROFILE,S8908,Standard Profile` | Steel profile code and description |
| `FRAMESET,T1` | Name of this frameset (T=Truss, W=Wall, F=Floor) |
| `COMPONENT,...` | One row per member |

### COMPONENT row fields
```
COMPONENT, id, label, qty, length_mm, [op, position_mm, ...]
```

| Field | Example | Meaning |
|-------|---------|---------|
| `id` | `T1-1` | Unique member ID within frameset |
| `label` | `LABEL_INV` or `LABEL_NRM` | Print orientation on inkjet label |
| `qty` | `1` | Number of identical pieces |
| `length_mm` | `3945.0` | Cut length in millimetres |
| operations | `DIMPLE,20.65,...` | Interleaved op/position pairs |

### Operation types
| Operation | What the machine does |
|-----------|----------------------|
| `DIMPLE` | Raised bump punch for screw location. Come in pairs ~50mm apart at each connection point |
| `LIP_CUT` | Cuts the lip of the C-section at a notch. Come in pairs ~0.16mm apart |
| `SWAGE` | Crimp/deformation at chord-to-web joints in trusses |
| `WEB` | Hole through the web for services (pipes, cables, bolts) |
| `END_TRUSS` | Truss end cut angle. Always two values: (length, 0.0) |

### Label orientation
- `LABEL_INV` / `LABEL_NRM` — every member comes in mirrored pairs because a C-section faces two directions
- In a well-formed truss, INV count == NRM count (verified in tests)

### Profile code
- `S8908` — Standard C-section. Likely: 89mm web, 0.8mm gauge (to be confirmed with Prin)

---

## The T1 Truss — What the Sample File Represents

The sample `T1_full.csv` is a **roof truss** with 22 components:

| Members | ID | Length | Type |
|---------|-----|--------|------|
| Top/bottom chords | T1-1, T1-2 | 3945mm | Long chord members with dimples + lip cuts at every stud connection |
| End verticals | T1-5, T1-22 | 483.95mm | End of truss, has END_TRUSS operations |
| Standard web members | T1-6 to T1-21 (16 pcs) | 491.98mm | All identical internal web members |
| Short web members | T1-3, T1-4 | 466.0mm | Slightly different web hole positions |

Stud spacing derived from dimple pattern: ~383.74mm centres (~400mm nominal).

---

## Future: USB Dongle → OPC UA

**Current state:** The Howick machine receives files via USB stick or local network.

**Vision (discussed with Gerard, March 2026):**  
Replace the USB stick workflow with a small compute module (e.g. Raspberry Pi / Intel NUC / Mac Mini) sitting next to the machine, running an OPC UA server. This would enable:

- Real-time job status from the machine back to plat-trunk
- Remote job submission (no USB stick, files pushed over network)
- Machine telemetry — coil weight remaining, pieces produced, error states
- Integration with plat-trunk's Automerge CRDT sync layer

**Protocol:** OPC UA (IEC 62541) — the industry standard for heterogeneous industrial hardware  
**Rust crate:** `async-opcua` (https://github.com/FreeOpcUa/async-opcua) — pure Rust, tokio, MPL-2.0

See the companion repo: https://github.com/joeblew999/opcua-howick (planned)

---

## Current Workflow (confirmed March 2026)

**File transfer method: USB stick**

The operator runs FrameBuilderMRD on a Windows PC, exports CSV files,
copies them to a USB stick, then physically plugs the USB stick into
the Howick machine's control PC. The machine software reads from a
specific folder on that PC.

**Integration path for opcua-howick:**

```
plat-trunk (browser)
    ↓ POST /api/jobs/howick
CF Worker → R2
    ↓ polls every 5s
opcua-howick (Pi or Windows PC on factory LAN)
    ↓ copies CSV over network share or local path
Howick control PC watched folder   ← THIS PATH IS THE MISSING PIECE
    ↓
Howick FRAMA machine runs it
```

The one remaining unknown: what folder path on the Howick control PC
does the machine software watch? Currently the operator copies USB
files into that folder manually. opcua-howick replaces the USB stick
by writing directly to that folder over the network (SMB share) or
locally if running on the same machine.

**Setup options for Prin's factory:**

Option A — Pi on LAN, SMB to Howick PC:
  Pi running opcua-howick mounts the Howick PC's shared folder via SMB.
  machine_input_dir = "//howick-pc/shared/jobs/"  (or similar)

Option B — opcua-howick runs ON the Howick PC (Windows):
  Build opcua-howick for Windows (x86_64-pc-windows-msvc).
  machine_input_dir = "C:\\Howick\\Jobs\\"  (exact path TBC with Prin)
  No network share needed — writes directly to the watched folder.

Option B is simpler for the first demo. opcua-howick already builds
for Windows via GitHub Actions (x86_64 Linux is built; Windows needs
adding). One .exe, drop it on the Howick PC, configure config.toml.

## Open Questions for Prin

1. What model is the Howick machine? (size determines model — under 3m = 3200, ~3.4m = 5600)
2. Is `S8908` always the same profile or does it vary per job?
3. Do WEB hole positions come from FrameBuilderMRD design or are they fixed per profile?
4. How are files currently transferred to the machine — USB stick or network?
5. Does the FRAMECAD machine also use FrameBuilderMRD, or different software?
6. Would Prin be open to a trial where plat-trunk generates the CSV directly?
