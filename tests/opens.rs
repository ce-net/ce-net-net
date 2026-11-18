use ce_net_net::*;

/// Two openers equal on pattern, mode and rank would leave open.with to guess: a description error (design 3.2 step 5).
#[test]
fn no_two_openers_tie() {
    let all = opens();
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert!(
                !(a.pattern == b.pattern && a.mode == b.mode && a.rank == b.rank),
                "{} and {} both open {:?} {:?} at rank {}",
                a.view,
                b.view,
                a.pattern,
                a.mode,
                a.rank
            );
        }
    }
}

/// Every opener's cell is placed by a system, or is the shell itself.
#[test]
fn every_opener_names_a_placed_cell() {
    for o in opens() {
        assert!(system_of(o.cell).is_some(), "{} opens {:?} in {}, which no system places", o.view, o.pattern, o.cell);
        assert!(o.view.contains("::"), "{}: view is <bundle>::<View>", o.view);
        assert!(o.needs.is_empty() || o.needs.contains(':') && !o.needs.contains('*'), "{}: needs one exact right", o.view);
    }
}

/// The floor: anything with a kind opens somewhere (name, size, hash), never a dead end.
#[test]
fn the_floor_exists_and_is_lowest() {
    let floor: Vec<&Opens> = REFS_FLOOR.iter().collect();
    assert_eq!(floor.len(), 1);
    assert!(opens().iter().filter(|o| o.pattern == "*").all(|o| o.rank <= 1));
}

const REFS_FLOOR: &[Opens] = &[Opens { pattern: "*", mode: Mode::View, cell: "ref.resolve", major: 1, view: "ref.view::Raw", rank: 1, needs: "ref.resolve:read" }];

/// The slice-1 table of the design, line by line.
#[test]
fn slice_1_table() {
    let view_of = |ty: &str| -> Vec<&'static str> {
        let mut c: Vec<Opens> = opens().into_iter().filter(|o| pattern_ok(o.pattern, ty)).collect();
        c.sort_by_key(|o| (std::cmp::Reverse(spec(o.pattern)), std::cmp::Reverse(o.rank)));
        c.iter().map(|o| o.view).collect()
    };
    assert_eq!(view_of("code/rust")[0], "ref.view::Text");
    assert_eq!(view_of("text/plain")[0], "ref.view::Text");
    assert_eq!(view_of("doc/markdown")[0], "ref.view::Text");
    for k in ["person", "promise", "call", "meet"] {
        assert_eq!(view_of(k)[0], "ref.view::Record", "{k}");
    }
    assert_eq!(view_of("cell")[0], "site.cellmap::Cell");
    assert_eq!(view_of("node")[0], "site.cellmap::Node");
    assert_eq!(view_of("assembly")[0], "apps.shell::Port");
    assert_eq!(view_of("map")[0], "apps.shell::Port");
    assert_eq!(view_of("web")[0], "apps.shell::Outside");
    assert_eq!(view_of("view")[0], "apps.shell::Mount");
    assert_eq!(view_of("*"), ["ref.view::Raw"]);
    assert_eq!(view_of("image/png"), ["ref.view::Raw"]);
}

fn pattern_ok(p: &str, ty: &str) -> bool {
    p == "*" || p == ty || p.strip_suffix("/*").is_some_and(|h| ty.strip_prefix(h).is_some_and(|t| t.starts_with('/')))
}
fn spec(p: &str) -> u8 {
    if p == "*" {
        0
    } else if p.ends_with("/*") {
        1
    } else {
        2
    }
}

/// Home slots: each names a placed cell, an exact right, and no two share a name.
#[test]
fn home_slots() {
    let names: Vec<&str> = HOME_SLOTS.iter().map(|s| s.name).collect();
    assert_eq!(names, ["calls", "promises", "approve", "cells", "links"]);
    for s in HOME_SLOTS {
        assert!(system_of(s.cell).is_some(), "slot {} reads {}, which no system places", s.name, s.cell);
        assert!(s.needs.contains(':') && !s.needs.contains('*'), "slot {}", s.name);
    }
    let links = HOME_SLOTS.iter().find(|s| s.name == "links").unwrap();
    assert_eq!(links.needs, "home.links:read", "pinned links are their own right, not the Home tab's (CE-REF-OPENERS-AND-HOME 5.1)");
    let tab = HOME.tab.expect("HOME offers a tab");
    assert_eq!((tab.title, tab.route, tab.order, tab.needs, tab.shell.name), ("Home", "/home/", 1, "home.shell:open", "home.shell"));
    assert_eq!(CE_NET.tabs()[0].1.route, "/home/", "Home is the first tab");
}
