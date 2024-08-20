use crate::utilities::Utilities;

use std::ffi::c_void;
use std::ptr::null_mut;
use std::ptr::addr_of_mut;

use windows_sys::Win32::System::Services::SERVICE_STATUS;
use windows_sys::Win32::System::Services::RegisterServiceCtrlHandlerW;
use windows_sys::Win32::System::Services::SetServiceStatus;
use windows_sys::Win32::System::Services::SERVICE_TABLE_ENTRYW;
use windows_sys::Win32::System::Services::StartServiceCtrlDispatcherW;

static mut _SERVICE_STATUS_HANDLE: *mut c_void = null_mut();

static mut _SERVICE_STATUS: SERVICE_STATUS = SERVICE_STATUS {
    dwServiceType: 0x00000010,
    dwCurrentState: 0x00000002,
    dwControlsAccepted: 0,
    dwWin32ExitCode: 259,
    dwServiceSpecificExitCode: 0,
    dwCheckPoint: 0,
    dwWaitHint: 0,
};

pub struct RuntimeSvc;

impl RuntimeSvc {
    pub fn new(service_name: String) {
        // Starts a new Service with the name of the service_name parameter
        unsafe {
            let service_name = Utilities::pointer_to_constant_wide(&service_name);
            let service_table = [
                SERVICE_TABLE_ENTRYW {
                    lpServiceName: service_name.as_ptr() as *mut u16,
                    lpServiceProc: Some(service_main),
                },
            ];

            StartServiceCtrlDispatcherW(service_table.as_ptr());
        }
    }
}

unsafe extern "system" fn service_control_handler(control: u32) {
    match control {
        1 => unsafe {
            _SERVICE_STATUS.dwCurrentState = 0x00000001; // SERVICE_STOPPED
            _SERVICE_STATUS.dwWin32ExitCode = 0;
            _SERVICE_STATUS.dwServiceSpecificExitCode = 0;
            _SERVICE_STATUS.dwCheckPoint = 0;
            _SERVICE_STATUS.dwWaitHint = 0;

            SetServiceStatus(_SERVICE_STATUS_HANDLE, addr_of_mut!(_SERVICE_STATUS));

            return;
        },
        _ => (),
    }
}

unsafe extern "system" fn service_main(_: u32, _: *mut *mut u16) {

    // Starting a Service Control Handler so our service keeps running
    if let Ok(exe_path) = Utilities::get_executable_path() {
        _SERVICE_STATUS_HANDLE = RegisterServiceCtrlHandlerW(
            Utilities::pointer_to_constant_wide(&exe_path.to_string()).as_ptr(),
            Some(service_control_handler),
        );

        if _SERVICE_STATUS_HANDLE == null_mut() {
            // If the Service Control Handler returns as failed, no service will run.
            return;
        }

        _SERVICE_STATUS.dwCurrentState = 0x00000004; // SERVICE_RUNNING
        _SERVICE_STATUS.dwCheckPoint = 0;            // Not used
        _SERVICE_STATUS.dwWaitHint = 0;              // Estimated time to start/stop

        SetServiceStatus(_SERVICE_STATUS_HANDLE, addr_of_mut!(_SERVICE_STATUS));

        // Still requires the main loop for the program
        // This can be taken from the RuntimeExe
        // RuntimeExe::new()
    }
    // Couldn't retrieve our own executable path
    return;
}