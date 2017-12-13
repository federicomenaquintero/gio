// This file was generated by gir (13f739b) from gir-files (469db10)
// DO NOT EDIT

use AppInfo;
use File;
use ffi;
use glib;
#[cfg(any(feature = "v2_36", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_36", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_36", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_36", feature = "dox"))]
use libc;
#[cfg(any(feature = "v2_36", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v2_36", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AppLaunchContext(Object<ffi::GAppLaunchContext, ffi::GAppLaunchContextClass>);

    match fn {
        get_type => || ffi::g_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    pub fn new() -> AppLaunchContext {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_new())
        }
    }
}

impl Default for AppLaunchContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AppLaunchContextExt {
    fn get_display<P: IsA<AppInfo>>(&self, info: &P, files: &[File]) -> Option<String>;

    fn get_environment(&self) -> Vec<String>;

    fn get_startup_notify_id<P: IsA<AppInfo>>(&self, info: &P, files: &[File]) -> Option<String>;

    fn launch_failed(&self, startup_notify_id: &str);

    fn setenv(&self, variable: &str, value: &str);

    fn unsetenv(&self, variable: &str);

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn connect_launch_failed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn connect_launched<F: Fn(&Self, &AppInfo, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppLaunchContext> + IsA<glib::object::Object>> AppLaunchContextExt for O {
    fn get_display<P: IsA<AppInfo>>(&self, info: &P, files: &[File]) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_get_display(self.to_glib_none().0, info.to_glib_none().0, files.to_glib_none().0))
        }
    }

    fn get_environment(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_launch_context_get_environment(self.to_glib_none().0))
        }
    }

    fn get_startup_notify_id<P: IsA<AppInfo>>(&self, info: &P, files: &[File]) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_get_startup_notify_id(self.to_glib_none().0, info.to_glib_none().0, files.to_glib_none().0))
        }
    }

    fn launch_failed(&self, startup_notify_id: &str) {
        unsafe {
            ffi::g_app_launch_context_launch_failed(self.to_glib_none().0, startup_notify_id.to_glib_none().0);
        }
    }

    fn setenv(&self, variable: &str, value: &str) {
        unsafe {
            ffi::g_app_launch_context_setenv(self.to_glib_none().0, variable.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn unsetenv(&self, variable: &str) {
        unsafe {
            ffi::g_app_launch_context_unsetenv(self.to_glib_none().0, variable.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn connect_launch_failed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "launch-failed",
                transmute(launch_failed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn connect_launched<F: Fn(&Self, &AppInfo, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &AppInfo, &glib::Variant) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "launched",
                transmute(launched_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_36", feature = "dox"))]
unsafe extern "C" fn launch_failed_trampoline<P>(this: *mut ffi::GAppLaunchContext, startup_notify_id: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<AppLaunchContext> {
    callback_guard!();
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&AppLaunchContext::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(startup_notify_id))
}

#[cfg(any(feature = "v2_36", feature = "dox"))]
unsafe extern "C" fn launched_trampoline<P>(this: *mut ffi::GAppLaunchContext, info: *mut ffi::GAppInfo, platform_data: *mut glib_ffi::GVariant, f: glib_ffi::gpointer)
where P: IsA<AppLaunchContext> {
    callback_guard!();
    let f: &&(Fn(&P, &AppInfo, &glib::Variant) + 'static) = transmute(f);
    f(&AppLaunchContext::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(info), &from_glib_borrow(platform_data))
}
