// See `src/mon/PGMap.h` in ceph's source
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct PGMap {
    pub osd_stats_sum: Option<OsdStatsSum>,
    pub pg_stats_delta: Option<PgStatsDelta>,
    pub min_last_epoch_clean: Option<i32>,
    pub stamp: Option<String>,
    pub pg_stats_sum: Option<PgStatsSum>,
    pub last_pg_scan: Option<i32>,
    pub full_ratio: Option<f32>,
    pub pool_stats: Option<Vec<PoolStats>>,
    pub version: Option<i32>,
    pub last_osdmap_epoch: Option<i32>,
    pub near_full_ratio: Option<f32>,
    pub osd_stats: Option<Vec<OsdStats>>,
    pub pg_stats: Option<Vec<PgStats>>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct OsdStats {
    pub snap_trim_queue_len: Option<i32>,
    pub kb: Option<i32>,
    pub fs_perf_stat: Option<FsPerfStat>,
    pub hb_in: Option<Vec<i32>>,
    pub num_snap_trimming: Option<i32>,
    pub hb_out: Option<Vec<i32>>,
    pub kb_avail: Option<i32>,
    pub kb_used: Option<i32>,
    pub op_queue_age_hist: Option<OpQueueAgeHist>,
    pub osd: Option<i32>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct PgStatsDelta {
    pub acting: Option<i32>,
    pub log_size: Option<i32>,
    pub ondisk_log_size: Option<i32>,
    pub stat_sum: Option<StatSum>,
    pub up: Option<i32>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct StatSum {
    pub num_evict: Option<i32>,
    pub num_evict_kb: Option<i32>,
    pub num_bytes_hit_set_archive: Option<i32>,
    pub num_whiteouts: Option<i32>,
    pub num_objects_pinned: Option<i32>,
    pub num_scrub_errors: Option<i32>,
    pub num_evict_mode_full: Option<i32>,
    pub num_read: Option<i32>,
    pub num_objects_recovered: Option<i32>,
    pub num_objects_omap: Option<i32>,
    pub num_objects_missing_on_primary: Option<i32>,
    pub num_write: Option<i32>,
    pub num_object_clones: Option<i32>,
    pub num_objects: Option<i32>,
    pub num_deep_scrub_errors: Option<i32>,
    pub num_shallow_scrub_errors: Option<i32>,
    pub num_read_kb: Option<i32>,
    pub num_objects_missing: Option<i32>,
    pub num_flush_kb: Option<i32>,
    pub num_flush_mode_high: Option<i32>,
    pub num_write_kb: Option<i32>,
    pub num_evict_mode_some: Option<i32>,
    pub num_objects_degraded: Option<i32>,
    pub num_flush: Option<i32>,
    pub num_objects_misplaced: Option<i32>,
    pub num_bytes_recovered: Option<i32>,
    pub num_objects_hit_set_archive: Option<i32>,
    pub num_keys_recovered: Option<i32>,
    pub num_flush_mode_low: Option<i32>,
    pub num_objects_unfound: Option<i32>,
    pub num_promote: Option<i32>,
    pub num_object_copies: Option<i32>,
    pub num_bytes: Option<i32>,
    pub num_objects_dirty: Option<i32>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct PgStatsSum {
    pub acting: Option<i32>,
    pub log_size: Option<i32>,
    pub ondisk_log_size: Option<i32>,
    pub stat_sum: Option<StatSum>,
    pub up: Option<i32>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct OsdStatsSum {
    pub snap_trim_queue_len: Option<i32>,
    pub kb: Option<i32>,
    pub fs_perf_stat: Option<FsPerfStat>,
    pub hb_in: Option<Vec<i32>>,
    pub num_snap_trimming: Option<i32>,
    pub hb_out: Option<Vec<i32>>,
    pub kb_avail: Option<i32>,
    pub kb_used: Option<i32>,
    pub op_queue_age_hist: Option<OpQueueAgeHist>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct OpQueueAgeHist {
    pub upper_bound: Option<i32>,
    pub histogram: Option<Vec<i32>>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct FsPerfStat {
    pub apply_latency_ms: Option<i32>,
    pub commit_latency_ms: Option<i32>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct PoolStats {
    pub log_size: Option<i32>,
    pub ondisk_log_size: Option<i32>,
    pub up: Option<i32>,
    pub acting: Option<i32>,
    pub poolid: Option<i32>,
    pub stat_sum: Option<StatSum>,
}

#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct PgStats {
    pub last_scrub: Option<String>,
    pub last_clean_scrub_stamp: Option<String>,
    pub parent_split_bits: Option<i32>,
    pub last_active: Option<String>,
    pub pin_stats_invalid: Option<bool>,
    pub reported_epoch: Option<String>,
    pub log_start: Option<String>,
    pub log_size: Option<i32>,
    pub hitset_stats_invalid: Option<bool>,
    pub stats_invalid: Option<bool>,
    pub acting_primary: Option<i32>,
    pub reported_seq: Option<String>,
    pub ondisk_log_size: Option<i32>,
    pub mapping_epoch: Option<i32>,
    pub dirty_stats_invalid: Option<bool>,
    pub state: String,
    pub version: Option<String>,
    pub last_became_peered: Option<String>,
    pub last_undegraded: Option<String>,
    pub pgid: Option<String>,
    pub parent: Option<String>,
    pub acting: Option<Vec<i32>>,
    pub up_primary: Option<i32>,
    pub last_fullsized: Option<String>,
    pub last_epoch_clean: Option<i32>,
    pub last_deep_scrub_stamp: Option<String>,
    pub stat_sum: Option<StatSum>,
    pub last_deep_scrub: Option<String>,
    pub last_fresh: Option<String>,
    pub last_scrub_stamp: Option<String>,
    pub created: Option<i32>,
    pub up: Option<Vec<i32>>,
    pub hitset_bytes_stats_invalid: Option<bool>,
    pub last_peered: Option<String>,
    pub last_became_active: Option<String>,
    pub omap_stats_invalid: Option<bool>,
    pub last_clean: Option<String>,
    pub last_unstale: Option<String>,
    pub last_change: Option<String>,
    pub blocked_by: Option<Vec<i32>>,
    pub ondisk_log_start: Option<String>,
}

#[test]
#[should_panic]
fn pgmap_from_jewel_file() {
    use from::FromFile;
    let pgmap = PGMap::from_file("test/pg_dump_jewel.json").unwrap();
    assert_eq!(pgmap.min_last_epoch_clean.unwrap(), 1);
    assert_eq!(pgmap.osd_stats_sum, None);
}

#[test]
#[should_panic]
fn pgmap_from_ceph() {
    use from::FromCeph;
    let pgmap = PGMap::from_ceph("pg dump");
    assert_eq!(pgmap.is_ok(), true);
}
