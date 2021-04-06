fn main() {
    windows::build!(
        windows::win32::system_services::{
            VirtualProtect, GetProcAddress, LoadLibraryA, 
            GetModuleHandleA, HINSTANCE, LRESULT, CW_USEDEFAULT, GWLP_ID,
        },
        windows::win32::windows_and_messaging::{
            RegisterClassExA, WNDCLASSEXA, DefWindowProcA,
            WPARAM, LPARAM, HWND, CreateWindowExA, DestroyWindow,
            SetWindowLongA,
        },
        windows::win32::menus_and_resources::{
            HMENU, CreateMenu,
        },
        windows::win32::debug::{
            SetLastError, GetLastError,
        },
        //windows::win32::display_devices::RECT
    );
}