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
//! Serving nodes: foreman-ops-1 and leif-desk (the Mac votes and holds a state.store replica only; Debian rejoins as a
//! standby). `replicas 2` is what each cell wants; site.cellmap shows wanted against running, per node.
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

net! { pub FOREMAN;
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
