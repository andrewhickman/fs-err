extern crate autocfg;

fn main() {
    let ac = autocfg::new();
    // Allows `#[cfg(rustc_1_63)]` and `#[cfg(rustc_1_89)]` to be used in code
    ac.emit_rustc_version(1, 63);
    ac.emit_rustc_version(1, 89);

    // Re-run if this file changes
    autocfg::rerun_path("build.rs");
}
