//! The places beside the tabs: every app route of the deployed gate (repo:ce-fleet door/gate.py, snapshot 55013253)
//! is a tab or an app place, so no old system goes missing from the entry.
use ce_net_net::{app, ref_ok, slug_of, APPS, CE_NET};

/// PORT_PATHS less NOT_APP_ROUTES, photos-upload and x; APP_PATHS less retired ring; GZ_APPS; the work paths.
const GATE: &[&str] = &[
    "vote", "port-twin", "tree", "ui-examples", "doc", "approvals", "core", "search", "feed", "home", "conversations",
    "architecture", "books", "twinton", "pce", "print-farm", "print-shop", "photos", "console", "trana", "launchpad",
    "schedule", "fleet", "troublemaker", "disassembly", "room", "projects", "media", "microduck", "huey", "huey-sound",
    "globe", "twin-ops", "sourcing", "box-lab", "parts", "ocean", "canvas", "ai", "business", "cad", "gev", "ledframe",
    "ledframe-v3", "tandem", "factory-voice", "assembly-guide", "docs", "lanes", "yuncheng-twin", "foreman", "voice",
    "attn", "brief", "devices", "leif", "secrets", "grant", "control-room", "devenv", "one",
];

#[test]
fn every_gate_app_is_a_tab_or_a_place() {
    let tabs: Vec<&str> = CE_NET.tabs().into_iter().map(|(_, t)| slug_of(t.route)).collect();
    for g in GATE {
        assert!(tabs.contains(g) || app(g).is_some(), "{g} is neither a tab nor a place");
    }
    // the old pages Leif reached them by
    for g in ["shell", "launcher", "maps", "me", "delegate", "runners"] {
        assert!(app(g).is_some(), "{g}");
    }
}

#[test]
fn places_are_unique_valid_refs_on_the_site() {
    let tabs: Vec<&str> = CE_NET.tabs().into_iter().map(|(_, t)| slug_of(t.route)).collect();
    for (i, a) in APPS.iter().enumerate() {
        assert!(ref_ok(&format!("app:{}", a.seg)), "{}", a.seg);
        assert!(!tabs.contains(&a.seg), "{} repeats a tab", a.seg);
        assert!(APPS[..i].iter().all(|b| b.seg != a.seg), "{} twice", a.seg);
        assert!(a.href.starts_with('/') && !a.href.starts_with("//") || a.href == "https://maps.ce-net.com/", "{}", a.href);
        assert!(!a.title.is_empty());
    }
    for s in ["voice", "leif", "one", "devices", "brief", "attn", "runners", "shell"] {
        assert!(app(s).unwrap().owner, "{s} is the owner's alone");
    }
}
