use ce_net_net::*;

const FOREMAN_SYSTEMS: [&str; 9] = ["TALK", "LANES", "BOARD", "VOICE", "MEMORY", "OUTBOUND", "WECHAT", "PROMPTS", "SCHEDULE"];

#[test]
fn systems_every_unpinned_cell_twice_over_the_serving_nodes() {
    let names: Vec<&str> = CE_NET.systems().iter().skip(1).map(|s| s.name).collect();
    let mut want = vec!["CORE", "APPROVALS", "CONTROL_ROOM", "FOREMAN"];
    want.extend(FOREMAN_SYSTEMS);
    want.push("SEARCH");
    assert_eq!(names, want);
    for (sys, c) in CE_NET.all_cells() {
        match c.on {
            None => assert!(c.replicas >= 2, "{sys}: {} wants {} replica(s)", c.cell.name, c.replicas),
            Some(n) => assert_eq!((c.cell.name, n), ("wechat.bot", "macbook"), "{sys}: only wechat.bot is pinned"),
        }
    }
    for s in CE_NET.systems().iter().skip(1).filter(|s| s.name != "WECHAT") {
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

#[test]
fn every_cell_in_exactly_one_system() {
    let all = CE_NET.all_cells();
    for (sys, c) in &all {
        let n = all.iter().filter(|(_, o)| o.cell.name == c.cell.name).count();
        assert_eq!(n, 1, "{} is in {n} systems ({sys} among them)", c.cell.name);
        assert_eq!(system_of(c.cell.name), Some(*sys));
    }
}

#[test]
fn foreman_is_the_include_of_its_systems() {
    let parts: Vec<&str> = FOREMAN.parts.iter().map(|p| p.name).collect();
    assert_eq!(parts, ["TALK", "LANES", "BOARD", "VOICE", "MEMORY", "OUTBOUND", "PROMPTS", "SCHEDULE"]);
    let under: Vec<&str> = FOREMAN.systems().iter().skip(1).map(|s| s.name).collect();
    assert_eq!(under, FOREMAN_SYSTEMS);
    assert_eq!(FOREMAN.tab.map(|t| t.route), Some("/foreman/"));
}

#[test]
fn no_leader_or_record_cell_off_a_datacenter_node() {
    for (sys, c) in CE_NET.all_cells() {
        let nodes = nodes_of(c.cell.name);
        assert!(!nodes.is_empty(), "{}", c.cell.name);
        for n in &nodes {
            let class = class_of(n).unwrap_or_else(|| panic!("{sys}: node {n} has no class"));
            if is_leader(c.cell.name) || has_records(c.cell.name) {
                assert_eq!(class, Class::Datacenter, "{sys}: {} (leader or records) may run on {n} ({class:?})", c.cell.name);
            }
        }
    }
    // Debian holds lane.host and nothing else.
    let on_debian: Vec<&str> = CE_NET.all_cells().iter().map(|(_, c)| c.cell.name).filter(|c| nodes_of(c).contains(&"debian")).collect();
    assert_eq!(on_debian, ["lane.host"]);
    assert_eq!(class_of("debian"), Some(Class::Worker));
}

#[test]
fn placement_aims_at_foreman_ops_1_first() {
    assert_eq!(SERVING, ["foreman-ops-1", "leif-desk"]);
    assert_eq!(NODES[0].name, "foreman-ops-1");
    for s in CE_NET.systems().iter().skip(1).filter(|s| !s.nodes.is_empty() && s.name != "WECHAT") {
        assert_eq!(s.nodes[0], "foreman-ops-1", "{}", s.name);
    }
    for n in SERVING {
        assert_eq!(class_of(n), Some(Class::Datacenter));
    }
}

#[test]
fn planned_cells_are_described_and_slice_1_is_live() {
    for p in PLANNED {
        assert!(system_of(p.name).is_some(), "{} planned but in no system", p.name);
    }
    for want in ["lane.inbox", "lane.route", "prompt.agents", "prompt.lead", "prompt.situation"] {
        assert!(is_planned(want), "{want}");
    }
    for live in ["apps.shell", "approve.inbox", "site.cellmap", "foreman.shell", "talk.messages", "status.cards"] {
        assert!(system_of(live).is_some() && !is_planned(live), "{live}");
    }
    for s in CE_NET.systems() {
        for need in s.needs {
            assert!(CE_NET.systems().iter().any(|o| o.name == need.name), "{} needs {}, not in CE_NET", s.name, need.name);
        }
    }
}
