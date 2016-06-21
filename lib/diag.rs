extern crate ansi_term;

use ansi_term::Colour;

use error::CSDError;
use osdmap::OsdMap;
use pgmap::PGMap;
use from::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    Unknown,
    Safe,
    NonSafe,
}

#[derive(Debug, Clone)]
pub struct DiagMap {
    pg_map: PGMap,
    osd_map: OsdMap,
}

impl DiagMap {
    pub fn new() -> Result<DiagMap, CSDError> {
        Ok(DiagMap {
            pg_map: try!(PGMap::from_ceph("pg dump")),
            osd_map: try!(OsdMap::from_ceph("osd dump")),
        })
    }

    // Quick check to see if `min_size +1` is satisfied
    pub fn quick_diag(self) -> Status {
        if let Some(pg_stats) = self.pg_map.pg_stats {
            if let Some(pools) = self.osd_map.pools {
                for stat in pg_stats.iter() {
                    for pool in pools.iter() {
                        if (stat.up.clone().unwrap_or(vec![]).len() as i32) >=
                            (pool.min_size.unwrap_or(2) + 1) {
                            println!("{} Safe to remove an OSD", Colour::Green.paint("●"));
                            return Status::Safe;
                        } else {
                            println!("{} Not safe to remove an OSD", Colour::Red.paint("●"));
                            return Status::NonSafe;
                        }
                    }
                }
            }
        }
        // If we've reached this point something is **wrong**
        Status::Unknown
    }

}


// JSON output in `test/` should show safe to remove
#[test]
fn test_quick_diag_jewel_safe() {
    let status: Status = DiagMap {
        pg_map: PGMap::from_file("test/pg_dump_jewel.json").unwrap(),
        osd_map: OsdMap::from_file("test/osd_dump_jewel.json").unwrap(),
    }.quick_diag();

    assert_eq!(status, Status::Safe);
}
