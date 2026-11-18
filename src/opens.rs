//! Openers and Home slots as data beside each system (repo:ce/docs/CE-REF-OPENERS-AND-HOME-2026-11-18.md sections 3.1
//! and 5.1). `cell-net` does not take `opens` or `slot` lines yet (design open point), so each system that opens a type
//! or offers a Home slot lists them here with the same fields; `open.with@1` reads [`opens`] and `home.shell@1` reads
//! [`HOME_SLOTS`]. When `net!` takes the lines, these consts move into the `net!` blocks and no reader changes.
//!
//! Slice 1 table (section 8 A). `cell` is the cell that must answer HealthOp for the opener to be offered and whose
//! data the view reads as the viewer; `view` is the view module the shell mounts (`<bundle>::<View>`). An opener whose
//! `needs` the viewer's chain does not hold is never offered and never named to that viewer.

/// What an opener does with the thing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    View,
    Edit,
}

/// One `opens` line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opens {
    /// a type string (`call`, `code/rust`), a family (`code/*`) or the floor (`*`)
    pub pattern: &'static str,
    pub mode: Mode,
    /// the cell that answers HealthOp for this opener and holds its data
    pub cell: &'static str,
    pub major: u16,
    /// the view the shell mounts: `<bundle>::<View>`
    pub view: &'static str,
    /// higher wins among equally specific patterns; equal rank for the same pattern and mode fails `cargo test`
    pub rank: u16,
    /// the right the viewer's chain must hold; "" = any signed-in viewer of open.with
    pub needs: &'static str,
}

const fn o(pattern: &'static str, cell: &'static str, major: u16, view: &'static str, rank: u16, needs: &'static str) -> Opens {
    Opens { pattern, mode: Mode::View, cell, major, view, rank, needs }
}

/// REFS: files by type through ref.resolve and ref.pin, as the viewer; `*` is the floor (name, size, hash, holder).
pub const REFS_OPENS: &[Opens] = &[
    o("code/*", "ref.pin", 1, "ref.view::Text", 20, "ref.pin:read"),
    o("text/*", "ref.pin", 1, "ref.view::Text", 20, "ref.pin:read"),
    o("doc/markdown", "ref.pin", 1, "ref.view::Text", 20, "ref.pin:read"),
    o("table/csv", "ref.pin", 1, "ref.view::Text", 20, "ref.pin:read"),
    o("*", "ref.resolve", 1, "ref.view::Raw", 1, "ref.resolve:read"),
];

/// CRM: one record each, read as the viewer from the cell that owns it.
pub const CRM_OPENS: &[Opens] = &[
    o("person", "crm.people", 1, "ref.view::Record", 20, "crm.people:read"),
    o("promise", "crm.promises", 1, "ref.view::Record", 20, "crm.promises:read"),
    o("call", "crm.calls", 2, "ref.view::Record", 20, "crm.calls:read"),
    o("meet", "search.hybrid", 1, "ref.view::Record", 20, "search.hybrid:read:calls"),
];

/// CORE: cells and nodes are ordinary refs ("browse all cells"); a `web:` ref is the outside web, said so.
pub const CORE_OPENS: &[Opens] = &[
    o("cell", "site.cellmap", 1, "site.cellmap::Cell", 50, "site.cellmap:read"),
    o("node", "site.cellmap", 1, "site.cellmap::Node", 50, "site.cellmap:read"),
    o("web", "apps.shell", 1, "apps.shell::Outside", 1, ""),
];

/// CAD and MAPS until `cad.view` and `maps.view` exist: the static sites of ours wrapped by a port view at rank 10
/// under their real refs, so the real views take over at rank 50 and no ref changes (section 5.4).
pub const CAD_OPENS: &[Opens] = &[o("assembly", "apps.shell", 1, "apps.shell::Port", 10, "")];
pub const MAPS_OPENS: &[Opens] = &[o("map", "apps.shell", 1, "apps.shell::Port", 10, "")];

/// Every opener of `CE_NET`.
pub fn opens() -> Vec<Opens> {
    [REFS_OPENS, CRM_OPENS, CORE_OPENS, CAD_OPENS, MAPS_OPENS].concat()
}

/// Where a slot sits on the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Area {
    Head,
    Main,
    Side,
}

/// One `slot ... in home.shell@1` line. `when` names the cheap op that says whether the slot has something for this
/// viewer (the shell asks it as the viewer; nothing = no slot).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotLine {
    pub name: &'static str,
    /// declared by this system
    pub system: &'static str,
    pub cell: &'static str,
    pub major: u16,
    pub view: &'static str,
    pub area: Area,
    pub order: u16,
    pub needs: &'static str,
    pub when: Option<&'static str>,
}

/// The Home slots of slice 1 (section 8 C). The feed slot is one line the V2 workflow adds (TALK), not here.
pub const HOME_SLOTS: &[SlotLine] = &[
    SlotLine { name: "calls", system: "CRM", cell: "crm.calls", major: 2, view: "home::CallsToday", area: Area::Main, order: 10, needs: "crm.calls:read", when: Some("crm.calls/List") },
    SlotLine { name: "promises", system: "CRM", cell: "crm.promises", major: 1, view: "home::PromisesDue", area: Area::Main, order: 20, needs: "crm.promises:read", when: Some("crm.promises/List") },
    SlotLine { name: "approve", system: "HOME", cell: "approve.inbox", major: 1, view: "approve.inbox::Pending", area: Area::Main, order: 5, needs: "approve.inbox:read", when: Some("approve.inbox/Badge") },
    SlotLine { name: "cells", system: "CORE", cell: "site.cellmap", major: 1, view: "site.cellmap::Summary", area: Area::Side, order: 50, needs: "site.cellmap:read", when: None },
    SlotLine { name: "links", system: "HOME", cell: "home.shell", major: 1, view: "home::Links", area: Area::Side, order: 90, needs: "home.shell:open", when: None },
];
