# ceph-safe-disk
[![Build Status](https://travis-ci.org/CanonicalLtd/ceph_safe_disk.svg?branch=master)](https://travis-ci.org/CanonicalLtd/ceph_safe_disk)

Ceph disk safety removal tool

# Building
Builds on stable `rustc` as of 1.8

# Using
`ceph-safe-disk` checks whether OSDs in a ceph cluster are removable or not.
This is done in two ways, the quick way, and the more exhaustive way.

```
Usage: ceph-safe-disk [OPTION]

Options:
    -h, --help          Print help information
    -q, --quick         Give a quick, non-exhaustive status of removable OSDs
    -e, --exhaustive    Give an exhaustive status of removable OSDs

Exit statuses:
    0: Safe to remove an OSD
    1: Not safe to remove an OSD
    2: General error

```

**Quick**

The quick option `-q` checks whether the cluster's minimum OSD size is
satisfied (that is the current size is `min_size + 1`). This could be unsafe as
a placement group might not be reproduced.

**Exhaustive**

The more exhaustive option `-e` maps out placement groups to OSDs and then checks
the safety of each individual OSD. If a placement group is marked unsafe then
the OSD is marked unsafe as well. This is done for each OSD in a placement
group's acting OSD list.

### A Note on Timing
The timing of this tool depends on when ceph reports the status of OSDs. More
information can be found on the [ceph documentation page here](http://docs.ceph.com/docs/master/rados/configuration/mon-osd-interaction/#osds-report-their-status).

# Safety
`ceph-safe-disk` marks OSDs in the following states: `Pending`, `Removable`, and
`Not removable`. `Removable` for an OSD being asbolutely safe to remove, `Not removable` 
for an OSD that is unsafe to remove, and `Pending` for an OSD that's safety cannot 
be guarenteed.

The only state for a placement group marked as absolutely safe is `active+clean`.
Some liberty was taken as to what is regarded as safe, unsafe, and pending.

PG states that currently immediately result in unsafe are:

- backfill
- backfill-toofull
- wait-backfill
- down
- undersized
- incomplete

Any other state is marked as `Pending`.
