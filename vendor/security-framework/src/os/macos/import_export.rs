//! OSX specific extensions to import/export functionality.

use core_foundation::array::CFArray;
use core_foundation::base::{CFType, TCFType};
use core_foundation::data::CFData;
use core_foundation::string::CFString;
use core_foundation_sys::base::CFGetTypeID;
use security_framework_sys::base::errSecSuccess;
use security_framework_sys::import_export::*;
use std::ptr;
use std::str::FromStr;

use Pkcs12ImportOptionsInternals;
use access::SecAccess;
use base::{Error, Result};
use certificate::SecCertificate;
use identity::SecIdentity;
use import_export::Pkcs12ImportOptions;
use key::SecKey;
use keychain::SecKeychain;

/// An extension trait adding OSX specific functionality to `Pkcs12ImportOptions`.
pub trait Pkcs12ImportOptionsExt {
    /// Specifies the keychain in which to import the identity.
    ///
    /// If this is not called, the default keychain will be used.
    fn keychain(&mut self, keychain: SecKeychain) -> &mut Self;

    /// Specifies the access control to be associated with the identity.
    fn access(&mut self, access: SecAccess) -> &mut Self;
}

impl Pkcs12ImportOptionsExt for Pkcs12ImportOptions {
    fn keychain(&mut self, keychain: SecKeychain) -> &mut Self {
        Pkcs12ImportOptionsInternals::keychain(self, keychain)
    }

    fn access(&mut self, access: SecAccess) -> &mut Self {
        Pkcs12ImportOptionsInternals::access(self, access)
    }
}

/// A builder type to import Security Framework types from serialized formats.
#[derive(Default)]
pub struct ImportOptions<'a> {
    filename: Option<CFString>,
    passphrase: Option<CFType>,
    secure_passphrase: bool,
    no_access_control: bool,
    alert_title: Option<CFString>,
    alert_prompt: Option<CFString>,
    items: Option<&'a mut SecItems>,
    keychain: Option<SecKeychain>,
}

impl<'a> ImportOptions<'a> {
    /// Creates a new builder with default options.
    pub fn new() -> ImportOptions<'a> {
        ImportOptions::default()
    }

    /// Sets the filename from which the imported data came.
    ///
    /// The extension of the file will used as a hint for parsing.
    pub fn filename(&mut self, filename: &str) -> &mut ImportOptions<'a> {
        self.filename = Some(CFString::from_str(filename).unwrap());
        self
    }

    /// Sets the passphrase to be used to decrypt the imported data.
    pub fn passphrase(&mut self, passphrase: &str) -> &mut ImportOptions<'a> {
        self.passphrase = Some(CFString::from_str(passphrase).unwrap().as_CFType());
        self
    }

    /// Sets the passphrase to be used to decrypt the imported data.
    pub fn passphrase_bytes(&mut self, passphrase: &[u8]) -> &mut ImportOptions<'a> {
        self.passphrase = Some(CFData::from_buffer(passphrase).as_CFType());
        self
    }

    /// If set, the user will be prompted to imput the passphrase used to
    /// decrypt the imported data.
    pub fn secure_passphrase(&mut self, secure_passphrase: bool) -> &mut ImportOptions<'a> {
        self.secure_passphrase = secure_passphrase;
        self
    }

    /// If set, imported items will have no access controls imposed on them.
    pub fn no_access_control(&mut self, no_access_control: bool) -> &mut ImportOptions<'a> {
        self.no_access_control = no_access_control;
        self
    }

    /// Sets the title of the alert popup used with the `secure_passphrase`
    /// option.
    pub fn alert_title(&mut self, alert_title: &str) -> &mut ImportOptions<'a> {
        self.alert_title = Some(CFString::from_str(alert_title).unwrap());
        self
    }

    /// Sets the prompt of the alert popup used with the `secure_passphrase`
    /// option.
    pub fn alert_prompt(&mut self, alert_prompt: &str) -> &mut ImportOptions<'a> {
        self.alert_prompt = Some(CFString::from_str(alert_prompt).unwrap());
        self
    }

    /// Sets the object into which imported items will be placed.
    pub fn items(&mut self, items: &'a mut SecItems) -> &mut ImportOptions<'a> {
        self.items = Some(items);
        self
    }

    /// Sets the keychain into which items will be imported.
    ///
    /// This must be specified to import `SecIdentity`s.
    pub fn keychain(&mut self, keychain: &SecKeychain) -> &mut ImportOptions<'a> {
        self.keychain = Some(keychain.clone());
        self
    }

    /// Imports items from serialized data.
    pub fn import(&mut self, data: &[u8]) -> Result<()> {
        let data = CFData::from_buffer(data);
        let data = data.as_concrete_TypeRef();

        let filename = match self.filename {
            Some(ref filename) => filename.as_concrete_TypeRef(),
            None => ptr::null(),
        };

        let mut key_params = SecItemImportExportKeyParameters {
            version: SEC_KEY_IMPORT_EXPORT_PARAMS_VERSION,
            flags: 0,
            passphrase: ptr::null(),
            alert_title: ptr::null(),
            alert_prompt: ptr::null(),
            access_ref: ptr::null_mut(),
            key_usage: ptr::null_mut(),
            key_attributes: ptr::null(),
        };

        if let Some(ref passphrase) = self.passphrase {
            key_params.passphrase = passphrase.as_CFTypeRef();
        }

        if self.secure_passphrase {
            key_params.flags |= kSecKeySecurePassphrase;
        }

        if self.no_access_control {
            key_params.flags |= kSecKeyNoAccessControl;
        }

        if let Some(ref alert_title) = self.alert_title {
            key_params.alert_title = alert_title.as_concrete_TypeRef();
        }

        if let Some(ref alert_prompt) = self.alert_prompt {
            key_params.alert_prompt = alert_prompt.as_concrete_TypeRef();
        }

        let keychain = match self.keychain {
            Some(ref keychain) => keychain.as_concrete_TypeRef(),
            None => ptr::null_mut(),
        };

        let mut raw_items = ptr::null();
        let items_ref = match self.items {
            Some(_) => &mut raw_items as *mut _,
            None => ptr::null_mut(),
        };

        unsafe {
            let ret = SecItemImport(data,
                                    filename,
                                    ptr::null_mut(),
                                    ptr::null_mut(),
                                    0,
                                    &mut key_params,
                                    keychain,
                                    items_ref);
            if ret != errSecSuccess {
                return Err(Error::from_code(ret));
            }

            if let Some(ref mut items) = self.items {
                let raw_items = CFArray::wrap_under_create_rule(raw_items);
                for item in raw_items.iter() {
                    let type_id = CFGetTypeID(item as *mut _);
                    if type_id == SecCertificate::type_id() {
                        items.certificates
                             .push(SecCertificate::wrap_under_get_rule(item as *mut _));
                    } else if type_id == SecIdentity::type_id() {
                        items.identities.push(SecIdentity::wrap_under_get_rule(item as *mut _));
                    } else if type_id == SecKey::type_id() {
                        items.keys.push(SecKey::wrap_under_get_rule(item as *mut _));
                    } else {
                        panic!("Got bad type from SecItemImport: {}", type_id);
                    }
                }
            }
        }

        Ok(())
    }
}

/// A type which holds items imported from serialized data.
///
/// Pass a reference to `ImportOptions::items`.
#[derive(Default)]
pub struct SecItems {
    /// Imported certificates.
    pub certificates: Vec<SecCertificate>,
    /// Imported identities.
    pub identities: Vec<SecIdentity>,
    /// Imported keys.
    pub keys: Vec<SecKey>,
}

#[cfg(test)]
mod test {
    use tempdir::TempDir;
    use hex::ToHex;

    use super::*;
    use os::macos::keychain;
    use import_export::*;

    #[test]
    fn certificate() {
        let data = include_bytes!("../../../test/server.der");
        let mut items = SecItems::default();
        ImportOptions::new()
            .filename("server.der")
            .items(&mut items)
            .import(data)
            .unwrap();
        assert_eq!(1, items.certificates.len());
        assert_eq!(0, items.identities.len());
        assert_eq!(0, items.keys.len());
    }

    #[test]
    fn key() {
        let data = include_bytes!("../../../test/server.key");
        let mut items = SecItems::default();
        ImportOptions::new()
            .filename("server.key")
            .items(&mut items)
            .import(data)
            .unwrap();
        assert_eq!(0, items.certificates.len());
        assert_eq!(0, items.identities.len());
        assert_eq!(1, items.keys.len());
    }

    #[test]
    fn identity() {
        let dir = TempDir::new("identity").unwrap();
        let keychain = keychain::CreateOptions::new()
                           .password("password")
                           .create(dir.path().join("identity.keychain"))
                           .unwrap();

        let data = include_bytes!("../../../test/server.p12");
        let mut items = SecItems::default();
        ImportOptions::new()
            .filename("server.p12")
            .passphrase("password123")
            .items(&mut items)
            .keychain(&keychain)
            .import(data)
            .unwrap();
        assert_eq!(1, items.identities.len());
        assert_eq!(0, items.certificates.len());
        assert_eq!(0, items.keys.len());
    }

    #[test]
    #[ignore] // since it requires manual intervention
    fn secure_passphrase_identity() {
        let dir = TempDir::new("identity").unwrap();
        let keychain = keychain::CreateOptions::new()
                           .password("password")
                           .create(dir.path().join("identity.keychain"))
                           .unwrap();

        let data = include_bytes!("../../../test/server.p12");
        let mut items = SecItems::default();
        ImportOptions::new()
            .filename("server.p12")
            .secure_passphrase(true)
            .alert_title("alert title")
            .alert_prompt("alert prompt")
            .items(&mut items)
            .keychain(&keychain)
            .import(data)
            .unwrap();
        assert_eq!(1, items.identities.len());
        assert_eq!(0, items.certificates.len());
        assert_eq!(0, items.keys.len());
    }

    #[test]
    fn pkcs12_import() {
        let dir = TempDir::new("pkcs12_import").unwrap();
        let keychain = keychain::CreateOptions::new()
                           .password("password")
                           .create(dir.path().join("pkcs12_import"))
                           .unwrap();

        let data = include_bytes!("../../../test/server.p12");
        let identities = p!(Pkcs12ImportOptions::new()
                                .passphrase("password123")
                                .keychain(keychain)
                                .import(data));
        assert_eq!(1, identities.len());
        assert_eq!(identities[0].key_id.to_hex(),
                   "ed6492936dcc8907e397e573b36e633458dc33f1");
    }
}
