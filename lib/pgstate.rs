use std::str::FromStr;

// Enum used to better determine placement group states
#[derive(Debug, PartialEq)]
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
    fn parse_state(state: &String) -> Vec<PgState> {
        let states: Vec<&str> = state.split('+').collect();
        let mut parsed_states = Vec::new();
        for state in states {
            if let Ok(new_state) = state.parse::<PgState>() {
                parsed_states.push(new_state);
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
        assert_eq!(states[0], PgState::Active);
        assert_eq!(states[1], PgState::Clean);
    }

    #[test]
    fn pg_state_parse_down_waitbackfill() {
        let states = PgState::parse_state(&String::from("down+wait-backfill"));
        assert_eq!(states[0], PgState::Down);
        assert_eq!(states[1], PgState::WaitBackfill);
    }

    #[test]
    fn pg_state_parse_down_degraded_stale() {
        let states = PgState::parse_state(&String::from("down+degraded+stale"));
        assert_eq!(states[0], PgState::Down);
        assert_eq!(states[1], PgState::Degraded);
        assert_eq!(states[2], PgState::Stale);
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
}
