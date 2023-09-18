use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HANDLE, HWND},
        UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
    },
};

#[no_mangle]
extern "C" fn do_stuff() {
    unsafe {
        MessageBoxA(
            HWND(0),
            PCSTR("Doing stuff!\x00".as_ptr()),
            PCSTR("Hello!\x00".as_ptr()),
            MESSAGEBOX_STYLE(0),
        );
    }
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DLLMain(dll_module: HANDLE, call_reason: u32, lpv_reserved: u32) -> BOOL {
    match call_reason {
        _ => {
            do_stuff();
            BOOL(0)
        }
    }
}
