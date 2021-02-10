mod contact;
use crate::{OckamError, ProfileChangeEvent, ProfileIdentifier, ProfileVault};
pub use contact::*;
use hashbrown::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

type ContactsDb = HashMap<ProfileIdentifier, Contact>;

pub struct Contacts {
    // TODO: it should be possible to store this to some persistent storage
    contacts: ContactsDb,
    vault: Arc<Mutex<dyn ProfileVault>>,
}

impl Contacts {
    pub fn new(contacts: ContactsDb, vault: Arc<Mutex<dyn ProfileVault>>) -> Self {
        Contacts { contacts, vault }
    }
}

impl Contacts {
    pub fn get_contact(&self, id: &ProfileIdentifier) -> Option<&Contact> {
        self.contacts.get(id)
    }

    pub fn add_contact(&mut self, contact: Contact) -> ockam_core::Result<()> {
        let mut vault = self.vault.lock().unwrap();
        contact.verify(vault.deref_mut())?;

        let _ = self.contacts.insert(contact.identifier().clone(), contact);

        Ok(())
    }

    pub fn update_contact(
        &mut self,
        id: &ProfileIdentifier,
        change_events: Vec<ProfileChangeEvent>,
    ) -> ockam_core::Result<()> {
        let contact;
        if let Some(c) = self.contacts.get_mut(id) {
            contact = c;
        } else {
            return Err(OckamError::ContactNotFound.into());
        }

        let mut vault = self.vault.lock().unwrap();

        contact.update(change_events, vault.deref_mut())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{KeyAttributes, Profile, ProfileKeyPurpose, ProfileKeyType};
    use ockam_vault::SoftwareVault;

    #[test]
    fn test_new() {
        let vault = SoftwareVault::default();
        let vault = Arc::new(Mutex::new(vault));
        let mut alice_profile = Profile::create(None, vault.clone()).unwrap();

        let truck_key_attributes = KeyAttributes::new(
            "Truck management".to_string(),
            ProfileKeyType::Issuing,
            ProfileKeyPurpose::IssueCredentials,
        );

        alice_profile
            .create_key(truck_key_attributes.clone(), None)
            .unwrap();

        let alice_id = alice_profile.identifier();
        let alice_contact = alice_profile.to_contact();

        // TODO: Serialize&Deserialize alice_contact

        let mut contacts = Contacts::new(Default::default(), vault);

        contacts.add_contact(alice_contact).unwrap();

        let _public_key = contacts
            .get_contact(alice_id)
            .unwrap()
            .get_public_key(&truck_key_attributes)
            .unwrap();
    }
}
