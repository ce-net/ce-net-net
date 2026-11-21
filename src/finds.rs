//! Which cell answers which kind on the search page, as data beside opens.rs (repo:ce/docs/SEARCH-PAGE-2026-11-21.md
//! section 5). `search.typed@1` reads [`FINDS`]: for each line whose kind is in the plan and whose `needs` the viewer's
//! chain holds, it asks `cell`'s `op` as the viewer (a grant minted for exactly the viewer's rights in that cell) and
//! maps the typed answer to rows. A kind with no line is not searched; a viewer without a line's `needs` causes no call
//! to that cell and is never told it exists (default deny). When `net!` takes a `finds` line, this moves into the
//! systems and no reader changes.

/// One `finds` line. `kind` is the search.plan kind word (`factory`, `part`, ...).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Finds {
    pub kind: &'static str,
    pub cell: &'static str,
    pub major: u16,
    pub op: &'static str,
    /// the right the viewer's chain must hold for the call to be made at all
    pub needs: &'static str,
    /// this source's weight inside its kind (reciprocal rank fusion, section 6)
    pub weight: f32,
    pub deadline_ms: u32,
}

const fn f(kind: &'static str, cell: &'static str, major: u16, op: &'static str, needs: &'static str, weight: f32, deadline_ms: u32) -> Finds {
    Finds { kind, cell, major, op, needs, weight, deadline_ms }
}

/// Slice 1. Person (crm.people) and Twin (twin.registry) are slice 2: no line, so they are not asked.
pub const FINDS: &[Finds] = &[
    f("factory", "factory.registry", 1, "Search", "factory.registry:read", 1.0, 1500),
    f("org", "factory.registry", 1, "Search", "factory.registry:read", 0.8, 1500),
    f("part", "parts.inventory", 1, "List", "parts.inventory:read", 1.0, 1500),
    f("part", "parts.source", 1, "Source", "parts.source:source", 0.9, 1500),
    f("printer", "printers.health", 1, "Read", "printers.health:read", 1.0, 1500),
    f("tool", "tool.run", 1, "Tools", "tool.run:list", 1.0, 1500),
    f("cell", "cell.search", 1, "Search", "cell.search:search", 1.0, 1500),
    f("capability", "cell.search", 1, "Search", "cell.search:search", 0.8, 1500),
    f("doc", "search.hybrid", 1, "Find", "search.hybrid:find", 1.0, 3000),
];

/// The lines of `FINDS`.
pub fn finds() -> &'static [Finds] {
    FINDS
}

/// The kinds a line may name (search.plan's `Kind`, as words).
pub const KINDS: &[&str] = &["factory", "part", "printer", "tool", "cell", "capability", "person", "org", "doc", "twin"];

/// The ce-ref kind a result of each search kind carries (`cap:` for capability, `repo:` for most docs).
pub fn ref_kind(kind: &str) -> &'static str {
    match kind {
        "factory" => "factory",
        "org" => "org",
        "part" => "part",
        "printer" => "printer",
        "tool" => "tool",
        "cell" => "cell",
        "capability" => "cap",
        "person" => "person",
        "twin" => "twin",
        _ => "repo",
    }
}
