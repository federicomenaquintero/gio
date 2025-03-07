// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use ActionGroup;
use ActionMap;
use ApplicationCommandLine;
use ApplicationFlags;
use Cancellable;
use Error;
use File;
use Notification;

glib_wrapper! {
    pub struct Application(Object<gio_sys::GApplication, gio_sys::GApplicationClass, ApplicationClass>) @implements ActionGroup, ActionMap;

    match fn {
        get_type => || gio_sys::g_application_get_type(),
    }
}

impl Application {
    pub fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Application {
        unsafe {
            from_glib_full(gio_sys::g_application_new(
                application_id.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    pub fn get_default() -> Option<Application> {
        unsafe { from_glib_none(gio_sys::g_application_get_default()) }
    }

    pub fn id_is_valid(application_id: &str) -> bool {
        unsafe {
            from_glib(gio_sys::g_application_id_is_valid(
                application_id.to_glib_none().0,
            ))
        }
    }
}

pub struct ApplicationBuilder {
    action_group: Option<ActionGroup>,
    application_id: Option<String>,
    flags: Option<ApplicationFlags>,
    inactivity_timeout: Option<u32>,
    resource_base_path: Option<String>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self {
            action_group: None,
            application_id: None,
            flags: None,
            inactivity_timeout: None,
            resource_base_path: None,
        }
    }

    pub fn build(self) -> Application {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref action_group) = self.action_group {
            properties.push(("action-group", action_group));
        }
        if let Some(ref application_id) = self.application_id {
            properties.push(("application-id", application_id));
        }
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
        }
        if let Some(ref inactivity_timeout) = self.inactivity_timeout {
            properties.push(("inactivity-timeout", inactivity_timeout));
        }
        if let Some(ref resource_base_path) = self.resource_base_path {
            properties.push(("resource-base-path", resource_base_path));
        }
        glib::Object::new(Application::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn action_group(mut self, action_group: &ActionGroup) -> Self {
        self.action_group = Some(action_group.clone());
        self
    }

    pub fn application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_string());
        self
    }

    pub fn flags(mut self, flags: ApplicationFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn inactivity_timeout(mut self, inactivity_timeout: u32) -> Self {
        self.inactivity_timeout = Some(inactivity_timeout);
        self
    }

    pub fn resource_base_path(mut self, resource_base_path: &str) -> Self {
        self.resource_base_path = Some(resource_base_path.to_string());
        self
    }
}

pub const NONE_APPLICATION: Option<&Application> = None;

pub trait ApplicationExt: 'static {
    fn activate(&self);

    fn add_main_option(
        &self,
        long_name: &str,
        short_name: glib::Char,
        flags: glib::OptionFlags,
        arg: glib::OptionArg,
        description: &str,
        arg_description: Option<&str>,
    );

    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]);

    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn bind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str);

    fn get_application_id(&self) -> Option<GString>;

    //fn get_dbus_connection(&self) -> /*Ignored*/Option<DBusConnection>;

    fn get_dbus_object_path(&self) -> Option<GString>;

    fn get_flags(&self) -> ApplicationFlags;

    fn get_inactivity_timeout(&self) -> u32;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn get_is_busy(&self) -> bool;

    fn get_is_registered(&self) -> bool;

    fn get_is_remote(&self) -> bool;

    fn get_resource_base_path(&self) -> Option<GString>;

    fn hold(&self);

    fn mark_busy(&self);

    fn open(&self, files: &[File], hint: &str);

    fn quit(&self);

    fn register<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error>;

    fn release(&self);

    fn send_notification(&self, id: Option<&str>, notification: &Notification);

    fn set_application_id(&self, application_id: Option<&str>);

    fn set_default(&self);

    fn set_flags(&self, flags: ApplicationFlags);

    fn set_inactivity_timeout(&self, inactivity_timeout: u32);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_description(&self, description: Option<&str>);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_parameter_string(&self, parameter_string: Option<&str>);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_summary(&self, summary: Option<&str>);

    fn set_resource_base_path(&self, resource_path: Option<&str>);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn unbind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str);

    fn unmark_busy(&self);

    fn withdraw_notification(&self, id: &str);

    fn set_property_action_group<P: IsA<ActionGroup> + SetValueOptional>(
        &self,
        action_group: Option<&P>,
    );

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_command_line<F: Fn(&Self, &ApplicationCommandLine) -> i32 + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    //fn connect_handle_local_options<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn connect_name_lost<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_application_id_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inactivity_timeout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_is_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_registered_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resource_base_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Application>> ApplicationExt for O {
    fn activate(&self) {
        unsafe {
            gio_sys::g_application_activate(self.as_ref().to_glib_none().0);
        }
    }

    fn add_main_option(
        &self,
        long_name: &str,
        short_name: glib::Char,
        flags: glib::OptionFlags,
        arg: glib::OptionArg,
        description: &str,
        arg_description: Option<&str>,
    ) {
        unsafe {
            gio_sys::g_application_add_main_option(
                self.as_ref().to_glib_none().0,
                long_name.to_glib_none().0,
                short_name.to_glib(),
                flags.to_glib(),
                arg.to_glib(),
                description.to_glib_none().0,
                arg_description.to_glib_none().0,
            );
        }
    }

    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]) {
    //    unsafe { TODO: call gio_sys:g_application_add_main_option_entries() }
    //}

    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup) {
    //    unsafe { TODO: call gio_sys:g_application_add_option_group() }
    //}

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn bind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str) {
        unsafe {
            gio_sys::g_application_bind_busy_property(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property.to_glib_none().0,
            );
        }
    }

    fn get_application_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_application_get_application_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_dbus_connection(&self) -> /*Ignored*/Option<DBusConnection> {
    //    unsafe { TODO: call gio_sys:g_application_get_dbus_connection() }
    //}

    fn get_dbus_object_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_application_get_dbus_object_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_flags(&self) -> ApplicationFlags {
        unsafe {
            from_glib(gio_sys::g_application_get_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_inactivity_timeout(&self) -> u32 {
        unsafe { gio_sys::g_application_get_inactivity_timeout(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn get_is_busy(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_application_get_is_busy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_registered(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_application_get_is_registered(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_remote(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_application_get_is_remote(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_resource_base_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_application_get_resource_base_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hold(&self) {
        unsafe {
            gio_sys::g_application_hold(self.as_ref().to_glib_none().0);
        }
    }

    fn mark_busy(&self) {
        unsafe {
            gio_sys::g_application_mark_busy(self.as_ref().to_glib_none().0);
        }
    }

    fn open(&self, files: &[File], hint: &str) {
        let n_files = files.len() as i32;
        unsafe {
            gio_sys::g_application_open(
                self.as_ref().to_glib_none().0,
                files.to_glib_none().0,
                n_files,
                hint.to_glib_none().0,
            );
        }
    }

    fn quit(&self) {
        unsafe {
            gio_sys::g_application_quit(self.as_ref().to_glib_none().0);
        }
    }

    fn register<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_application_register(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn release(&self) {
        unsafe {
            gio_sys::g_application_release(self.as_ref().to_glib_none().0);
        }
    }

    fn send_notification(&self, id: Option<&str>, notification: &Notification) {
        unsafe {
            gio_sys::g_application_send_notification(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                notification.to_glib_none().0,
            );
        }
    }

    fn set_application_id(&self, application_id: Option<&str>) {
        unsafe {
            gio_sys::g_application_set_application_id(
                self.as_ref().to_glib_none().0,
                application_id.to_glib_none().0,
            );
        }
    }

    fn set_default(&self) {
        unsafe {
            gio_sys::g_application_set_default(self.as_ref().to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: ApplicationFlags) {
        unsafe {
            gio_sys::g_application_set_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    fn set_inactivity_timeout(&self, inactivity_timeout: u32) {
        unsafe {
            gio_sys::g_application_set_inactivity_timeout(
                self.as_ref().to_glib_none().0,
                inactivity_timeout,
            );
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_description(&self, description: Option<&str>) {
        unsafe {
            gio_sys::g_application_set_option_context_description(
                self.as_ref().to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_parameter_string(&self, parameter_string: Option<&str>) {
        unsafe {
            gio_sys::g_application_set_option_context_parameter_string(
                self.as_ref().to_glib_none().0,
                parameter_string.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_summary(&self, summary: Option<&str>) {
        unsafe {
            gio_sys::g_application_set_option_context_summary(
                self.as_ref().to_glib_none().0,
                summary.to_glib_none().0,
            );
        }
    }

    fn set_resource_base_path(&self, resource_path: Option<&str>) {
        unsafe {
            gio_sys::g_application_set_resource_base_path(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn unbind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str) {
        unsafe {
            gio_sys::g_application_unbind_busy_property(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property.to_glib_none().0,
            );
        }
    }

    fn unmark_busy(&self) {
        unsafe {
            gio_sys::g_application_unmark_busy(self.as_ref().to_glib_none().0);
        }
    }

    fn withdraw_notification(&self, id: &str) {
        unsafe {
            gio_sys::g_application_withdraw_notification(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            );
        }
    }

    fn set_property_action_group<P: IsA<ActionGroup> + SetValueOptional>(
        &self,
        action_group: Option<&P>,
    ) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"action-group\0".as_ptr() as *const _,
                Value::from(action_group).to_glib_none().0,
            );
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_command_line<F: Fn(&Self, &ApplicationCommandLine) -> i32 + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn command_line_trampoline<
            P,
            F: Fn(&P, &ApplicationCommandLine) -> i32 + 'static,
        >(
            this: *mut gio_sys::GApplication,
            command_line: *mut gio_sys::GApplicationCommandLine,
            f: glib_sys::gpointer,
        ) -> libc::c_int
        where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Application::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(command_line),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"command-line\0".as_ptr() as *const _,
                Some(transmute(command_line_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_handle_local_options<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored options: GLib.VariantDict
    //}

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn connect_name_lost<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn name_lost_trampoline<P, F: Fn(&P) -> bool + 'static>(
            this: *mut gio_sys::GApplication,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"name-lost\0".as_ptr() as *const _,
                Some(transmute(name_lost_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn shutdown_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"shutdown\0".as_ptr() as *const _,
                Some(transmute(shutdown_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn startup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"startup\0".as_ptr() as *const _,
                Some(transmute(startup_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_group_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action-group\0".as_ptr() as *const _,
                Some(transmute(
                    notify_action_group_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_application_id_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_application_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::application-id\0".as_ptr() as *const _,
                Some(transmute(
                    notify_application_id_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute(notify_flags_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_inactivity_timeout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inactivity_timeout_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inactivity-timeout\0".as_ptr() as *const _,
                Some(transmute(
                    notify_inactivity_timeout_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_is_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_busy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-busy\0".as_ptr() as *const _,
                Some(transmute(notify_is_busy_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_registered_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_registered_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-registered\0".as_ptr() as *const _,
                Some(transmute(
                    notify_is_registered_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_remote_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-remote\0".as_ptr() as *const _,
                Some(transmute(notify_is_remote_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_resource_base_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resource_base_path_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GApplication,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resource-base-path\0".as_ptr() as *const _,
                Some(transmute(
                    notify_resource_base_path_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Application")
    }
}
