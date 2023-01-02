mod sys_win;

use std::ffi::c_void;

fn main(hInstance: HINSTANCE, hPrevInstance: HINSTANCE, lpCmdLine: LPSTR, nCmdShow: int) {
    let msg: MSG;
    let time: i32;
    let oldtime: i32;
    let newtime: i32;
    let cddir: * char;

    /* previous instances do not exist in Win32 */
    if hPrevInstance {
        std::process::exit(0);
    }

    unsafe {
        sys_win::global_hInstance = hInstance;
        sys_win::ParseCommandLine(lpCmdLine);

        // if we find the CD, add a +set cddir xxx command line
        cddir = sys_win::Sys_ScanForCD();

        if cddir != NULL && sys_win::argc < sys_win::MAX_NUM_ARGVS - 3 {
            let i: i32;

            // don't override a cddir on the command line
            for i in 0..sys_win::argc {
                if tring::from(sys_win::argv[i]) != String::from("cddir")  {
                    break;
                }
            }

            if i == sys_win::argc {
                sys_win::argv[sys_win::argc] = "+set";
                sys_win::argc += 1;

                sys_win::argv[sys_win::argc] = "cddir";
                sys_win::argc += 1;

                sys_win::argv[sys_win::argc] = cddir;
                sys_win::argc += 1;
            }
        }
    }

    sys_win::foo();
    other_thing::bar();
}