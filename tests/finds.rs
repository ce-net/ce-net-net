//! The finds table of SEARCH-PAGE-2026-11-21 section 5.
use ce_net_net::finds::{ref_kind, KINDS};
use ce_net_net::*;

#[test]
fn every_finds_cell_is_placed() {
    for f in finds() {
        assert!(system_of(f.cell).is_some(), "{} answers {} but no system places it", f.cell, f.kind);
        assert!(CE_NET.places(f.cell, f.major as u32), "{}@{}", f.cell, f.major);
    }
}

#[test]
fn no_two_lines_equal_on_kind_and_cell() {
    let all = finds();
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert!(!(a.kind == b.kind && a.cell == b.cell), "{} {} twice", a.kind, a.cell);
        }
    }
}

#[test]
fn kinds_needs_and_deadlines_are_sane() {
    for f in finds() {
        assert!(KINDS.contains(&f.kind), "{}: not a search.plan kind", f.kind);
        assert!(f.needs.starts_with(&format!("{}:", f.cell)) && !f.needs.contains('*'), "{}: needs one exact right of {}", f.needs, f.cell);
        assert!(f.weight > 0.0 && f.weight <= 1.0, "{} {}", f.kind, f.cell);
        assert!((100..=4000).contains(&f.deadline_ms), "{} {}: deadline under the Find deadline of 4 s", f.kind, f.cell);
    }
}

/// A kind the page finds also opens: an opener for its ref kind, whose `needs` the source's own right implies (the
/// same cell), or a kind whose opener is the existing record and text views.
#[test]
fn every_found_kind_has_an_opener() {
    let all = opens();
    for f in finds().iter().filter(|f| f.kind != "doc" && f.kind != "capability") {
        let rk = ref_kind(f.kind);
        assert!(all.iter().any(|o| o.pattern == rk), "{} rows ({rk}:) have no opener", f.kind);
        // a kind that is new on this page opens in the view of the cell that answered it; `cell:` keeps site.cellmap's
        if f.kind != "cell" {
            assert!(all.iter().any(|x| x.pattern == rk && x.cell == f.cell), "{rk}: no opener reads {}", f.cell);
        }
    }
}

#[test]
fn the_search_cells_are_in_search() {
    assert_eq!(system_of("search.plan"), Some("SEARCH"));
    assert_eq!(system_of("search.typed"), Some("SEARCH"));
    let slot = HOME_SLOTS.iter().find(|s| s.name == "search").expect("a search slot on Home");
    assert_eq!((slot.cell, slot.view, slot.area, slot.order, slot.needs), ("search.typed", "search.typed::Box", Area::Head, 1, "search.typed:find"));
}
