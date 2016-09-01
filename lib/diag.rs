extern crate ansi_term;
extern crate json;

use ansi_term::Colour;

use error::CSDError;
use osdmap::OsdMap;
use pgstate::*;
use pgmap::*;
use from::*;

use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::fmt;

// The removability status of an OSD. Using an enum for precedence:
// Safe < Unknown < NonSafe
#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum Status {
    Safe,
    Unknown,
    NonSafe,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Status::Unknown => write!(f, "Pending"),
            &Status::Safe => write!(f, "Removable"),
            &Status::NonSafe => write!(f, "Not removable"),
        }
    }
}

// Holds information about a PG's status, it's ID and state
#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct PgInfo {
    pg_id: String,
    pg_state: String,
    rm_safety: RmSafety,
}

impl PgInfo {
    fn new(states: &String, pgid: String) -> PgInfo {
        PgInfo {
            pg_id: pgid,
            pg_state: states.clone(),
            rm_safety: RmSafety::new(states),
        }
    }
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
    pub fn quick_diag(self, json_format: bool) -> bool {
        for stat in self.pg_map.pg_stats {
            for pool in self.osd_map.pools.iter() {
                if (stat.up.clone().len() as i32) >= (pool.min_size + 1) {
                    if json_format {
                        println!("{{\"Safe to remove an OSD\": true}}");
                    } else {
                        println!("{} Safe to remove an OSD", Colour::Green.paint("●"));
                    }
                    return true;
                } else {
                    if json_format {
                        println!("{{\"Safe to remove an OSD\": false}}");
                    } else {
                        println!("{} Not safe to remove an OSD", Colour::Red.paint("●"));
                    }
                    return false;
                }
            }
        }
        return false;
    }

    // Maps out PGs and their states to each OSD in their `acting` list.
    // Returns a more general `Status` based on whether there is a removable
    // OSD or not.
    // `osd_diag` holds an OSD's removability status. Using a binary heap we
    // can always know which state it has that holds the highest precedent.
    pub fn exhaustive_diag(self, json_format: bool) -> Status {
        let mut pg_info: Vec<(i32, PgInfo)> = Vec::new();
        let mut osd_diag: BTreeMap<i32, BinaryHeap<Status>> = BTreeMap::new();
        let mut general_status = Status::Safe;

        // Populate PG statuses. For each PG we push it's list of acting OSDs
        // and the state of the PG
        for pg_stat in self.pg_map.pg_stats {
            for acting in pg_stat.acting {
                pg_info.push((acting, PgInfo::new(&pg_stat.state, pg_stat.pgid.clone())));
            }
        }

        // Generate OSD removability.
        for pg in &pg_info {
            if let None = osd_diag.get_mut(&pg.0) {
                osd_diag.insert(pg.0, BinaryHeap::new());
            } else if let Some(osd_diag) = osd_diag.get_mut(&pg.0) {
                match pg.1.rm_safety {
                    RmSafety::None => osd_diag.push(Status::NonSafe),
                    RmSafety::Pending => osd_diag.push(Status::Unknown),
                    RmSafety::Total => osd_diag.push(Status::Safe),
                };
            }
        }
        if json_format {
            let mut output = json::JsonValue::new_object();
            // Put OSD id's into a list based on their safety status
            output[format!("{}", Status::NonSafe)] = json::JsonValue::new_array();
            output[format!("{}", Status::Safe)] = json::JsonValue::new_array();
            output[format!("{}", Status::Unknown)] = json::JsonValue::new_array();

            for (osd, status) in osd_diag {
                if let Some(osd_status) = status.peek() {
                    match osd_status {
                        &Status::NonSafe => {
                            // output[format!("{}", osd)] = format!("{:?}", osd_status).into();
                            output[format!("{}", Status::NonSafe)].push(osd);
                            general_status = osd_status.clone();
                        }
                        &Status::Safe => {
                            // output[format!("{}", osd)] = format!("{:?}", osd_status).into();
                            output[format!("{}", Status::Safe)].push(osd);
                        }
                        &Status::Unknown => {
                            // output[format!("{}", osd)] = format!("{:?}", osd_status).into();
                            output[format!("{}", Status::Unknown)].push(osd);
                            general_status = osd_status.clone();
                        }
                    };
                }
            }
            println!("{}", output.dump());
        } else {
            // Print the statuses of OSDs
            println!("Current OSD statuses:");
            for (osd, status) in osd_diag {
                if let Some(osd_status) = status.peek() {
                    match osd_status {
                        &Status::NonSafe => {
                            println!("{} {}: {}", Colour::Red.paint("●"), osd, osd_status);
                            general_status = osd_status.clone();
                        }
                        &Status::Safe => {
                            println!("{} {}: {}", Colour::Green.paint("●"), osd, osd_status)
                        }
                        &Status::Unknown => {
                            println!("{} {}: {}", Colour::Yellow.paint("●"), osd, osd_status);
                            general_status = osd_status.clone();
                        }
                    };
                }
            }
        }
        return general_status;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use osdmap::OsdMap;
    use pgmap::*;
    use from::*;

    #[test]
    fn quick_diag_jewel_safe() {
        let status = DiagMap {
                pg_map: PGMap::from_file("test/jewel/pg_dump_safe.json").unwrap(),
                osd_map: OsdMap::from_file("test/jewel/osd_dump_safe.json").unwrap(),
            }
            .quick_diag(false);

        assert_eq!(status, true);
    }

    #[test]
    fn exhaustive_diag_jewel_safe() {
        let status: Status = DiagMap {
                pg_map: PGMap::from_file("test/jewel/pg_dump_safe.json").unwrap(),
                osd_map: OsdMap::from_file("test/jewel/osd_dump_safe.json").unwrap(),
            }
            .exhaustive_diag(false);

        assert_eq!(status, Status::Safe);
    }

    #[test]
    fn exhaustive_diag_jewel_non_safe() {
        let status: Status = DiagMap {
                pg_map: PGMap::from_file("test/jewel/pg_dump_non_safe.json").unwrap(),
                osd_map: OsdMap::from_file("test/jewel/osd_dump_non_safe.json").unwrap(),
            }
            .exhaustive_diag(false);

        assert_eq!(status, Status::NonSafe);
    }

    #[test]
    fn exhaustive_diag_jewel_pending() {
        let status: Status = DiagMap {
                pg_map: PGMap::from_file("test/jewel/pg_dump_pending.json").unwrap(),
                osd_map: OsdMap::from_file("test/jewel/osd_dump_pending.json").unwrap(),
            }
            .exhaustive_diag(false);

        assert_eq!(status, Status::Unknown);
    }

    #[test]
    fn quick_diag_firefly_safe() {
        let status = DiagMap {
                pg_map: PGMap::from_file("test/firefly/pg_dump_safe.json").unwrap(),
                osd_map: OsdMap::from_file("test/firefly/osd_dump_safe.json").unwrap(),
            }
            .quick_diag(false);

        assert_eq!(status, true);
    }

    #[test]
    fn exhaustive_diag_firefly_safe() {
        let status: Status = DiagMap {
                pg_map: PGMap::from_file("test/firefly/pg_dump_safe.json").unwrap(),
                osd_map: OsdMap::from_file("test/firefly/osd_dump_safe.json").unwrap(),
            }
            .exhaustive_diag(false);

        assert_eq!(status, Status::Safe);
    }

}
