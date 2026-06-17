use super::error::{DbError, DbResult};
use super::models::ItemStatus;

pub fn can_transition(from: ItemStatus, to: ItemStatus) -> bool {
    if from == to {
        return true;
    }

    use ItemStatus::*;
    matches!(
        (from, to),
        (Inbox, Brewing)
            | (Inbox, Archived)
            | (Brewing, Inbox)
            | (Brewing, Ready)
            | (Brewing, Archived)
            | (Ready, Brewing)
            | (Ready, Producing)
            | (Ready, Archived)
            | (Producing, Shipped)
            | (Producing, Archived)
            | (Shipped, Archived)
            | (Archived, Inbox)
    )
}

pub fn assert_transition(from: ItemStatus, to: ItemStatus) -> DbResult<()> {
    if can_transition(from, to) {
        Ok(())
    } else {
        Err(DbError::InvalidStatusTransition {
            from: from.as_str().to_string(),
            to: to.as_str().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::models::ItemStatus::*;

    #[test]
    fn allows_inbox_to_brewing() {
        assert!(can_transition(Inbox, Brewing));
    }

    #[test]
    fn blocks_inbox_to_producing() {
        assert!(!can_transition(Inbox, Producing));
    }

    #[test]
    fn allows_ready_to_producing() {
        assert!(can_transition(Ready, Producing));
    }
}
