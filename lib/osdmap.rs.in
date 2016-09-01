#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct OsdMap {
    pub pool_max: i32,
    pub max_osd: i32,
    pub created: String,
    pub modified: String,
    pub osd_xinfo: Vec<OsdXinfo>,
    pub osds: Vec<Osds>,
    pub epoch: i32,
    pub flags: String,
    pub cluster_snapshot: String,
    pub pools: Vec<Pools>,
    pub fsid: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Default {
    pub k: String,
    pub technique: String,
    pub m: String,
    pub plugin: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Osds {
    pub heartbeat_back_addr: String,
    pub uuid: String,
    pub weight: f32,
    pub up_from: i32,
    pub heartbeat_front_addr: String,
    pub down_at: i32,
    pub up: i32,
    pub lost_at: i32,
    pub primary_affinity: f32,
    pub state: Vec<String>,
    pub last_clean_begin: i32,
    pub last_clean_end: i32,
    pub public_addr: String,
    pub up_thru: i32,
    pub cluster_addr: String,
    pub osd: i32,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct OsdXinfo {
    pub laggy_probability: f32,
    pub laggy_interval: i32,
    pub features: u64,
    pub old_weight: Option<i32>,
    pub down_stamp: String,
    pub osd: i32,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Pools {
    pub cache_target_full_ratio_micro: i32,
    pub fast_read: Option<bool>,
    pub stripe_width: i32,
    pub flags_names: String,
    pub tier_of: i32,
    pub hit_set_grade_decay_rate: Option<i32>,
    pub pg_placement_num: i32,
    pub use_gmt_hitset: Option<bool>,
    pub quota_max_bytes: i32,
    pub erasure_code_profile: String,
    pub expected_num_objects: Option<i32>,
    // "replicated size"
    pub size: i32,
    pub snap_seq: i32,
    pub auid: i32,
    pub cache_min_flush_age: i32,
    pub hit_set_period: i32,
    pub min_read_recency_for_promote: i32,
    pub target_max_objects: i32,
    pub pg_num: i32,
    pub crush_ruleset: i32,
    pub pool_name: String,
    pub cache_min_evict_age: i32,
    pub snap_mode: String,
    pub cache_mode: String,
    pub min_size: i32,
    pub cache_target_dirty_high_ratio_micro: Option<i32>,
    pub crash_replay_interval: i32,
    pub object_hash: i32,
    pub write_tier: i32,
    pub cache_target_dirty_ratio_micro: i32,
    pub pool: i32,
    pub removed_snaps: String,
    pub last_force_op_resend: String,
    pub quota_max_objects: i32,
    pub hit_set_count: i32,
    pub flags: i32,
    pub target_max_bytes: i32,
    pub snap_epoch: i32,
    pub hit_set_search_last_n: Option<i32>,
    pub last_change: String,
    pub min_write_recency_for_promote: Option<i32>,
    pub read_tier: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use from::FromFile;

    // Jewel tests
    #[test]
    #[should_panic]
    fn osdmap_from_jewel_file_panic() {
        let osdmap = OsdMap::from_file("test/jewel/osd_dump_safe.json").unwrap();
        // If this is safe to remove then osd's len should be min_size + 1
        assert_eq!(osdmap.osds.len(), 0);
    }

    #[test]
    fn osdmap_from_jewel_file() {
        let osdmap = OsdMap::from_file("test/jewel/osd_dump_safe.json").unwrap();
        // Default config for min_size is 2
        assert_eq!(osdmap.pools.first().unwrap().min_size, 2);
    }

    #[test]
    #[should_panic]
    fn osdmap_from_jewel_file_no_osd_panic() {
        let osdmap = OsdMap::from_file("test/jewel/osd_dump_no_osd.json").unwrap();
        // This has no osds so len should be 0
        assert_eq!(osdmap.osds.len(), 3);
    }

    #[test]
    fn osdmap_from_jewel_file_no_osd() {
        let osdmap = OsdMap::from_file("test/jewel/osd_dump_no_osd.json").unwrap();
        let mut osds_up: Vec<i32> = Vec::new();
        // There should be no OSDs present
        for osd in osdmap.osds {
            if osd.up == 1 {
                osds_up.push(1);
            }
        }
        assert_eq!(osds_up.len(), 0);
    }

    #[test]
    fn osdmap_from_jewel_file_non_safe() {
        let osdmap = OsdMap::from_file("test/jewel/osd_dump_non_safe.json").unwrap();
        let mut osds_up: Vec<i32> = Vec::new();
        // Since this is non-safe there should be min_size or less OSDs present
        for osd in osdmap.osds {
            if osd.up == 1 {
                osds_up.push(1);
            }
        }
        assert_eq!(osds_up.len() as i32, osdmap.pools.first().unwrap().min_size);
    }

    // Firefly tests
    #[test]
    fn osdmap_from_firefly_file() {
        let osdmap = OsdMap::from_file("test/firefly/osd_dump_safe.json").unwrap();
        assert_eq!(osdmap.pools.first().unwrap().min_size, 2);
    }

    #[test]
    #[should_panic]
    fn osdmap_from_firefly_file_panic() {
        let osdmap = OsdMap::from_file("test/firefly/osd_dump_safe.json").unwrap();
        // If this is safe to remove then osd's len should be min_size + 1
        assert_eq!(osdmap.osds.len() as i32, 0);
    }

    #[test]
    #[should_panic]
    fn osdmap_from_ceph_panic() {
        use from::FromCeph;
        let osdmap = OsdMap::from_ceph("osd dump");
        assert_eq!(osdmap.is_ok(), true);
    }
}
