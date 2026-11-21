//! The one entrypoint's link table (repo:ce/docs/UI-AS-CELLS-2026-11-21.md 9.4): every old address of apps.ce-net.com
//! and the ce-ref it opens inside the entry document (`/?open=<ref>`). Data, not code: `link` is pure, and which rows
//! are switched on (`off`, redirect, alias) is held per row by site.serve, never here.
//!
//! A target is built only from a validated ref (`ref_ok`), so no row can redirect off the site. POST, streams and API
//! paths are not in the table (`/foreman/api/*`, `/voice/*`, `/api/*`, any `/api/` segment): they stay with their
//! old backend until their callers have moved.

/// How an old address answers once its row is switched on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkMode {
    /// 302 to `/?open=<ref>` (never 301: a cached permanent redirect cannot be rolled back).
    Redirect,
    /// The entry document served at the old path with the ref implied (installed apps keep their scope).
    Alias,
}

/// One old address resolved: the row it belongs to (the switch key), the ref it opens (`None`: the entry document
/// itself), and how it answers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    pub row: &'static str,
    pub open: Option<String>,
    pub mode: LinkMode,
}

/// The rows, as (row, mode). Every tab route of `CE_NET` has a row (test `links_every_tab_has_a_row`).
pub const ROWS: &[(&str, LinkMode)] = &[
    ("shell", LinkMode::Redirect),
    ("home", LinkMode::Redirect),
    ("core", LinkMode::Redirect),
    ("architecture", LinkMode::Redirect),
    ("approvals", LinkMode::Redirect),
    ("conversations", LinkMode::Redirect),
    ("search", LinkMode::Redirect),
    ("control-room", LinkMode::Redirect),
    ("doc", LinkMode::Redirect),
    ("ocean", LinkMode::Redirect),
    ("s", LinkMode::Redirect),
    ("feed", LinkMode::Alias),
    ("now", LinkMode::Redirect),
    ("business-monitor", LinkMode::Redirect),
    ("foreman", LinkMode::Alias),
];

/// The ref types the entry document opens.
pub const REF_TYPES: &[&str] = &["app", "doc", "conv", "s", "file", "person", "cell"];

fn id_ok(s: &str) -> bool {
    (1..=64).contains(&s.len()) && s.bytes().any(|c| c.is_ascii_alphanumeric()) && s.bytes().all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
}

/// The short ref grammar `type:id[/sub][@pin]`: a known type, an id of `[A-Za-z0-9._-]` (1 to 64), an optional sub of
/// segments of the same set (or `%` escapes, at most 128 bytes), an optional pin of digits.
pub fn ref_ok(r: &str) -> bool {
    let Some((ty, rest)) = r.split_once(':') else { return false };
    if !REF_TYPES.contains(&ty) || r.len() > 220 {
        return false;
    }
    let (rest, pin) = match rest.split_once('@') {
        Some((a, p)) => (a, Some(p)),
        None => (rest, None),
    };
    if let Some(p) = pin {
        if !(1..=10).contains(&p.len()) || !p.bytes().all(|c| c.is_ascii_digit()) {
            return false;
        }
    }
    let (id, sub) = match rest.split_once('/') {
        Some((a, s)) => (a, Some(s)),
        None => (rest, None),
    };
    id_ok(id)
        && sub.map_or(true, |s| {
            (1..=128).contains(&s.len())
                && s.split('/').all(|seg| !seg.is_empty() && seg != "." && seg != ".." && seg.bytes().all(|c| c.is_ascii_alphanumeric() || b"._-%+".contains(&c)))
        })
}

/// The slug of a tab route: `/approvals/` -> `approvals`.
pub fn slug_of(route: &str) -> &str {
    route.trim_matches('/')
}

fn q<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|kv| kv.split_once('=').filter(|(k, _)| *k == key).map(|(_, v)| v))
}

fn mode(row: &str) -> LinkMode {
    ROWS.iter().find(|(r, _)| *r == row).map(|(_, m)| *m).unwrap_or(LinkMode::Redirect)
}

/// An old document address (`path` without the query, `query` without `?`) to its row and ref, or `None` when it is
/// not an old page (unknown path, an API path, an invalid id).
pub fn link(path: &str, query: &str) -> Option<Link> {
    let segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let first = *segs.first()?;
    if segs.iter().any(|s| *s == "api") || first == "voice" {
        return None;
    }
    let row = ROWS.iter().find(|(r, _)| *r == first)?.0;
    let rest = &segs[1..];
    let mk = |open: Option<String>| -> Option<Link> {
        if let Some(r) = &open {
            if !ref_ok(r) {
                return None;
            }
        }
        Some(Link { row, open, mode: mode(row) })
    };
    let sub = |base: &str| -> Option<String> {
        if rest.is_empty() {
            Some(base.to_string())
        } else {
            Some(format!("{base}/{}", rest.join("/")))
        }
    };
    match (row, rest.len()) {
        ("shell", 0) => mk(None),
        // S2 (11.7): the Home row opens the Home place, so a switched `/home/` lands on a script-free entry.
        ("home", 0) => mk(Some("app:home".into())),
        ("core", 0) if q(query, "view") == Some("architecture") => mk(Some("app:architecture".into())),
        ("core" | "architecture" | "search", 0) => match (row, q(query, "q")) {
            ("search", Some(t)) if !t.is_empty() => mk(Some(format!("app:search/{t}"))),
            _ => mk(Some(format!("app:{row}"))),
        },
        ("conversations", 0) => mk(Some("app:conversations".into())),
        ("conversations", 1) => mk(Some(format!("conv:{}", rest[0]))),
        ("approvals" | "control-room" | "ocean" | "foreman" | "feed", _) => mk(sub(&format!("app:{row}"))),
        ("now" | "business-monitor", 0) => mk(Some(format!("app:{row}"))),
        ("doc", 1) => match q(query, "v") {
            Some(v) => mk(Some(format!("doc:{}@{v}", rest[0]))),
            None => mk(Some(format!("doc:{}", rest[0]))),
        },
        ("s", 1) => mk(Some(format!("s:{}", rest[0]))),
        _ => None,
    }
}

/// Where a link lands inside the entry document under `base` (`/x` before the switch, empty after it): `<base>/` or
/// `<base>/?open=<ref>`, plus the old query keys the row did not consume (`view`, `q`, `v` are consumed).
pub fn target(base: &str, l: &Link, query: &str) -> String {
    let keep: Vec<&str> = query
        .split('&')
        .filter(|kv| !kv.is_empty())
        .filter(|kv| {
            let k = kv.split('=').next().unwrap_or("");
            !["view", "q", "v", "open"].contains(&k)
        })
        .filter(|kv| kv.len() <= 64 && kv.bytes().all(|c| c.is_ascii_alphanumeric() || b"=._-%+".contains(&c)))
        .collect();
    let mut parts: Vec<String> = vec![];
    if let Some(r) = &l.open {
        parts.push(format!("open={r}"));
    }
    parts.extend(keep.iter().map(|s| s.to_string()));
    if parts.is_empty() {
        format!("{base}/")
    } else {
        format!("{base}/?{}", parts.join("&"))
    }
}

/// The place a session's own path stands for (repo:ce/docs/UI-AS-CELLS-2026-11-21.md 11.5), as the ref of a `Shared`
/// row: `/<seg>/` of `APPS` or of a tab is `app:<seg>`, `/<seg>/<rest>` is `app:<seg>/<rest>`, `/doc/<id>` is
/// `doc:<id>`; a segment in neither table, an API path or a ref the grammar refuses is `None`. Pure; the path is the
/// decoded path the gate signed (a query is cut off). A path never adds an ability: the row is drawn as a link out.
pub fn place_of_path(path: &str) -> Option<String> {
    let path = path.split(['?', '#']).next().unwrap_or("");
    if !path.starts_with('/') {
        return None;
    }
    let segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let first = *segs.first()?;
    if segs.iter().any(|s| *s == "api") {
        return None;
    }
    let r = match (first, &segs[1..]) {
        ("doc", [id]) => format!("doc:{id}"),
        (seg, rest) => {
            let known = crate::apps::app(seg).is_some() || crate::CE_NET.tabs().iter().any(|(_, t)| slug_of(t.route) == seg);
            if !known {
                return None;
            }
            if rest.is_empty() {
                format!("app:{seg}")
            } else {
                format!("app:{seg}/{}", rest.join("/"))
            }
        }
    };
    ref_ok(&r).then_some(r)
}
