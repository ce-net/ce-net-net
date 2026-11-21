//! The link table of the one entrypoint (repo:ce/docs/UI-AS-CELLS-2026-11-21.md 9.4), golden: every row as written.
use ce_net_net::*;

fn to(path: &str, query: &str) -> Option<(String, LinkMode)> {
    link(path, query).map(|l| (links::target("", &l, query), l.mode))
}

#[test]
fn links_every_row_maps_as_written() {
    use LinkMode::*;
    let golden: &[(&str, &str, &str, LinkMode)] = &[
        ("/shell/", "", "/", Redirect),
        ("/home/", "", "/", Redirect),
        ("/core/", "", "/?open=app:core", Redirect),
        ("/core/", "view=architecture", "/?open=app:architecture", Redirect),
        ("/architecture/", "", "/?open=app:architecture", Redirect),
        ("/approvals/", "", "/?open=app:approvals", Redirect),
        ("/approvals/gb-f1aa6d1817eadde5", "", "/?open=app:approvals/gb-f1aa6d1817eadde5", Redirect),
        ("/conversations/", "", "/?open=app:conversations", Redirect),
        ("/conversations/85884916", "", "/?open=conv:85884916", Redirect),
        ("/search/", "q=cell+map", "/?open=app:search/cell+map", Redirect),
        ("/search/", "", "/?open=app:search", Redirect),
        ("/control-room/", "", "/?open=app:control-room", Redirect),
        ("/control-room/nodes", "", "/?open=app:control-room/nodes", Redirect),
        ("/doc/102afc34be08", "", "/?open=doc:102afc34be08", Redirect),
        ("/doc/102afc34be08", "v=12", "/?open=doc:102afc34be08@12", Redirect),
        ("/ocean/", "", "/?open=app:ocean", Redirect),
        ("/ocean/eng/notes", "", "/?open=app:ocean/eng/notes", Redirect),
        ("/s/sh-cac665c453e96111", "", "/?open=s:sh-cac665c453e96111", Redirect),
        ("/feed/", "", "/?open=app:feed", Alias),
        ("/foreman/", "", "/?open=app:foreman", Alias),
        ("/foreman/stats", "", "/?open=app:foreman/stats", Alias),
    ];
    for (p, q, want, m) in golden {
        assert_eq!(to(p, q), Some((want.to_string(), *m)), "{p}?{q}");
    }
}

#[test]
fn links_other_query_keys_are_carried_and_bad_ones_dropped() {
    assert_eq!(to("/doc/abc", "v=3&theme=light"), Some(("/?open=doc:abc@3&theme=light".into(), LinkMode::Redirect)));
    assert_eq!(to("/core/", "x=%3Cscript%3E&y=a\"b"), Some(("/?open=app:core&x=%3Cscript%3E".into(), LinkMode::Redirect)));
    assert_eq!(links::target("/x", &link("/shell/", "").unwrap(), ""), "/x/");
}

#[test]
fn links_api_unknown_and_bad_ids_are_none() {
    for (p, q) in [
        ("/foreman/api/state", ""),
        ("/api/cell/ai.session/List", ""),
        ("/voice/ws", ""),
        ("/nope/", ""),
        ("/", ""),
        ("/doc/a/b", ""),
        ("/doc/..", ""),
        ("/doc/x", "v=1a"),
        ("/s/%2F%2Fevil.example", ""),
        ("/conversations/a b", ""),
    ] {
        assert_eq!(link(p, q), None, "{p}?{q}");
    }
}

#[test]
fn links_every_tab_has_a_row() {
    for (sys, t) in CE_NET.tabs() {
        let l = link(t.route, "").unwrap_or_else(|| panic!("{sys}: tab {} has no link row", t.route));
        assert!(links::ROWS.iter().any(|(r, _)| *r == l.row));
    }
}

#[test]
fn links_ref_grammar() {
    for ok in ["app:foreman", "app:foreman/stats", "doc:7Qk2@12", "conv:85884916", "app:search/cell+map", "s:sh-1"] {
        assert!(ref_ok(ok), "{ok}");
    }
    for bad in ["", "app", "evil:x", "app:", "app:a/../b", "doc:x@", "doc:x@1a", "app:a b", "app:a//b", "doc:<x>"] {
        assert!(!ref_ok(bad), "{bad}");
    }
}
