#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct OsdMap {
    pub pool_max: Option<i32>,
    pub max_osd: Option<i32>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub osd_xinfo: Option<Vec<OsdXinfo>>,
    pub osds: Option<Vec<Osds>>,
    pub epoch: Option<i32>,
    pub flags: Option<String>,
    pub cluster_snapshot: Option<String>,
    pub pools: Option<Vec<Pools>>,
    pub fsid: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct Default {
    pub k: Option<String>,
    pub technique: Option<String>,
    pub m: Option<String>,
    pub plugin: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct Osds {
    pub heartbeat_back_addr: Option<String>,
    pub uuid: Option<String>,
    pub weight: Option<f32>,
    pub up_from: Option<i32>,
    pub heartbeat_front_addr: Option<String>,
    pub down_at: Option<i32>,
    pub up: Option<i32>,
    pub lost_at: Option<i32>,
    pub primary_affinity: Option<f32>,
    pub state: Option<Vec<String>>,
    pub last_clean_begin: Option<i32>,
    pub last_clean_end: Option<i32>,
    pub public_addr: Option<String>,
    pub up_thru: Option<i32>,
    pub cluster_addr: Option<String>,
    pub osd: Option<i32>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct OsdXinfo {
    pub laggy_probability: Option<f32>,
    pub laggy_interval: Option<i32>,
    pub features: Option<u64>,
    pub old_weight: Option<i32>,
    pub down_stamp: Option<String>,
    pub osd: Option<i32>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct Pools {
    pub cache_target_full_ratio_micro: Option<i32>,
    pub fast_read: Option<bool>,
    pub stripe_width: Option<i32>,
    pub flags_names: Option<String>,
    pub tier_of: Option<i32>,
    pub hit_set_grade_decay_rate: Option<i32>,
    pub pg_placement_num: Option<i32>,
    pub use_gmt_hitset: Option<bool>,
    pub quota_max_bytes: Option<i32>,
    pub erasure_code_profile: Option<String>,
    pub expected_num_objects: Option<i32>,
    // "replicated size"
    pub size: Option<i32>,
    pub snap_seq: Option<i32>,
    pub auid: Option<i32>,
    pub cache_min_flush_age: Option<i32>,
    pub hit_set_period: Option<i32>,
    pub min_read_recency_for_promote: Option<i32>,
    pub target_max_objects: Option<i32>,
    pub pg_num: Option<i32>,
    pub crush_ruleset: Option<i32>,
    pub pool_name: Option<String>,
    pub cache_min_evict_age: Option<i32>,
    pub snap_mode: Option<String>,
    pub cache_mode: Option<String>,
    pub min_size: Option<i32>,
    pub cache_target_dirty_high_ratio_micro: Option<i32>,
    pub crash_replay_interval: Option<i32>,
    pub object_hash: Option<i32>,
    pub write_tier: Option<i32>,
    pub cache_target_dirty_ratio_micro: Option<i32>,
    pub pool: Option<i32>,
    pub removed_snaps: Option<String>,
    pub last_force_op_resend: Option<String>,
    pub quota_max_objects: Option<i32>,
    pub hit_set_count: Option<i32>,
    pub flags: Option<i32>,
    pub target_max_bytes: Option<i32>,
    pub snap_epoch: Option<i32>,
    pub hit_set_search_last_n: Option<i32>,
    pub last_change: Option<String>,
    pub min_write_recency_for_promote: Option<i32>,
    pub read_tier: Option<i32>,
}

#[test]
#[should_panic]
fn osdmap_from_file() {
    use from::FromFile;
    let osdmap = OsdMap::from_file("test/osd_dump.json").unwrap();
    assert_eq!(osdmap.pool_max.unwrap(), 1);
    assert_eq!(osdmap.pool_max, None);
}

#[test]
#[should_panic]
fn osdmap_from_ceph() {
    use from::FromCeph;
    let osdmap = OsdMap::from_ceph("osd dump");
    assert_eq!(osdmap.is_ok(), true);
}
