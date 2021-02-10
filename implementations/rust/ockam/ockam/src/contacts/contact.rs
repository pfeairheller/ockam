use crate::profile_change_history::ProfileChangeHistory;
use crate::{KeyAttributes, ProfileChangeEvent, ProfileIdentifier, ProfileVault};
use ockam_vault_core::PublicKey;

#[derive(Clone)]
pub struct Contact {
    identifier: ProfileIdentifier,
    change_history: ProfileChangeHistory,
}

impl Contact {
    pub fn identifier(&self) -> &ProfileIdentifier {
        &self.identifier
    }
    pub fn change_events(&self) -> &[ProfileChangeEvent] {
        self.change_history.as_ref()
    }
}

impl Contact {
    pub fn new(identifier: ProfileIdentifier, change_events: Vec<ProfileChangeEvent>) -> Self {
        Contact {
            identifier,
            change_history: ProfileChangeHistory::new(change_events),
        }
    }
}

impl Contact {
    pub fn verify(&self, vault: &mut dyn ProfileVault) -> ockam_core::Result<()> {
        for change_event in self.change_events().as_ref() {
            self.change_history.verify(change_event, vault)?;
        }

        Ok(())
    }

    pub fn update(
        &mut self,
        change_events: Vec<ProfileChangeEvent>,
        vault: &mut dyn ProfileVault,
    ) -> ockam_core::Result<()> {
        for change in change_events.iter() {
            self.change_history.verify(change, vault)?;
            self.change_history.push_event(change.clone());
        }

        Ok(())
    }
}

impl Contact {
    pub fn get_public_key(&self, key_attributes: &KeyAttributes) -> ockam_core::Result<PublicKey> {
        self.change_history.get_public_key(key_attributes)
    }
}
