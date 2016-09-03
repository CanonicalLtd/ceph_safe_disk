extern crate serde_codegen;

use std::env;
use std::path::Path;

pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let pgmap_src = Path::new("lib/pgmap.rs.in");
    let pgmap_dst = Path::new(&out_dir).join("pgmap.rs");

    let osdmap_src = Path::new("lib/osdmap.rs.in");
    let osdmap_dst = Path::new(&out_dir).join("osdmap.rs");

    let diag_src = Path::new("lib/diag.rs.in");
    let diag_dest = Path::new(&out_dir).join("diag.rs");

    serde_codegen::expand(&pgmap_src, &pgmap_dst).unwrap();
    serde_codegen::expand(&osdmap_src, &osdmap_dst).unwrap();
    serde_codegen::expand(&diag_src, &diag_dest).unwrap();
}
