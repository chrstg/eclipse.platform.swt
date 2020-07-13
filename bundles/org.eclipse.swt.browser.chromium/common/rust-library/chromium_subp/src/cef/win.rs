/********************************************************************************
 * Copyright (c) 2020 Equo
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0.
 *
 * SPDX-License-Identifier: EPL-2.0
 *
 * Contributors:
 *   Guillermo Zunino, Equo - initial implementation
 ********************************************************************************/
 /* automatically generated by rust-bindgen */

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
use cef::cef_string_t;

pub type DWORD = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HINSTANCE__ {
    pub unused: ::std::os::raw::c_int,
}
pub type HINSTANCE = *mut HINSTANCE__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HWND__ {
    pub unused: ::std::os::raw::c_int,
}
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HMENU__ {
    pub unused: ::std::os::raw::c_int,
}
pub type HMENU = *mut HMENU__;
///
/// Structure representing CefExecuteProcess arguments.
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cef_main_args_t {
    pub instance: HINSTANCE,
}
///
/// Structure representing window information.
///
#[repr(C)]
pub struct _cef_window_info_t {
    /// Standard parameters required by CreateWindowEx()
    pub ex_style: DWORD,
    pub window_name: cef_string_t,
    pub style: DWORD,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub parent_window: HWND,
    pub menu: HMENU,
    ///
    /// Set to true (1) to create the browser using windowless (off-screen)
    /// rendering. No window will be created for the browser and all rendering will
    /// occur via the CefRenderHandler interface. The |parent_window| value will be
    /// used to identify monitor info and to act as the parent window for dialogs,
    /// context menus, etc. If |parent_window| is not provided then the main screen
    /// monitor will be used and some functionality that requires a parent window
    /// may not function correctly. In order to create windowless browsers the
    /// CefSettings.windowless_rendering_enabled value must be set to true.
    /// Transparent painting is enabled by default but can be disabled by setting
    /// CefBrowserSettings.background_color to an opaque value.
    ///
    pub windowless_rendering_enabled: ::std::os::raw::c_int,
    ///
    /// Handle for the new browser window. Only used with windowed rendering.
    ///
    pub window: HWND,
}
