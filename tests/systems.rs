use ce_net_net::*;

#[test]
fn five_systems_every_cell_twice_over_the_serving_nodes() {
    let names: Vec<&str> = CE_NET.systems().iter().skip(1).map(|s| s.name).collect();
    assert_eq!(names, ["CORE", "APPROVALS", "CONTROL_ROOM", "FOREMAN", "SEARCH"]);
    for (sys, c) in CE_NET.all_cells() {
        assert!(c.replicas >= 2, "{sys}: {} wants {} replica(s)", c.cell.name, c.replicas);
    }
    for s in CE_NET.systems().iter().skip(1) {
        assert_eq!(s.nodes, SERVING, "{}", s.name);
    }
}

#[test]
fn approvals_is_its_own_system_and_foreman_one_tab_among_others() {
    assert_eq!(system_of("approve.inbox"), Some("APPROVALS"));
    assert_eq!(system_of("grant.bundle"), Some("APPROVALS"));
    assert!(!FOREMAN.cells.iter().any(|c| c.cell.name.starts_with("approve.") || c.cell.name == "grant.bundle"));
    let tabs: Vec<(&str, &str)> = CE_NET.tabs().iter().map(|(s, t)| (*s, t.route)).collect();
    assert_eq!(tabs, [("FOREMAN", "/foreman/"), ("SEARCH", "/search/"), ("APPROVALS", "/approvals/"), ("CONTROL_ROOM", "/control-room/"), ("CORE", "/core/")]);
}
