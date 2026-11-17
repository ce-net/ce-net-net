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

net! { pub SEARCH;
    needs [CORE];
    node foreman-ops-1; node leif-desk;
    cell search.hybrid@1 replicas 2;
    cell search.mesh@1 replicas 2;
    tab "Search" route "/search/" order 15 needs "search.hybrid:find" shell search.hybrid@1;
}

net! { pub CE_NET; include [CORE, APPROVALS, CONTROL_ROOM, FOREMAN, SEARCH]; }

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
