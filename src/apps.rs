//! Every app and system a person opens on apps.ce-net.com, as places of the one entrypoint (repo:ce
//! docs/UI-AS-CELLS-2026-11-21.md section 9). Leif 2026-11-21: "where are all of my other systems and apps and things
//! we have had previously? everything must be cells and backwards compatible. all capabilities".
//!
//! The rows are the gate's own routes (repo:ce-fleet door/gate.py on the deployed snapshot 55013253: `PORT_PATHS`
//! less `NOT_APP_ROUTES`, `APP_PATHS` (ce-serve apps), `GZ_APPS`, the work paths of `WORK_PATHS`) plus the hosts and
//! old pages a person reached them by (the launcher at `/?old`, the ce shell at `/shell/`, maps.ce-net.com). A tab of
//! `CE_NET` (Foreman, Feed, Search, ...) is not repeated here: it is a place by its own `needs`.
//!
//! `href` is the old page, unchanged: the gate still serves it, so every old URL keeps working, and a place whose cell
//! does not yet render in the entry opens it as a link out. Who sees a place is the gate's rule, applied by site.serve
//! from the gate's people and share files (server side): the owner every place, a person the `apps:<seg>` grants they
//! hold (`apps:*` and `*` cover), an admin every place but the owner's own, a share row `app:/<seg>/`. `owner`: one of
//! the gate's `LEIF_SEGS` (or Leif-only in its cell), which no grant opens.

/// One place: the gate path segment it lives under (the ref is `app:<seg>`), its name, its old page, and whether it is
/// the owner's alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct App {
    pub seg: &'static str,
    pub title: &'static str,
    pub href: &'static str,
    pub owner: bool,
}

const fn a(seg: &'static str, title: &'static str, href: &'static str) -> App {
    App { seg, title, href, owner: false }
}

const fn o(seg: &'static str, title: &'static str, href: &'static str) -> App {
    App { seg, title, href, owner: true }
}

/// The places beside the tabs, in the order the entry lists them: the owner's own work first, then the apps by kind.
pub const APPS: &[App] = &[
    // the owner's own (gate LEIF_SEGS and Leif-only cells)
    o("shell", "ce shell", "/shell/"),
    o("launcher", "Launcher", "/?old"),
    o("voice", "Voice", "/voice/"),
    o("brief", "Brief", "/brief/"),
    o("attn", "Attention", "/attn/"),
    o("devices", "Devices", "/devices/"),
    o("one", "One", "/one/"),
    o("leif", "Needs Leif", "/leif/"),
    o("runners", "Runners", "/runners/"),
    o("delegate", "Delegate", "/delegate/"),
    // work and operations
    a("me", "Me", "/me/"),
    // devenv is the Code tab now (DEVENV in lib.rs), not a place
    a("secrets", "Secrets", "/secrets/"),
    a("grant", "Grant", "/grant/"),
    a("fleet", "Fleet", "/fleet/"),
    a("console", "Console", "/console/"),
    a("lanes", "Lanes", "/lanes/"),
    a("schedule", "Schedule", "/schedule/"),
    a("projects", "Projects", "/projects/"),
    a("room", "Rooms", "/room/"),
    a("trana", "Trana", "/trana/"),
    a("launchpad", "Launchpad", "/launchpad/"),
    a("troublemaker", "Troublemaker", "/troublemaker/"),
    a("books", "Books", "/books/"),
    a("business", "Business", "/business/"),
    a("sourcing", "Sourcing", "/sourcing/"),
    a("vote", "Festival vote", "/vote/"),
    // documents and media
    a("doc", "Documents", "/doc/"),
    a("ocean", "Ocean", "/ocean/"),
    a("docs", "Docs", "/docs/"),
    a("canvas", "Canvas", "/canvas/"),
    a("photos", "Photos", "/photos/"),
    a("media", "Media", "/media/"),
    a("ai", "AI", "/ai/"),
    a("ui-examples", "UI examples", "/ui-examples/"),
    // twins and maps
    a("twinton", "Twinton", "/twinton/"),
    a("maps", "Maps", "https://maps.ce-net.com/"),
    a("globe", "Globe", "/globe/"),
    a("twin-ops", "Twin ops", "/twin-ops/"),
    a("port-twin", "Port twin", "/port-twin/"),
    a("yuncheng-twin", "Yuncheng twin", "/yuncheng-twin/"),
    a("tree", "Cell tree", "/tree/"),
    // making things
    a("parts", "Parts", "/parts/"),
    a("cad", "CAD", "/cad/"),
    a("print-farm", "Print farm", "/print-farm/"),
    a("print-shop", "Print shop", "/print-shop/"),
    a("pce", "PCE", "/pce/"),
    a("assembly-guide", "Assembly guide", "/assembly-guide/"),
    a("disassembly", "Disassembly", "/disassembly/"),
    a("box-lab", "Box lab", "/box-lab/"),
    a("factory-voice", "Factory voice", "/factory-voice/"),
    a("gev", "GEV", "/gev/"),
    a("ledframe", "Ledframe", "/ledframe/"),
    a("ledframe-v3", "Ledframe v3", "/ledframe-v3/"),
    a("tandem", "Tandem", "/tandem/"),
    // robots
    a("microduck", "Microduck", "/microduck/"),
    a("huey", "Huey", "/huey/"),
    a("huey-sound", "Huey sound", "/huey-sound/"),
    // Neon's pages (Gemini 4 Neon on macbook, repo:ce/docs/NEON-INTEGRATION-2026-11-21.md): served by its http.expose
    // on the Mac's tailnet address, not by the gate, so they are the owner's alone and open as a link out (only a
    // device on the tailnet reaches them). Their figures are computed in the browser (demo data) until each page reads
    // its cell through http.expose; the cells behind them answer HealthOp from ops-1 (repo:ce tools/neon-cells).
    o("mesh-hub", "Mesh hub (Neon)", "http://100.112.251.83:8795/mesh-hub/"),
    o("edu-anim3d", "3D lessons (Neon)", "http://100.112.251.83:8795/edu-anim3d/"),
    o("mfg-twin3d", "Factory twin (Neon)", "http://100.112.251.83:8795/mfg-twin3d/"),
    o("venture-forge", "Venture forge (Neon)", "http://100.112.251.83:8795/venture-forge/"),
    o("critic-loop", "Critic loop (Neon)", "http://100.112.251.83:8795/critic-loop/"),
];

/// The door of Neon's http.expose on macbook: the one origin outside the gate a place may link to, owner-only.
pub const NEON_PAGES: &str = "http://100.112.251.83:8795/";

/// The place for `seg`, if it is one.
pub fn app(seg: &str) -> Option<&'static App> {
    APPS.iter().find(|a| a.seg == seg)
}
