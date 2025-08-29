cfg_if::cfg_if! {
    if #[cfg(not(target_os = "windows"))] {
        use nix::sys::signal;
        use nix::sys::signal::Signal;
        use nix::unistd::Pid;

        pub fn pause_proc(pid: i32) {
            let _ = signal::kill(Pid::from_raw(pid), Signal::SIGSTOP);
        }

        pub fn cont_proc(pid: i32) {
            let _ = signal::kill(Pid::from_raw(pid), Signal::SIGCONT);
        }

        pub fn is_process_effectively_dead(pid: u32) -> bool {
            use sysinfo::{System, Pid, ProcessStatus, ProcessesToUpdate};
            let mut sys = System::new();
            let pid = Pid::from(pid as usize);

            // Update just this PID; returns how many processes were updated.
            let updated = sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
            if updated == 0 {
                return true;
            }

            match sys.process(pid).map(|p| p.status()) {
                Some(ProcessStatus::Zombie) | Some(ProcessStatus::Dead) => true,
                Some(_) => false,
                None => true,
            }
        }
    } else {
        use ntapi::ntpsapi::NtSuspendProcess;
        use ntapi::ntpsapi::NtResumeProcess;

        use winapi::um::winnt::PROCESS_ALL_ACCESS;
        use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::processthreadsapi::GetExitCodeProcess;
        use winapi::um::minwinbase::STILL_ACTIVE;
        use winapi::shared::ntdef::NULL;

        pub fn pause_proc(pid: i32) {
            unsafe {
                let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid as u32);

                if process_handle == NULL {
                    return;
                }

                NtSuspendProcess(process_handle);
            }
        }

        pub fn cont_proc(pid: i32) {
            unsafe {
                let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid as u32);

                if process_handle == NULL {
                    return;
                }

                NtResumeProcess(process_handle);
            }
        }

        pub fn is_process_effectively_dead(pid: u32) -> bool {
            unsafe {
                let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid as u32);
                let mut exit_code = 0;

                // process probably doesnt exist at this point
                if process_handle == NULL {
                    return true;
                }

                if GetExitCodeProcess(process_handle, &mut exit_code) == 0 {
                    return true;
                }

                exit_code != STILL_ACTIVE
            }
        }
    }
}
