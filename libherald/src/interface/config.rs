use super::*;

pub struct ConfigQObject;

pub struct ConfigEmitter {
    pub(super) qobject: Arc<AtomicPtr<ConfigQObject>>,
    pub(super) config_id_changed: fn(*mut ConfigQObject),
    pub(super) nts_conversation_id_changed: fn(*mut ConfigQObject),
    pub(super) preferred_expiration_changed: fn(*mut ConfigQObject),
}

impl ConfigEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ConfigEmitter {
        ConfigEmitter {
            qobject: self.qobject.clone(),
            config_id_changed: self.config_id_changed,
            nts_conversation_id_changed: self.nts_conversation_id_changed,
            preferred_expiration_changed: self.preferred_expiration_changed,
        }
    }

    pub fn clear(&self) {
        let n: *const ConfigQObject = null();
        self.qobject
            .store(n as *mut ConfigQObject, Ordering::SeqCst);
    }

    pub fn config_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.config_id_changed)(ptr);
        }
    }

    pub fn nts_conversation_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.nts_conversation_id_changed)(ptr);
        }
    }

    pub fn preferred_expiration_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.preferred_expiration_changed)(ptr);
        }
    }
}

pub trait ConfigTrait {
    fn new(emit: ConfigEmitter) -> Self;

    fn emit(&mut self) -> &mut ConfigEmitter;

    fn config_id(&self) -> &str;

    fn nts_conversation_id(&self) -> &[u8];

    fn preferred_expiration(&self) -> u8;

    fn set_preferred_expiration(
        &mut self,
        value: u8,
    );

    fn set_name(
        &mut self,
        name: String,
    ) -> ();

    fn set_profile_picture(
        &mut self,
        profile_picture: String,
    ) -> ();
}

#[no_mangle]
pub unsafe extern "C" fn config_new(ptr_bundle: *mut ConfigPtrBundle) -> *mut Config {
    let d_config = config_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_config))
}

pub unsafe fn config_new_inner(ptr_bundle: *mut ConfigPtrBundle) -> Config {
    let ptr_bundle = *ptr_bundle;

    let ConfigPtrBundle {
        config,
        config_config_id_changed,
        config_nts_conversation_id_changed,
        config_preferred_expiration_changed,
    } = ptr_bundle;
    let config_emit = ConfigEmitter {
        qobject: Arc::new(AtomicPtr::new(config)),
        config_id_changed: config_config_id_changed,
        nts_conversation_id_changed: config_nts_conversation_id_changed,
        preferred_expiration_changed: config_preferred_expiration_changed,
    };
    let d_config = Config::new(config_emit);
    d_config
}

#[no_mangle]
pub unsafe extern "C" fn config_free(ptr: *mut Config) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn config_set_name(
    ptr: *mut Config,
    name_str: *const c_ushort,
    name_len: c_int,
) {
    let obj = &mut *ptr;
    let mut name = String::new();
    set_string_from_utf16(&mut name, name_str, name_len);
    obj.set_name(name)
}

#[no_mangle]
pub unsafe extern "C" fn config_set_profile_picture(
    ptr: *mut Config,
    profile_picture_str: *const c_ushort,
    profile_picture_len: c_int,
) {
    let obj = &mut *ptr;
    let mut profile_picture = String::new();
    set_string_from_utf16(
        &mut profile_picture,
        profile_picture_str,
        profile_picture_len,
    );
    obj.set_profile_picture(profile_picture)
}

#[no_mangle]
pub unsafe extern "C" fn config_config_id_get(
    ptr: *const Config,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.config_id();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn config_nts_conversation_id_get(
    ptr: *const Config,
    prop: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.nts_conversation_id();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn config_preferred_expiration_get(ptr: *const Config) -> u8 {
    (&*ptr).preferred_expiration()
}

#[no_mangle]
pub unsafe extern "C" fn config_preferred_expiration_set(
    ptr: *mut Config,
    value: u8,
) {
    (&mut *ptr).set_preferred_expiration(value)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConfigPtrBundle {
    config: *mut ConfigQObject,
    config_config_id_changed: fn(*mut ConfigQObject),
    config_nts_conversation_id_changed: fn(*mut ConfigQObject),
    config_preferred_expiration_changed: fn(*mut ConfigQObject),
}
