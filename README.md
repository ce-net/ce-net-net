# ce-net-net

ce-net as cell systems (repo:ce/docs/SITES-ON-CELLS.md section 2): `CORE`, `APPROVALS`, `CONTROL_ROOM`, `FOREMAN`,
each one `net!` value (repo:cell-stack crates/cell-net: `include`, `needs [SYSTEM]`, one `tab` per system), and
`CE_NET`, their sum. Approvals is its own system, not a part of Foreman.

The tabs of apps.ce-net.com come only from here: repo:apps-shell keeps a tab for a viewer when their chain holds its
`needs` and its `shell` cell answers HealthOp by name. repo:site-cellmap shows every cell of every system per node.

`cargo test`: four systems, every cell wants two replicas over foreman-ops-1 and leif-desk, tabs in order.
Apache-2.0.
