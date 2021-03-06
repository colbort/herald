use super::*;

pub struct HeraldQObject;

pub struct HeraldEmitter {
    pub(super) qobject: Arc<AtomicPtr<HeraldQObject>>,
    pub(super) config_init_changed: fn(*mut HeraldQObject),
    pub(super) connection_pending_changed: fn(*mut HeraldQObject),
    pub(super) connection_up_changed: fn(*mut HeraldQObject),
    pub(super) registration_failure_code_changed: fn(*mut HeraldQObject),
    pub(super) try_poll: fn(*mut HeraldQObject),
}

impl HeraldEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> HeraldEmitter {
        HeraldEmitter {
            qobject: self.qobject.clone(),
            config_init_changed: self.config_init_changed,
            connection_pending_changed: self.connection_pending_changed,
            connection_up_changed: self.connection_up_changed,
            registration_failure_code_changed: self.registration_failure_code_changed,
            try_poll: self.try_poll,
        }
    }

    pub fn clear(&self) {
        let n: *const HeraldQObject = null();
        self.qobject
            .store(n as *mut HeraldQObject, Ordering::SeqCst);
    }

    pub fn config_init_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.config_init_changed)(ptr);
        }
    }

    pub fn connection_pending_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.connection_pending_changed)(ptr);
        }
    }

    pub fn connection_up_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.connection_up_changed)(ptr);
        }
    }

    pub fn registration_failure_code_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.registration_failure_code_changed)(ptr);
        }
    }

    pub fn try_poll(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.try_poll)(ptr);
        }
    }
}

pub trait HeraldTrait {
    fn new(
        emit: HeraldEmitter,
        config: Config,
        conversation_builder: ConversationBuilder,
        conversations: Conversations,
        errors: Errors,
        message_search: MessageSearch,
        notifications: Notifications,
        users: Users,
        users_search: UsersSearch,
        utils: Utils,
    ) -> Self;

    fn emit(&mut self) -> &mut HeraldEmitter;

    fn config(&self) -> &Config;

    fn config_mut(&mut self) -> &mut Config;

    fn config_init(&self) -> bool;

    fn connection_pending(&self) -> bool;

    fn connection_up(&self) -> bool;

    fn conversation_builder(&self) -> &ConversationBuilder;

    fn conversation_builder_mut(&mut self) -> &mut ConversationBuilder;

    fn conversations(&self) -> &Conversations;

    fn conversations_mut(&mut self) -> &mut Conversations;

    fn errors(&self) -> &Errors;

    fn errors_mut(&mut self) -> &mut Errors;

    fn message_search(&self) -> &MessageSearch;

    fn message_search_mut(&mut self) -> &mut MessageSearch;

    fn notifications(&self) -> &Notifications;

    fn notifications_mut(&mut self) -> &mut Notifications;

    fn registration_failure_code(&self) -> Option<u8>;

    fn users(&self) -> &Users;

    fn users_mut(&mut self) -> &mut Users;

    fn users_search(&self) -> &UsersSearch;

    fn users_search_mut(&mut self) -> &mut UsersSearch;

    fn utils(&self) -> &Utils;

    fn utils_mut(&mut self) -> &mut Utils;

    fn login(&mut self) -> bool;

    fn poll_update(&mut self) -> ();

    fn register_new_user(
        &mut self,
        user_id: String,
        addr: String,
        port: String,
    ) -> ();

    fn set_app_local_data_dir(
        &mut self,
        path: String,
    ) -> ();
}

#[no_mangle]
pub unsafe extern "C" fn herald_new(ptr_bundle: *mut HeraldPtrBundle) -> *mut Herald {
    let d_herald = herald_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_herald))
}

pub unsafe fn herald_new_inner(ptr_bundle: *mut HeraldPtrBundle) -> Herald {
    let ptr_bundle = *ptr_bundle;

    let HeraldPtrBundle {
        herald,
        config,
        config_config_id_changed,
        config_nts_conversation_id_changed,
        config_preferred_expiration_changed,
        herald_config_init_changed,
        herald_connection_pending_changed,
        herald_connection_up_changed,
        conversation_builder,
        conversation_builder_picture_changed,
        conversation_builder_new_data_ready,
        conversation_builder_layout_about_to_be_changed,
        conversation_builder_layout_changed,
        conversation_builder_data_changed,
        conversation_builder_begin_reset_model,
        conversation_builder_end_reset_model,
        conversation_builder_begin_insert_rows,
        conversation_builder_end_insert_rows,
        conversation_builder_begin_move_rows,
        conversation_builder_end_move_rows,
        conversation_builder_begin_remove_rows,
        conversation_builder_end_remove_rows,
        conversations,
        conversations_builder_conversation_id_changed,
        conversations_filter_changed,
        conversations_filter_regex_changed,
        conversations_new_data_ready,
        conversations_layout_about_to_be_changed,
        conversations_layout_changed,
        conversations_data_changed,
        conversations_begin_reset_model,
        conversations_end_reset_model,
        conversations_begin_insert_rows,
        conversations_end_insert_rows,
        conversations_begin_move_rows,
        conversations_end_move_rows,
        conversations_begin_remove_rows,
        conversations_end_remove_rows,
        errors,
        errors_new_error,
        message_search,
        message_search_regex_search_changed,
        message_search_search_pattern_changed,
        message_search_new_data_ready,
        message_search_layout_about_to_be_changed,
        message_search_layout_changed,
        message_search_data_changed,
        message_search_begin_reset_model,
        message_search_end_reset_model,
        message_search_begin_insert_rows,
        message_search_end_insert_rows,
        message_search_begin_move_rows,
        message_search_end_move_rows,
        message_search_begin_remove_rows,
        message_search_end_remove_rows,
        notifications,
        notifications_notify,
        herald_registration_failure_code_changed,
        users,
        users_filter_changed,
        users_filter_regex_changed,
        users_new_data_ready,
        users_layout_about_to_be_changed,
        users_layout_changed,
        users_data_changed,
        users_begin_reset_model,
        users_end_reset_model,
        users_begin_insert_rows,
        users_end_insert_rows,
        users_begin_move_rows,
        users_end_move_rows,
        users_begin_remove_rows,
        users_end_remove_rows,
        users_search,
        users_search_filter_changed,
        users_search_new_data_ready,
        users_search_layout_about_to_be_changed,
        users_search_layout_changed,
        users_search_data_changed,
        users_search_begin_reset_model,
        users_search_end_reset_model,
        users_search_begin_insert_rows,
        users_search_end_insert_rows,
        users_search_begin_move_rows,
        users_search_end_move_rows,
        users_search_begin_remove_rows,
        users_search_end_remove_rows,
        utils,
        herald_try_poll,
    } = ptr_bundle;
    let config_emit = ConfigEmitter {
        qobject: Arc::new(AtomicPtr::new(config)),
        config_id_changed: config_config_id_changed,
        nts_conversation_id_changed: config_nts_conversation_id_changed,
        preferred_expiration_changed: config_preferred_expiration_changed,
    };
    let d_config = Config::new(config_emit);
    let conversation_builder_emit = ConversationBuilderEmitter {
        qobject: Arc::new(AtomicPtr::new(conversation_builder)),
        picture_changed: conversation_builder_picture_changed,
        new_data_ready: conversation_builder_new_data_ready,
    };
    let model = ConversationBuilderList {
        qobject: conversation_builder,
        layout_about_to_be_changed: conversation_builder_layout_about_to_be_changed,
        layout_changed: conversation_builder_layout_changed,
        data_changed: conversation_builder_data_changed,
        begin_reset_model: conversation_builder_begin_reset_model,
        end_reset_model: conversation_builder_end_reset_model,
        begin_insert_rows: conversation_builder_begin_insert_rows,
        end_insert_rows: conversation_builder_end_insert_rows,
        begin_move_rows: conversation_builder_begin_move_rows,
        end_move_rows: conversation_builder_end_move_rows,
        begin_remove_rows: conversation_builder_begin_remove_rows,
        end_remove_rows: conversation_builder_end_remove_rows,
    };
    let d_conversation_builder = ConversationBuilder::new(conversation_builder_emit, model);
    let conversations_emit = ConversationsEmitter {
        qobject: Arc::new(AtomicPtr::new(conversations)),
        builder_conversation_id_changed: conversations_builder_conversation_id_changed,
        filter_changed: conversations_filter_changed,
        filter_regex_changed: conversations_filter_regex_changed,
        new_data_ready: conversations_new_data_ready,
    };
    let model = ConversationsList {
        qobject: conversations,
        layout_about_to_be_changed: conversations_layout_about_to_be_changed,
        layout_changed: conversations_layout_changed,
        data_changed: conversations_data_changed,
        begin_reset_model: conversations_begin_reset_model,
        end_reset_model: conversations_end_reset_model,
        begin_insert_rows: conversations_begin_insert_rows,
        end_insert_rows: conversations_end_insert_rows,
        begin_move_rows: conversations_begin_move_rows,
        end_move_rows: conversations_end_move_rows,
        begin_remove_rows: conversations_begin_remove_rows,
        end_remove_rows: conversations_end_remove_rows,
    };
    let d_conversations = Conversations::new(conversations_emit, model);
    let errors_emit = ErrorsEmitter {
        qobject: Arc::new(AtomicPtr::new(errors)),
        new_error: errors_new_error,
    };
    let d_errors = Errors::new(errors_emit);
    let message_search_emit = MessageSearchEmitter {
        qobject: Arc::new(AtomicPtr::new(message_search)),
        regex_search_changed: message_search_regex_search_changed,
        search_pattern_changed: message_search_search_pattern_changed,
        new_data_ready: message_search_new_data_ready,
    };
    let model = MessageSearchList {
        qobject: message_search,
        layout_about_to_be_changed: message_search_layout_about_to_be_changed,
        layout_changed: message_search_layout_changed,
        data_changed: message_search_data_changed,
        begin_reset_model: message_search_begin_reset_model,
        end_reset_model: message_search_end_reset_model,
        begin_insert_rows: message_search_begin_insert_rows,
        end_insert_rows: message_search_end_insert_rows,
        begin_move_rows: message_search_begin_move_rows,
        end_move_rows: message_search_end_move_rows,
        begin_remove_rows: message_search_begin_remove_rows,
        end_remove_rows: message_search_end_remove_rows,
    };
    let d_message_search = MessageSearch::new(message_search_emit, model);
    let notifications_emit = NotificationsEmitter {
        qobject: Arc::new(AtomicPtr::new(notifications)),
        notify: notifications_notify,
    };
    let d_notifications = Notifications::new(notifications_emit);
    let users_emit = UsersEmitter {
        qobject: Arc::new(AtomicPtr::new(users)),
        filter_changed: users_filter_changed,
        filter_regex_changed: users_filter_regex_changed,
        new_data_ready: users_new_data_ready,
    };
    let model = UsersList {
        qobject: users,
        layout_about_to_be_changed: users_layout_about_to_be_changed,
        layout_changed: users_layout_changed,
        data_changed: users_data_changed,
        begin_reset_model: users_begin_reset_model,
        end_reset_model: users_end_reset_model,
        begin_insert_rows: users_begin_insert_rows,
        end_insert_rows: users_end_insert_rows,
        begin_move_rows: users_begin_move_rows,
        end_move_rows: users_end_move_rows,
        begin_remove_rows: users_begin_remove_rows,
        end_remove_rows: users_end_remove_rows,
    };
    let d_users = Users::new(users_emit, model);
    let users_search_emit = UsersSearchEmitter {
        qobject: Arc::new(AtomicPtr::new(users_search)),
        filter_changed: users_search_filter_changed,
        new_data_ready: users_search_new_data_ready,
    };
    let model = UsersSearchList {
        qobject: users_search,
        layout_about_to_be_changed: users_search_layout_about_to_be_changed,
        layout_changed: users_search_layout_changed,
        data_changed: users_search_data_changed,
        begin_reset_model: users_search_begin_reset_model,
        end_reset_model: users_search_end_reset_model,
        begin_insert_rows: users_search_begin_insert_rows,
        end_insert_rows: users_search_end_insert_rows,
        begin_move_rows: users_search_begin_move_rows,
        end_move_rows: users_search_end_move_rows,
        begin_remove_rows: users_search_begin_remove_rows,
        end_remove_rows: users_search_end_remove_rows,
    };
    let d_users_search = UsersSearch::new(users_search_emit, model);
    let utils_emit = UtilsEmitter {
        qobject: Arc::new(AtomicPtr::new(utils)),
    };
    let d_utils = Utils::new(utils_emit);
    let herald_emit = HeraldEmitter {
        qobject: Arc::new(AtomicPtr::new(herald)),
        config_init_changed: herald_config_init_changed,
        connection_pending_changed: herald_connection_pending_changed,
        connection_up_changed: herald_connection_up_changed,
        registration_failure_code_changed: herald_registration_failure_code_changed,
        try_poll: herald_try_poll,
    };
    let d_herald = Herald::new(
        herald_emit,
        d_config,
        d_conversation_builder,
        d_conversations,
        d_errors,
        d_message_search,
        d_notifications,
        d_users,
        d_users_search,
        d_utils,
    );
    d_herald
}

#[no_mangle]
pub unsafe extern "C" fn herald_free(ptr: *mut Herald) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn herald_login(ptr: *mut Herald) -> bool {
    let obj = &mut *ptr;
    obj.login()
}

#[no_mangle]
pub unsafe extern "C" fn herald_poll_update(ptr: *mut Herald) {
    let obj = &mut *ptr;
    obj.poll_update()
}

#[no_mangle]
pub unsafe extern "C" fn herald_register_new_user(
    ptr: *mut Herald,
    user_id_str: *const c_ushort,
    user_id_len: c_int,
    addr_str: *const c_ushort,
    addr_len: c_int,
    port_str: *const c_ushort,
    port_len: c_int,
) {
    let obj = &mut *ptr;
    let mut user_id = String::new();
    set_string_from_utf16(&mut user_id, user_id_str, user_id_len);
    let mut addr = String::new();
    set_string_from_utf16(&mut addr, addr_str, addr_len);
    let mut port = String::new();
    set_string_from_utf16(&mut port, port_str, port_len);
    obj.register_new_user(user_id, addr, port)
}

#[no_mangle]
pub unsafe extern "C" fn herald_set_app_local_data_dir(
    ptr: *mut Herald,
    path_str: *const c_ushort,
    path_len: c_int,
) {
    let obj = &mut *ptr;
    let mut path = String::new();
    set_string_from_utf16(&mut path, path_str, path_len);
    obj.set_app_local_data_dir(path)
}

#[no_mangle]
pub unsafe extern "C" fn herald_config_get(ptr: *mut Herald) -> *mut Config {
    (&mut *ptr).config_mut()
}

#[no_mangle]
pub unsafe extern "C" fn herald_config_init_get(ptr: *const Herald) -> bool {
    (&*ptr).config_init()
}

#[no_mangle]
pub unsafe extern "C" fn herald_connection_pending_get(ptr: *const Herald) -> bool {
    (&*ptr).connection_pending()
}

#[no_mangle]
pub unsafe extern "C" fn herald_connection_up_get(ptr: *const Herald) -> bool {
    (&*ptr).connection_up()
}

#[no_mangle]
pub unsafe extern "C" fn herald_conversation_builder_get(
    ptr: *mut Herald
) -> *mut ConversationBuilder {
    (&mut *ptr).conversation_builder_mut()
}

#[no_mangle]
pub unsafe extern "C" fn herald_conversations_get(ptr: *mut Herald) -> *mut Conversations {
    (&mut *ptr).conversations_mut()
}

#[no_mangle]
pub unsafe extern "C" fn herald_errors_get(ptr: *mut Herald) -> *mut Errors {
    (&mut *ptr).errors_mut()
}

#[no_mangle]
pub unsafe extern "C" fn herald_message_search_get(ptr: *mut Herald) -> *mut MessageSearch {
    (&mut *ptr).message_search_mut()
}

#[no_mangle]
pub unsafe extern "C" fn herald_notifications_get(ptr: *mut Herald) -> *mut Notifications {
    (&mut *ptr).notifications_mut()
}

#[no_mangle]
pub unsafe extern "C" fn herald_registration_failure_code_get(ptr: *const Herald) -> COption<u8> {
    match (&*ptr).registration_failure_code() {
        Some(value) => COption {
            data: value,
            some: true,
        },
        None => COption {
            data: u8::default(),
            some: false,
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn herald_users_get(ptr: *mut Herald) -> *mut Users {
    (&mut *ptr).users_mut()
}

#[no_mangle]
pub unsafe extern "C" fn herald_users_search_get(ptr: *mut Herald) -> *mut UsersSearch {
    (&mut *ptr).users_search_mut()
}

#[no_mangle]
pub unsafe extern "C" fn herald_utils_get(ptr: *mut Herald) -> *mut Utils {
    (&mut *ptr).utils_mut()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HeraldPtrBundle {
    herald: *mut HeraldQObject,
    config: *mut ConfigQObject,
    config_config_id_changed: fn(*mut ConfigQObject),
    config_nts_conversation_id_changed: fn(*mut ConfigQObject),
    config_preferred_expiration_changed: fn(*mut ConfigQObject),
    herald_config_init_changed: fn(*mut HeraldQObject),
    herald_connection_pending_changed: fn(*mut HeraldQObject),
    herald_connection_up_changed: fn(*mut HeraldQObject),
    conversation_builder: *mut ConversationBuilderQObject,
    conversation_builder_picture_changed: fn(*mut ConversationBuilderQObject),
    conversation_builder_new_data_ready: fn(*mut ConversationBuilderQObject),
    conversation_builder_layout_about_to_be_changed: fn(*mut ConversationBuilderQObject),
    conversation_builder_layout_changed: fn(*mut ConversationBuilderQObject),
    conversation_builder_data_changed: fn(*mut ConversationBuilderQObject, usize, usize),
    conversation_builder_begin_reset_model: fn(*mut ConversationBuilderQObject),
    conversation_builder_end_reset_model: fn(*mut ConversationBuilderQObject),
    conversation_builder_begin_insert_rows: fn(*mut ConversationBuilderQObject, usize, usize),
    conversation_builder_end_insert_rows: fn(*mut ConversationBuilderQObject),
    conversation_builder_begin_move_rows: fn(*mut ConversationBuilderQObject, usize, usize, usize),
    conversation_builder_end_move_rows: fn(*mut ConversationBuilderQObject),
    conversation_builder_begin_remove_rows: fn(*mut ConversationBuilderQObject, usize, usize),
    conversation_builder_end_remove_rows: fn(*mut ConversationBuilderQObject),
    conversations: *mut ConversationsQObject,
    conversations_builder_conversation_id_changed: fn(*mut ConversationsQObject),
    conversations_filter_changed: fn(*mut ConversationsQObject),
    conversations_filter_regex_changed: fn(*mut ConversationsQObject),
    conversations_new_data_ready: fn(*mut ConversationsQObject),
    conversations_layout_about_to_be_changed: fn(*mut ConversationsQObject),
    conversations_layout_changed: fn(*mut ConversationsQObject),
    conversations_data_changed: fn(*mut ConversationsQObject, usize, usize),
    conversations_begin_reset_model: fn(*mut ConversationsQObject),
    conversations_end_reset_model: fn(*mut ConversationsQObject),
    conversations_begin_insert_rows: fn(*mut ConversationsQObject, usize, usize),
    conversations_end_insert_rows: fn(*mut ConversationsQObject),
    conversations_begin_move_rows: fn(*mut ConversationsQObject, usize, usize, usize),
    conversations_end_move_rows: fn(*mut ConversationsQObject),
    conversations_begin_remove_rows: fn(*mut ConversationsQObject, usize, usize),
    conversations_end_remove_rows: fn(*mut ConversationsQObject),
    errors: *mut ErrorsQObject,
    errors_new_error: fn(*mut ErrorsQObject),
    message_search: *mut MessageSearchQObject,
    message_search_regex_search_changed: fn(*mut MessageSearchQObject),
    message_search_search_pattern_changed: fn(*mut MessageSearchQObject),
    message_search_new_data_ready: fn(*mut MessageSearchQObject),
    message_search_layout_about_to_be_changed: fn(*mut MessageSearchQObject),
    message_search_layout_changed: fn(*mut MessageSearchQObject),
    message_search_data_changed: fn(*mut MessageSearchQObject, usize, usize),
    message_search_begin_reset_model: fn(*mut MessageSearchQObject),
    message_search_end_reset_model: fn(*mut MessageSearchQObject),
    message_search_begin_insert_rows: fn(*mut MessageSearchQObject, usize, usize),
    message_search_end_insert_rows: fn(*mut MessageSearchQObject),
    message_search_begin_move_rows: fn(*mut MessageSearchQObject, usize, usize, usize),
    message_search_end_move_rows: fn(*mut MessageSearchQObject),
    message_search_begin_remove_rows: fn(*mut MessageSearchQObject, usize, usize),
    message_search_end_remove_rows: fn(*mut MessageSearchQObject),
    notifications: *mut NotificationsQObject,
    notifications_notify: fn(*mut NotificationsQObject),
    herald_registration_failure_code_changed: fn(*mut HeraldQObject),
    users: *mut UsersQObject,
    users_filter_changed: fn(*mut UsersQObject),
    users_filter_regex_changed: fn(*mut UsersQObject),
    users_new_data_ready: fn(*mut UsersQObject),
    users_layout_about_to_be_changed: fn(*mut UsersQObject),
    users_layout_changed: fn(*mut UsersQObject),
    users_data_changed: fn(*mut UsersQObject, usize, usize),
    users_begin_reset_model: fn(*mut UsersQObject),
    users_end_reset_model: fn(*mut UsersQObject),
    users_begin_insert_rows: fn(*mut UsersQObject, usize, usize),
    users_end_insert_rows: fn(*mut UsersQObject),
    users_begin_move_rows: fn(*mut UsersQObject, usize, usize, usize),
    users_end_move_rows: fn(*mut UsersQObject),
    users_begin_remove_rows: fn(*mut UsersQObject, usize, usize),
    users_end_remove_rows: fn(*mut UsersQObject),
    users_search: *mut UsersSearchQObject,
    users_search_filter_changed: fn(*mut UsersSearchQObject),
    users_search_new_data_ready: fn(*mut UsersSearchQObject),
    users_search_layout_about_to_be_changed: fn(*mut UsersSearchQObject),
    users_search_layout_changed: fn(*mut UsersSearchQObject),
    users_search_data_changed: fn(*mut UsersSearchQObject, usize, usize),
    users_search_begin_reset_model: fn(*mut UsersSearchQObject),
    users_search_end_reset_model: fn(*mut UsersSearchQObject),
    users_search_begin_insert_rows: fn(*mut UsersSearchQObject, usize, usize),
    users_search_end_insert_rows: fn(*mut UsersSearchQObject),
    users_search_begin_move_rows: fn(*mut UsersSearchQObject, usize, usize, usize),
    users_search_end_move_rows: fn(*mut UsersSearchQObject),
    users_search_begin_remove_rows: fn(*mut UsersSearchQObject, usize, usize),
    users_search_end_remove_rows: fn(*mut UsersSearchQObject),
    utils: *mut UtilsQObject,
    herald_try_poll: fn(*mut HeraldQObject),
}
