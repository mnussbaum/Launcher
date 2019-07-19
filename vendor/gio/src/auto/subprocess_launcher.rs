// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use Subprocess;
use SubprocessFlags;
use ffi;
use glib::translate::*;
use std;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct SubprocessLauncher(Object<ffi::GSubprocessLauncher, SubprocessLauncherClass>);

    match fn {
        get_type => || ffi::g_subprocess_launcher_get_type(),
    }
}

impl SubprocessLauncher {
    pub fn new(flags: SubprocessFlags) -> SubprocessLauncher {
        unsafe {
            from_glib_full(ffi::g_subprocess_launcher_new(flags.to_glib()))
        }
    }

    pub fn getenv<P: AsRef<std::path::Path>>(&self, variable: P) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_subprocess_launcher_getenv(self.to_glib_none().0, variable.as_ref().to_glib_none().0))
        }
    }

    //#[cfg(any(unix, feature = "dox"))]
    //pub fn set_child_setup(&self, child_setup: /*Ignored*/glib::Fn() + 'static, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::g_subprocess_launcher_set_child_setup() }
    //}

    pub fn set_cwd<P: AsRef<std::path::Path>>(&self, cwd: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_cwd(self.to_glib_none().0, cwd.as_ref().to_glib_none().0);
        }
    }

    pub fn set_environ(&self, env: &[&std::path::Path]) {
        unsafe {
            ffi::g_subprocess_launcher_set_environ(self.to_glib_none().0, env.to_glib_none().0);
        }
    }

    pub fn set_flags(&self, flags: SubprocessFlags) {
        unsafe {
            ffi::g_subprocess_launcher_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn set_stderr_file_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_stderr_file_path(self.to_glib_none().0, path.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn set_stdin_file_path(&self, path: &str) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdin_file_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn set_stdout_file_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdout_file_path(self.to_glib_none().0, path.as_ref().to_glib_none().0);
        }
    }

    pub fn setenv<P: AsRef<std::ffi::OsStr>, Q: AsRef<std::ffi::OsStr>>(&self, variable: P, value: Q, overwrite: bool) {
        unsafe {
            ffi::g_subprocess_launcher_setenv(self.to_glib_none().0, variable.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0, overwrite.to_glib());
        }
    }

    //pub fn spawn(&self, error: &mut Error, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Subprocess> {
    //    unsafe { TODO: call ffi::g_subprocess_launcher_spawn() }
    //}

    pub fn spawnv(&self, argv: &[&std::ffi::OsStr]) -> Result<Subprocess, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_subprocess_launcher_spawnv(self.to_glib_none().0, argv.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn unsetenv<P: AsRef<std::ffi::OsStr>>(&self, variable: P) {
        unsafe {
            ffi::g_subprocess_launcher_unsetenv(self.to_glib_none().0, variable.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for SubprocessLauncher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SubprocessLauncher")
    }
}