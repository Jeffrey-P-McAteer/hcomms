
use sciter;
use include_dir::{
  include_dir, Dir
};
// use notify_rust::{ // good idea, does not work on recent win10 builds
//   Notification, Hint, Timeout
// };


use std::path::{
  Path, PathBuf
};
use std::fs;
use std::env;

const WWW_DATA: Dir = include_dir!("src/www/");

macro_rules! print_err {
  ($expression:expr) => {
    if let Err(e) = $expression {
      eprintln!("{}: {:?}", stringify!($expression), e);
    }
  }
}

pub fn main() {
  hide_console_on_windows();

  load_sciter_lib();

  print_err!( sciter::set_options( sciter::RuntimeOptions::UxTheming(false) ) );
  print_err!( sciter::set_options( sciter::RuntimeOptions::ConnectionTimeout(9000) ) );
  print_err!( sciter::set_options( sciter::RuntimeOptions::DebugMode(true) ) );
  
  let www_dir = load_sciter_www();
  let mut www_index = www_dir.clone();
  www_index.push("index.html");
  let www_index_s = www_index.as_path().to_string_lossy();

  let handler = EHandler { };
  let mut frame = sciter::Window::new();
  print_err!( frame.set_options( sciter::window::Options::MainWindow(true) ) );
  frame.event_handler(handler);
  frame.load_file(&www_index_s);
  frame.run_app();
}


struct EHandler;

impl EHandler {
  fn tell_rust(&self, a: String) -> String {
    println!("a={}", &a);
    "Hello TIS!".to_string()
  }
}

impl sciter::EventHandler for EHandler {
  sciter::dispatch_script_call! {
    fn tell_rust(String);
  }
}




// This fn ensures sciter is on the system path.
// We try to use an OS-specific temp directory
// to extract the libs to, then modify the system PATH.
fn load_sciter_lib() {
  #[cfg(target_os = "windows")]
  load_sciter_lib_win();
  #[cfg(target_os = "macos")]
  load_sciter_lib_mac();
  #[cfg(target_os = "linux")]
  load_sciter_lib_linux();
}

#[cfg(target_os = "windows")]
fn load_sciter_lib_win() {
  let libsciter_tmp_dir = env::var("TEMP").unwrap_or(".".to_string()); // windows guarantees %TEMP% to exist, but if not we use CWD
  let libsciter_tmp_dir = format!("{}\\hcomms_libsciter", libsciter_tmp_dir);
  let libsciter_tmp_dir = Path::new(&libsciter_tmp_dir);
  if !libsciter_tmp_dir.exists() {
    if let Err(e) = fs::create_dir_all(&libsciter_tmp_dir) {
      eprintln!("Error creating {:?}: {}", &libsciter_tmp_dir, e);
      return;
    }
  }

  const LIBSCITER_DATA: Dir = include_dir!("libsciter/bin.win/x64/");
  extract_dir_to(&LIBSCITER_DATA, &libsciter_tmp_dir.to_path_buf());

  // Now update PATH to include libsciter_tmp_dir
  let original_path = env::var("PATH").unwrap_or("".to_string());
  let new_path = format!("{};{}", original_path, libsciter_tmp_dir.to_string_lossy());
  env::set_var("PATH", new_path);
}

#[cfg(target_os = "macos")]
fn load_sciter_lib_mac() {
  let libsciter_tmp_dir = env::var("TMPDIR").unwrap_or(".".to_string());
  let libsciter_tmp_dir = format!("{}/hcomms_libsciter", libsciter_tmp_dir);
  let libsciter_tmp_dir = Path::new(&libsciter_tmp_dir);
  if !libsciter_tmp_dir.exists() {
    if let Err(e) = fs::create_dir_all(&libsciter_tmp_dir) {
      eprintln!("Error creating {:?}: {}", &libsciter_tmp_dir, e);
      return;
    }
  }

  const LIBSCITER_DATA: Dir = include_dir!("libsciter/bin.osx/");
  extract_dir_to(&LIBSCITER_DATA, &libsciter_tmp_dir.to_path_buf());

  // Now update PATH to include libsciter_tmp_dir
  let original_path = env::var("PATH").unwrap_or("".to_string());
  let new_path = format!("{}:{}", original_path, libsciter_tmp_dir.to_string_lossy());
  env::set_var("PATH", new_path);
}

#[cfg(target_os = "linux")]
fn load_sciter_lib_linux() {
  let libsciter_tmp_dir = Path::new("/tmp/hcomms_libsciter/");
  if !libsciter_tmp_dir.exists() {
    if let Err(e) = fs::create_dir_all(&libsciter_tmp_dir) {
      eprintln!("Error creating {:?}: {}", &libsciter_tmp_dir, e);
      return;
    }
  }
  
  const LIBSCITER_DATA: Dir = include_dir!("libsciter/bin.lnx/x64/");
  extract_dir_to(&LIBSCITER_DATA, &libsciter_tmp_dir.to_path_buf());

  // Now update PATH to include libsciter_tmp_dir
  let original_path = env::var("PATH").unwrap_or("".to_string());
  let new_path = format!("{}:{}", original_path, libsciter_tmp_dir.to_string_lossy());
  env::set_var("PATH", new_path);

}


fn load_sciter_www() -> PathBuf {
  #[cfg(target_os = "windows")]
  return load_sciter_www_win();
  #[cfg(target_os = "macos")]
  return load_sciter_www_mac();
  #[cfg(target_os = "linux")]
  return load_sciter_www_linux();
}

fn extract_dir_to(data: &Dir, target: &PathBuf) {
  for file_data in data.files() {
    let out_path = target.join(&file_data.path());
    let mut out_len = 0;
    if let Ok(meta) = fs::metadata(&out_path) {
      out_len = meta.len();
    }
    if ! out_path.exists() || file_data.contents().len() as u64 != out_len {
      if let Err(e) = fs::write(&out_path, file_data.contents()) {
        println!("Error extracting {:?} to {:?}: {}", &file_data.path(), &out_path, e);
      }
    }
  }

  for dir_data in data.dirs() {
    let out_dir = target.join(&dir_data.path());
    if let Err(e) = fs::create_dir_all(&out_dir) {
      eprintln!("Error creating {:?}: {}", &out_dir, e);
    }
    extract_dir_to(dir_data, target);
  }

}

#[cfg(target_os = "windows")]
fn load_sciter_www_win() -> PathBuf {
  let www_tmp_dir = env::var("TEMP").unwrap_or(".".to_string()); // windows guarantees %TEMP% to exist, but if not we use CWD
  let www_tmp_dir = format!("{}\\hcomms_www", www_tmp_dir);
  let sciter_www_dir = PathBuf::from(&www_tmp_dir);
  if !sciter_www_dir.exists() {
    if let Err(e) = fs::create_dir_all(&sciter_www_dir) {
      eprintln!("Error creating {:?}: {}", &sciter_www_dir, e);
      return sciter_www_dir;
    }
  }
  
  extract_dir_to(&WWW_DATA, &sciter_www_dir);

  sciter_www_dir
}
#[cfg(target_os = "macos")]
fn load_sciter_www_mac() -> PathBuf {
  let www_tmp_dir = env::var("TMPDIR").unwrap_or(".".to_string());
  let www_tmp_dir = format!("{}/hcomms_www", www_tmp_dir);
  let sciter_www_dir = PathBuf::from(&www_tmp_dir);
  if !sciter_www_dir.exists() {
    if let Err(e) = fs::create_dir_all(&sciter_www_dir) {
      eprintln!("Error creating {:?}: {}", &sciter_www_dir, e);
      return sciter_www_dir;
    }
  }
  
  extract_dir_to(&WWW_DATA, &sciter_www_dir);

  sciter_www_dir
}

#[cfg(target_os = "linux")]
fn load_sciter_www_linux() -> PathBuf {
  let sciter_www_dir = PathBuf::from("/tmp/hcomms_www/");
  if !sciter_www_dir.exists() {
    if let Err(e) = fs::create_dir_all(&sciter_www_dir) {
      eprintln!("Error creating {:?}: {}", &sciter_www_dir, e);
      return sciter_www_dir;
    }
  }
  
  extract_dir_to(&WWW_DATA, &sciter_www_dir);

  sciter_www_dir
}


// This fn does nothin on linux/unix machines
// and it calls winapi system calls to hide the console
// on windows.
// Users may set the environment variable NO_CONSOLE_DETATCH=1
// to prevent detatching from the console when the GUI is opened.
fn hide_console_on_windows() {
  #[cfg(target_os = "windows")]
  {
    if let Ok(val) = env::var("NO_CONSOLE_DETATCH") {
      if val.contains("y") || val.contains("Y") || val.contains("1") {
        return;
      }
    }
    hide_console_on_windows_win();
  }
}

#[cfg(target_os = "windows")]
fn hide_console_on_windows_win() {
  //use std::ptr;
  //use winapi::um::wincon::GetConsoleWindow;
  //use winapi::um::winuser::{ShowWindow, SW_HIDE};

  // Below always hides console, even when run from cmd.exe
  // let window = unsafe {GetConsoleWindow()};
  // // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
  // if window != ptr::null_mut() {
  //     unsafe {
  //         ShowWindow(window, SW_HIDE);
  //     }
  // }

  // Check if we are run from the console or just launched with explorer.exe
  let mut console_proc_list_buff: Vec<u32> = vec![0; 16];
  let num_procs = unsafe { winapi::um::wincon::GetConsoleProcessList(console_proc_list_buff.as_mut_ptr(), 16) };
  if num_procs == 1 {
    // We were launched from explorer.exe, detatch the console
    unsafe { winapi::um::wincon::FreeConsole() };
  }
  // Otherwise do nothing, we want console messages when run from the console.

}



