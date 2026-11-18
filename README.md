# ce-net-net

ce-net as cell systems (repo:ce/docs/SITES-ON-CELLS.md section 2): `CORE`, `APPROVALS`, `CONTROL_ROOM`, `FOREMAN`,
each one `net!` value (repo:cell-stack crates/cell-net: `include`, `needs [SYSTEM]`, one `tab` per system), and
`CE_NET`, their sum. Approvals is its own system, not a part of Foreman.

The tabs of apps.ce-net.com come only from here: repo:apps-shell keeps a tab for a viewer when their chain holds its
`needs` and its `shell` cell answers HealthOp by name. repo:site-cellmap shows every cell of every system per node.

Foreman is the `include` of TALK, LANES, BOARD, VOICE, MEMORY, OUTBOUND (with WECHAT, the Mac), PROMPTS, SCHEDULE
(repo:ce/docs/FOREMAN-ON-CELLS.md step 1, description only). `NODES` gives each node a class: foreman-ops-1 and
leif-desk `Datacenter`, Debian `Worker` (lane.host only). `PLANNED`, `LEADERS`, `RECORDS`, `RESIDENT` mark cells;
a name no system places fails the build.

CRM (crm.people, crm.promises, crm.calls), REFS (ref.resolve, ref.pin, open.with) and HOME (home.shell, tab "Home"
`/home/` order 1, needs `home.shell:open`) come from repo:ce/docs/CE-REF-OPENERS-AND-HOME-2026-11-18.md. `cell-net`
takes no `opens` or `slot` lines yet, so `src/opens.rs` lists them beside the systems with the same fields: `opens()`
is the opener table open.with@1 reads (pattern, mode, cell, view, rank, needs) and `HOME_SLOTS` the Home slots
home.shell@1 lays out. Two openers equal on pattern, mode and rank fail `cargo test`.

`cargo test`: every cell in exactly one system, no leader or record cell off a datacenter node, placement
foreman-ops-1 first, every unpinned cell wants two replicas, tabs in order.
Apache-2.0.
