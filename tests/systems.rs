use ce_net_net::*;

const FOREMAN_SYSTEMS: [&str; 9] = ["TALK", "LANES", "BOARD", "VOICE", "MEMORY", "OUTBOUND", "WECHAT", "PROMPTS", "SCHEDULE"];

#[test]
fn systems_every_unpinned_cell_twice_over_the_serving_nodes() {
    let names: Vec<&str> = CE_NET.systems().iter().skip(1).map(|s| s.name).collect();
    let mut want = vec!["HOME", "CORE", "APPROVALS", "CONTROL_ROOM", "FOREMAN"];
    want.extend(FOREMAN_SYSTEMS);
    want.extend(["FEED", "SEARCH", "CRM", "REFS", "MAKE"]);
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
    assert_eq!(tabs, [("HOME", "/home/"), ("FOREMAN", "/foreman/"), ("FEED", "/feed/"), ("SEARCH", "/search/"), ("APPROVALS", "/approvals/"), ("CONTROL_ROOM", "/control-room/"), ("CORE", "/core/")]);
}

#[test]
fn every_cell_in_exactly_one_system() {
    let all = CE_NET.all_cells();
    for (sys, c) in &all {
        // a cell is its name and major: feed.view@1 (TALK, frozen) and feed.view@2 (FEED) are two cells of one process
        let n = all.iter().filter(|(_, o)| o.cell.name == c.cell.name && o.cell.major == c.cell.major).count();
        assert_eq!(n, 1, "{}@{} is in {n} systems ({sys} among them)", c.cell.name, c.cell.major);
        assert_eq!(all.iter().find(|(_, o)| o.cell.name == c.cell.name && o.cell.major == c.cell.major).map(|(s, _)| *s), Some(*sys));
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

/// The Feed tab (feed.view@2) is offered only to a chain holding `feed.view:read`; its shell is the cell itself, and
/// the cell belongs to FEED (net! wants the shell in the tab's own system).
#[test]
fn the_feed_tab_is_offered_only_with_feed_view_read() {
    let feed: Vec<_> = CE_NET.tabs().into_iter().filter(|(_, t)| t.route == "/feed/").collect();
    assert_eq!(feed.len(), 1);
    let (sys, t) = &feed[0];
    assert_eq!((*sys, t.title, t.needs, t.shell.name, t.order), ("FEED", "Feed", "feed.view:read", "feed.view", 11));
    assert!(CE_NET.all_cells().iter().any(|(s, c)| *s == "FEED" && c.cell.name == "feed.view" && c.cell.major == 2 && c.replicas == 2));
    // what apps.shell does with `needs`: ce-cap's cover rule over the viewer's abilities
    let covers = |abilities: &[&str], right: &str| abilities.iter().any(|a| *a == right || *a == "*" || a.strip_suffix(":*").map_or(false, |d| right.starts_with(&format!("{d}:"))));
    assert!(covers(&["feed.view:read"], t.needs));
    assert!(covers(&["feed.view:*"], t.needs));
    for no in [&["feed.view:mark"][..], &["feed.view:health"], &["foreman.shell:open"], &["search.hybrid:find"], &[]] {
        assert!(!covers(no, t.needs), "{no:?} must not see the Feed tab");
    }
}
