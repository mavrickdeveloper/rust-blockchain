use std::collections::HashMap;

// Define the Stakeholder and Delegate structures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stakeholder {
    pub id: u64,
    pub stake: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Delegate {
    pub id: u64,
    pub votes: u64,
}

// Define the Delegated Proof of Stake structure.
pub struct DelegatedProofOfStake {
    pub stakeholders: HashMap<u64, Stakeholder>,
    pub delegates: Vec<Delegate>,
}

impl DelegatedProofOfStake {
    // Create a new, empty Delegated Proof of Stake instance.
    pub fn new() -> Self {
        Self {
            stakeholders: HashMap::new(),
            delegates: Vec::new(),
        }
    }

    // Add a new stakeholder to the Delegated Proof of Stake instance.
    pub fn add_stakeholder(&mut self, id: u64, stake: u64) {
        self.stakeholders.insert(id, Stakeholder { id, stake });
    }

    // Add a new delegate to the Delegated Proof of Stake instance.
    pub fn add_delegate(&mut self, id: u64) {
        let delegate = Delegate { id, votes: 0 };
        self.delegates.push(delegate);
    }

    // A stakeholder votes for a delegate.
    pub fn vote(&mut self, stakeholder_id: u64, delegate_id: u64) -> bool {
        if let Some(stakeholder) = self.stakeholders.get(&stakeholder_id) {
            let stake = stakeholder.stake;
            if let Some(delegate) = self.delegates.iter_mut().find(|d| d.id == delegate_id) {
                delegate.votes += stake;
                return true;
            }
        }
        false
    }

    // Get the top N delegates based on votes.
    pub fn get_top_delegates(&self, n: usize) -> Vec<Delegate> {
        let mut sorted_delegates = self.delegates.clone();
        sorted_delegates.sort_by(|a, b| b.votes.cmp(&a.votes));

        sorted_delegates.into_iter().take(n).collect()
    }
}

fn main() {
    // Create a new Delegated Proof of Stake instance.
    let mut dpos = DelegatedProofOfStake::new();

    // Add stakeholders.
    dpos.add_stakeholder(1, 100);
    dpos.add_stakeholder(2, 200);
    dpos.add_stakeholder(3, 300);

    // Add delegates.
    dpos.add_delegate(11);
    dpos.add_delegate(12);
    dpos.add_delegate(13);

    // Stakeholders vote for delegates.
    dpos.vote(1, 11);
    dpos.vote(2, 12);
    dpos.vote(3, 13);

    // Get the top 3 delegates.
    let top_delegates = dpos.get_top_delegates(3);
    println!("Top delegates: {:?}", top_delegates);
}
