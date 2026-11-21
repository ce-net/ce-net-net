//! ce-net as cell systems (repo:ce/docs/SITES-ON-CELLS.md section 2). Each system is its own `net!` value: its cells,
//! the replicas it wants over the serving nodes, and the one tab its shell offers. `CE_NET` is their sum.
//!
//! There is no tab list anywhere else. apps.shell takes `CE_NET.tabs()` and keeps a tab for a viewer only when their
//! chain holds the tab's `needs` and its `shell` cell answers HealthOp by name (section 3). Approvals is its own
//! system here, not a part of Foreman: Foreman's tab is one tab among the others.
//!
//! SEARCH is one box over everything: search.hybrid (private and internal, filtered by the viewer's exact read grants)
//! and search.mesh (every node's own index, merged). Its tab's shell is search.hybrid itself: the tab is live exactly
//! while the index answers. leif-desk (2 cores, 3 GB) cannot hold search.hybrid's index yet, so it runs 1 of 2.
//!
//! Serving nodes: foreman-ops-1 first, leif-desk the second datacenter replica (NODES, class `Datacenter`). Debian is
//! class `Worker` and holds only lane.host; the Mac holds wechat.bot only. `replicas 2` is what each cell wants;
//! site.cellmap shows wanted against running, per node, and PLANNED cells as planned.
#![forbid(unsafe_code)]

pub use cell_net::{CellRef, CellSpec, Network, Tab};
use cell_net::net;

pub mod apps;
pub mod finds;
pub mod links;
pub mod opens;
pub use apps::{app, App, APPS};
pub use finds::{finds, Finds, FINDS};
pub use links::{link, ref_ok, slug_of, Link, LinkMode};
pub use opens::{opens, Area, Mode, Opens, SlotLine, HOME_SLOTS};

/// The nodes that serve apps.ce-net.com.
pub const SERVING: &[&str] = &["foreman-ops-1", "leif-desk"];

net! { pub CORE;
    node foreman-ops-1; node leif-desk;
    cell state.store@1 replicas 2;
    cell account.session@1 replicas 2 needs [state.store@1];
    cell account.passkey@1 replicas 2 needs [state.store@1];
    cell account.signin@1 replicas 2 needs [account.session@1, account.passkey@1];
    cell cell.install@3 replicas 2;
    cell apps.shell@1 replicas 2 needs [account.session@1];
    cell site.cellmap@1 replicas 2 needs [cell.install@3];
    tab "Core" route "/core/" order 90 needs "site.cellmap:read" shell site.cellmap@1;
}

net! { pub APPROVALS;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell grant.bundle@1 replicas 2;
    cell needs.list@1 replicas 2;
    cell feed.approve@1 replicas 2;
    cell approve.inbox@1 replicas 2 needs [grant.bundle@1, needs.list@1];
    tab "Approvals" route "/approvals/" order 20 needs "approve.inbox:read" shell approve.inbox@1;
}

net! { pub CONTROL_ROOM;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell cr.keys@2 replicas 2;
    cell cr.devices@2 replicas 2 needs [cell.install@3];
    cell cr.audit@2 replicas 2;
    cell cr.shell@2 replicas 2 needs [cr.keys@2, cr.devices@2, cr.audit@2];
    tab "Control room" route "/control-room/" order 30 needs "cr.shell:open" shell cr.shell@2;
}

// Foreman as eight cell systems (repo:ce/docs/FOREMAN-ON-CELLS.md section 2, step 1: description only, nothing is
// installed or re-routed by this). Every system aims at foreman-ops-1 first; leif-desk is the second datacenter
// replica (none is installed there until card gb-b3a1c4df346e3d0e is signed); Debian is class `worker` and holds
// only `lane.host` (see RESIDENT). Cells in PLANNED are not built yet: in the description, not in the plan.

net! { pub TALK;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell talk.messages@1 replicas 2;
    cell feed.view@1 replicas 2;
    cell feed.thread@1 replicas 2;
    cell feed.search@1 replicas 2;
    cell feed.say@1 replicas 2;
    cell feed.stream@1 replicas 2;
}

net! { pub LANES;
    needs [CORE, TALK];
    node foreman-ops-1; node leif-desk;
    cell lane.registry@1 replicas 2;
    cell lane.inbox@1 replicas 2;
    cell lane.route@1 replicas 2;
    cell lane.launch@1 replicas 2;
    cell lane.host@1 replicas 2;
    cell lane.board@1 replicas 2;
}

net! { pub BOARD;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell status.cards@1 replicas 2;
    cell goal.rows@1 replicas 2;
    cell todo.board@1 replicas 2;
    cell handoff@1 replicas 2;
}

net! { pub VOICE;
    needs [CORE, TALK];
    node foreman-ops-1; node leif-desk;
    cell voice.session@1 replicas 2;
    cell voice.transcribe@1 replicas 2;
    cell voice.modes@1 replicas 2;
    cell voice.brief@1 replicas 2;
    cell voice.phone-line@1 replicas 2;
}

net! { pub MEMORY;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell memory.db@1 replicas 2;
    cell memory.atom-write@1 replicas 2;
    cell memory.search@1 replicas 2;
    cell memory.hook@1 replicas 2;
}

// The one cell that runs on Leif's Mac (his WeChat login). Its own system so the Mac is never a node OUTBOUND's
// other cells, the gate among them, could be placed on.
net! { pub WECHAT;
    needs [CORE];
    node macbook { cell wechat.bot@1; }
}

net! { pub OUTBOUND;
    include [WECHAT];
    needs [CORE, APPROVALS];
    node foreman-ops-1; node leif-desk;
    cell outbound.gate@1 replicas 2;
    cell mail.send@1 replicas 2;
    cell mail.outreach-send@1 replicas 2;
    cell leif.notify@1 replicas 2;
}

net! { pub PROMPTS;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell prompt.agents@1 replicas 2;
    cell prompt.lead@1 replicas 2;
    cell prompt.situation@1 replicas 2;
}

net! { pub SCHEDULE;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell cell.tick@1 replicas 2;
}

net! { pub FOREMAN;
    include [TALK, LANES, BOARD, VOICE, MEMORY, OUTBOUND, PROMPTS, SCHEDULE];
    needs [CORE, APPROVALS];
    node foreman-ops-1; node leif-desk;
    cell work.show@1 replicas 2;
    cell foreman.shell@1 replicas 2 needs [work.show@1];
    tab "Foreman" route "/foreman/" order 10 needs "foreman.shell:open" shell foreman.shell@1;
}

// net! allows one tab a system and wants the tab's shell in the same system, so the Feed tab is its own system (needs
// FOREMAN) and holds feed.view@2, its shell. A shadow of the feed in /foreman/: the old feed stays live
// (repo:ce docs/FOREMAN-AS-CELLS-V2-2026-11-18.md slice 1, package B).
net! { pub FEED;
    needs [FOREMAN];
    node foreman-ops-1; node leif-desk;
    cell feed.view@2 replicas 2;
    tab "Feed" route "/feed/" order 11 needs "feed.view:read" shell feed.view@2;
}

// The frontend as cells (repo:ce/docs/UI-AS-CELLS-2026-11-21.md, slice 1): site.serve@1 holds http.serve for
// apps.ce-net.com/x/ and has no data right, page.compose@2 (the entry route and `?open=`, section 9) fits the pieces a viewer's chain covers, feed.render@1 is the
// feed's render:html piece (feed.view@2 stays the data and the live view). No tab yet: /x/feed/ is a shadow beside /feed/
// and /foreman/. Declared over both serving nodes like every system; installed on foreman-ops-1 only until step 2
// (win-b591's mesh-wide install, repo:site-serve docs/SLICE-1.md).
net! { pub FRONTEND;
    needs [FEED];
    node foreman-ops-1; node leif-desk;
    cell site.serve@1 replicas 2;
    cell page.compose@2 replicas 2;
    cell feed.render@1 replicas 2;
}

// The person's short list and the business place (repo:ce docs/CRITICAL-DECISIONS-2026-11-21.md Part 2, sections 16
// to 19). person.now@1 serves person.backlog@1 and person.handoff@1 from its process; operator.run@1 serves
// operator.track, operator.profile and operator.report; business.view@1 is the render:html of the business.monitor cell
// beside contracts.list@1, finance.invoices@1, finance.revenue@1 and operations.now@1. Both tabs open in place in the
// entry (/x/?open=app:now, app:business-monitor; the old /business/ page stays a place of apps.rs). Declared over both serving nodes like every system; installed on foreman-ops-1 first (HANDOFF in each repo).
net! { pub NOW;
    needs [FRONTEND];
    node foreman-ops-1; node leif-desk;
    cell person.now@1 replicas 2;
    cell operator.run@1 replicas 2;
    tab "Now" route "/now/" order 2 needs "person.now:read" shell person.now@1;
}

net! { pub BUSINESS;
    needs [NOW];
    node foreman-ops-1; node leif-desk;
    cell business.view@1 replicas 2;
    tab "Business monitor" route "/business-monitor/" order 3 needs "business.view:html" shell business.view@1;
}

net! { pub SEARCH;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell search.hybrid@1 replicas 2;
    cell search.mesh@1 replicas 2;
    cell cell.search@1 replicas 2;
    // the typed search page (repo:ce/docs/SEARCH-PAGE-2026-11-21.md): words to intents, then the sources asked as
    // the viewer (finds.rs) and merged; both stateless
    cell search.plan@1 replicas 2;
    cell search.typed@1 replicas 2 needs [search.plan@1];
    tab "Search" route "/search/" order 15 needs "search.hybrid:find" shell search.hybrid@1;
}

// The CRM cells (live on foreman-ops-1): their record views open `person:`, `promise:`, `call:` refs, and calls and
// promises offer Home slots (opens.rs).
net! { pub CRM;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell crm.people@1 replicas 2;
    cell crm.promises@1 replicas 2;
    cell crm.calls@2 replicas 2;
}

// The things the search page finds as typed results (SEARCH-PAGE-2026-11-21 section 4): factories and suppliers,
// parts in stock and their offers, printers, tools. Each cell owns its records and answers for the viewer itself.
// Declared at the serving nodes like every system; where they run today (factory.registry's records and
// parts.inventory are on debian only, factory.registry's ops-1 replica is empty) is the open point of the HANDOFF.
net! { pub MAKE;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell factory.registry@1 replicas 2;
    cell parts.inventory@1 replicas 2;
    cell parts.source@1 replicas 2;
    cell printers.health@1 replicas 2;
    cell tool.run@1 replicas 2;
}

// ce-ref, the only link (repo:ce/docs/CE-REF-OPENERS-AND-HOME-2026-11-18.md): where a thing is (ref.resolve), its
// bytes by hash (ref.pin), and which view opens it for this viewer (open.with, reading opens.rs).
net! { pub REFS;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell ref.resolve@1 replicas 2;
    cell ref.pin@1 replicas 2;
    cell open.with@1 replicas 2;
}

// Leif's Home: home.shell owns a layout and nothing else; every panel is another cell's view (HOME_SLOTS, opens.rs).
net! { pub HOME;
    needs [CORE, APPROVALS, REFS];
    node foreman-ops-1; node leif-desk;
    cell home.shell@1 replicas 2;
    tab "Home" route "/home/" order 1 needs "home.shell:open" shell home.shell@1;
}

// The Architecture atlas (repo:ce/docs/ARCHITECTURE-ATLAS-APP-2026-11-18.md 7.4): arch.map draws ce-net from what its
// cells report and ships its own view; arch.notes keeps the notes on it as rows. The tab's shell is arch.map itself; it
// emerges for a viewer holding arch.map:see once arch.map answers HealthOp by name. The gate route /architecture/ is
// added in a quiet window; until then apps.shell opens the tab's view under Core (/core/?view=architecture).
// arch.map also reads cell.registry@2, cells.model@1, cells.graph@1, cell.search@1 and call.pulse@1 (its cell.toml
// depends_on): net! takes a need only when a system places it, and no system places those yet, so they are not
// written here; the design's line names cell.registry@2 and cells.model@1 and gains them when they are placed.
net! { pub ARCHITECTURE;
    needs [CORE, REFS];
    node foreman-ops-1; node leif-desk;
    cell arch.map@1 replicas 2 needs [site.cellmap@1, arch.notes@1];
    cell arch.notes@1 replicas 2 needs [state.store@1];
    tab "Architecture" route "/architecture/" order 16 needs "arch.map:see" shell arch.map@1;
}

// Every AI conversation of the realm in one list (repo:ce/docs/AI-AS-CELLS-2026-11-18.md slice 2): the tab's shell
// is ai.session itself; it emerges for a viewer holding ai.session:list once ai.session answers HealthOp by name.
net! { pub CONVERSATIONS;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell ai.session@1 replicas 2;
    tab "Conversations" route "/conversations/" order 12 needs "ai.session:list" shell ai.session@1;
}

// The ops cells that deploy and check cells (repo:ce/docs/OPS-CELLS-2026-11-21.md 8.6). No tab: the panel is cr.ops in
// the control room. A cell enters OPS in the commit of its install handoff, one per cell, because health.check and
// net.plan treat every placed cell as wanted from then on; the order of `needs` is net.plan's and the install order.
// node.place also provides node.facts@1 (its second capability, same process).
net! { pub OPS;
    needs [CORE, SCHEDULE];
    node foreman-ops-1; node leif-desk;
    cell cli.provider@1 replicas 2;
    cell node.place@1 replicas 2;
    cell health.check@1 replicas 2 needs [state.store@1];
    cell drift.check@1 replicas 2 needs [state.store@1, health.check@1];
}

net! { pub CE_NET; include [HOME, CORE, APPROVALS, CONTROL_ROOM, FOREMAN, FEED, FRONTEND, SEARCH, CRM, REFS, CONVERSATIONS, ARCHITECTURE, MAKE, NOW, BUSINESS, OPS]; }

/// The system a cell belongs to, by its api name.
pub fn system_of(api: &str) -> Option<&'static str> {
    CE_NET.all_cells().into_iter().find(|(_, c)| c.cell.name == api).map(|(s, _)| s)
}

/// What a node is for (FOREMAN-ON-CELLS rule 4). Only `Datacenter` holds leaders, records, gates and schedules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// foreman-ops-1 (primary), leif-desk (second replica)
    Datacenter,
    /// Debian: lanes and compute through `lane.host`; losing it loses running agent sessions and nothing else
    Worker,
    /// Leif's own machine: one pinned cell that needs his login, nothing of record
    Device,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub name: &'static str,
    pub class: Class,
}

/// Every node any system names, in placement order: foreman-ops-1 first.
pub const NODES: &[Node] = &[
    Node { name: "foreman-ops-1", class: Class::Datacenter },
    Node { name: "leif-desk", class: Class::Datacenter },
    Node { name: "debian", class: Class::Worker },
    Node { name: "macbook", class: Class::Device },
];

pub fn class_of(node: &str) -> Option<Class> {
    NODES.iter().find(|n| n.name == node).map(|n| n.class)
}

const fn c(name: &'static str, major: u32) -> CellRef {
    CellRef { name, major }
}

/// Declared, not built yet: shown by site.cellmap as planned, never in `net.plan`'s install set. lane.inbox and
/// lane.route run today on ops-1 with their old contracts; the typed @1 contracts of FOREMAN-ON-CELLS 2.1 are planned.
pub const PLANNED: &[CellRef] = &[
    c("lane.registry", 1), c("lane.inbox", 1), c("lane.route", 1), c("lane.host", 1), c("lane.board", 1),
    c("handoff", 1), c("voice.session", 1), c("outbound.gate", 1), c("leif.notify", 1),
    c("prompt.agents", 1), c("prompt.lead", 1), c("prompt.situation", 1),
];

/// Single-writer cells behind the ce-replica lease.
pub const LEADERS: &[CellRef] = &[
    c("lane.registry", 1), c("lane.launch", 1), c("voice.session", 1), c("outbound.gate", 1), c("work.show", 1),
];

/// Cells that own state of record (`#[record(cell = ..)]` rows in state.store, or files until moved).
pub const RECORDS: &[CellRef] = &[
    c("state.store", 1), c("talk.messages", 1), c("lane.registry", 1), c("lane.inbox", 1), c("lane.route", 1),
    c("status.cards", 1), c("goal.rows", 1), c("todo.board", 1), c("handoff", 1), c("voice.session", 1),
    c("memory.db", 1), c("outbound.gate", 1), c("prompt.agents", 1), c("prompt.lead", 1), c("cell.tick", 1),
    c("work.show", 1),
];

/// Cells that run once on every node of these classes, besides their replicas.
pub const RESIDENT: &[(CellRef, &[Class])] = &[
    (c("lane.host", 1), &[Class::Worker, Class::Datacenter]),
    (c("cell.tick", 1), &[Class::Datacenter]),
];

const fn all_placed(list: &[CellRef]) -> bool {
    let mut i = 0;
    while i < list.len() {
        if !CE_NET.places(list[i].name, list[i].major) {
            return false;
        }
        i += 1;
    }
    true
}

// A name in a list above that no system places fails the build.
const _: () = assert!(all_placed(PLANNED) && all_placed(LEADERS) && all_placed(RECORDS));
const _: () = {
    let mut i = 0;
    while i < RESIDENT.len() {
        assert!(CE_NET.places(RESIDENT[i].0.name, RESIDENT[i].0.major));
        i += 1;
    }
};

fn listed(list: &[CellRef], api: &str) -> bool {
    list.iter().any(|c| c.name == api)
}
pub fn is_planned(api: &str) -> bool {
    listed(PLANNED, api)
}
pub fn is_leader(api: &str) -> bool {
    listed(LEADERS, api)
}
pub fn has_records(api: &str) -> bool {
    listed(RECORDS, api)
}

/// Every node a cell of `CE_NET` may run on: its pin, else its system's nodes, plus every node of its resident classes.
pub fn nodes_of(api: &str) -> Vec<&'static str> {
    let Some((sys, spec)) = CE_NET.all_cells().into_iter().find(|(_, c)| c.cell.name == api) else { return Vec::new() };
    let sys = CE_NET.systems().into_iter().find(|s| s.name == sys).expect("all_cells names its system");
    let mut out: Vec<&'static str> = match spec.on {
        Some(n) => vec![n],
        None => sys.nodes.to_vec(),
    };
    for (_, classes) in RESIDENT.iter().filter(|(r, _)| r.name == api) {
        for n in NODES.iter().filter(|n| classes.contains(&n.class)) {
            if !out.contains(&n.name) {
                out.push(n.name);
            }
        }
    }
    out
}
