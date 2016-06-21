use osdmap::OsdMap;
use pgmap::PGMap;
use error::CSDError;
use from::*;

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct PoolMap {
    osd_map: OsdMap,
    pg_map: PGMap,
}

impl PoolMap {
    pub fn new() -> Result<PoolMap, CSDError> {
        let pool_map = PoolMap {
            osd_map: try!(OsdMap::from_ceph("osd dump")),
            pg_map: try!(PGMap::from_ceph("pg dump")),
        };
        Ok(pool_map)
    }

    pub fn map_pools(&self) {
        if let Some(pg_stats) = self.pg_map.pg_stats.as_ref() {
            let mut last_id: String = String::new();
            for pg_stat in pg_stats {
                let pgid = pg_stat.pgid.clone().unwrap();
                let split: Vec<&str> = pgid.split('.').collect();
                println!("{}", split.last().unwrap());
            }
        }
    }
}

#[test]
#[should_panic]
fn poolmap_new() {
    let poolmap = PoolMap {
        osd_map: OsdMap::from_file("test/osd_dump.json").unwrap(),
        pg_map: PGMap::from_file("test/pg_dump.json").unwrap(),
    };
    assert_eq!(poolmap.osd_map.pool_max.unwrap(), 1);
    assert_eq!(poolmap.pg_map.osd_stats_sum, None);
}
