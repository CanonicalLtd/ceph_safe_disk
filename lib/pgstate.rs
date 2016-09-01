use std::str::FromStr;
use std::collections::BTreeSet;

// `Pending` PGs are stuck or peering and might still be recoverable, so we
// cannot say with certainty whether they're safe or not.
#[derive(PartialOrd, PartialEq, Debug, Clone, Eq, Ord)]
pub enum RmSafety {
    None,
    Pending,
    Total,
}

// Only `Total` safety is when PG is 'active+clean', where some states are
// completely unsafe. The rest are when objects are being moved around and the
// state cannot be completely determined.
impl RmSafety {
    pub fn new(states: &String) -> RmSafety {
        let pg_states = PgState::parse_state(states);
        if pg_states.contains(&PgState::Active) && pg_states.contains(&PgState::Clean) {
            return RmSafety::Total;
        } else if pg_states.contains(&PgState::Backfill) ||
           pg_states.contains(&PgState::BackfillToofull) ||
           pg_states.contains(&PgState::WaitBackfill) ||
           pg_states.contains(&PgState::Down) ||
           pg_states.contains(&PgState::Undersized) ||
           pg_states.contains(&PgState::Incomplete) {
            return RmSafety::None;
        } else {
            return RmSafety::Pending;
        }
    }
}

// Enum used to better determine placement group states
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum PgState {
    Creating,
    Active,
    Clean,
    Down,
    Replay,
    Splitting,
    Scrubbing,
    Degraded,
    Inconsistent,
    Peering,
    Repair,
    Recovering,
    Backfill,
    WaitBackfill,
    BackfillToofull,
    Incomplete,
    Stale,
    Remapped,
    Undersized,
    Peered,
}

impl FromStr for PgState {
    type Err = ();

    fn from_str(state: &str) -> Result<Self, Self::Err> {
        match state {
            "creating" => Ok(PgState::Creating),
            "active" => Ok(PgState::Active),
            "clean" => Ok(PgState::Clean),
            "down" => Ok(PgState::Down),
            "replay" => Ok(PgState::Replay),
            "splitting" => Ok(PgState::Splitting),
            "scrubbing" => Ok(PgState::Scrubbing),
            "degraded" => Ok(PgState::Degraded),
            "inconsistent" => Ok(PgState::Inconsistent),
            "peering" => Ok(PgState::Peering),
            "repair" => Ok(PgState::Repair),
            "recovering" => Ok(PgState::Recovering),
            "backfill" => Ok(PgState::Backfill),
            "wait-backfill" => Ok(PgState::WaitBackfill),
            "backfill-toofull" => Ok(PgState::BackfillToofull),
            "incomplete" => Ok(PgState::Incomplete),
            "stale" => Ok(PgState::Stale),
            "remapped" => Ok(PgState::Remapped),
            "undersized" => Ok(PgState::Undersized),
            "peered" => Ok(PgState::Peered),
            _ => Err(()),
        }
    }
}

impl PgState {
    pub fn parse_state(state: &String) -> BTreeSet<PgState> {
        let states: Vec<&str> = state.split('+').collect();
        let mut parsed_states = BTreeSet::new();
        for state in states {
            if let Ok(new_state) = state.parse::<PgState>() {
                parsed_states.insert(new_state);
            }
        }
        parsed_states
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pg_state_parse_active_clean() {
        let states = PgState::parse_state(&String::from("active+clean"));
        assert_eq!(states.contains(&PgState::Active), true);
        assert_eq!(states.contains(&PgState::Clean), true);
    }

    #[test]
    fn pg_state_parse_down_waitbackfill() {
        let states = PgState::parse_state(&String::from("down+wait-backfill"));
        assert_eq!(states.contains(&PgState::Down), true);
        assert_eq!(states.contains(&PgState::WaitBackfill), true);
        assert_eq!(states.contains(&PgState::Active), false);
    }

    #[test]
    fn pg_state_parse_down_degraded_stale() {
        let states = PgState::parse_state(&String::from("down+degraded+stale"));
        assert_eq!(states.contains(&PgState::Down), true);
        assert_eq!(states.contains(&PgState::Degraded), true);
        assert_eq!(states.contains(&PgState::Stale), true);
    }

    #[test]
    fn pg_state_parse_none() {
        let states = PgState::parse_state(&String::from(""));
        assert_eq!(states.len(), 0);
    }

    #[test]
    fn pg_state_none() {
        let state = "";
        assert_eq!(state.parse::<PgState>(), Err(()));
    }

    #[test]
    fn rm_safety_safe() {
        let safety = RmSafety::new(&String::from("active+clean"));
        assert_eq!(safety, RmSafety::Total);
    }

    // Following PGs are down and out
    #[test]
    fn rm_safety_peering_failure() {
        let safety = RmSafety::new(&String::from("down+peering"));
        assert_eq!(safety, RmSafety::None);
    }

    #[test]
    fn rm_safety_down() {
        let safety = RmSafety::new(&String::from("down"));
        assert_eq!(safety, RmSafety::None);
    }

    // Following PGs might still be okay
    #[test]
    fn rm_safety_unfound_objects() {
        let safety = RmSafety::new(&String::from("active+degraded"));
        assert_eq!(safety, RmSafety::Pending);
    }

    #[test]
    fn rm_safety_homeless_pg() {
        let safety = RmSafety::new(&String::from("stale+active+remapped"));
        assert_eq!(safety, RmSafety::Pending);
    }

    #[test]
    fn rm_safety_pending() {
        let safety = RmSafety::new(&String::from("peering"));
        assert_eq!(safety, RmSafety::Pending);
    }

}
