
use sciter;
use include_dir::{include_dir, Dir};

use std::path::{
  Path
};
use std::fs;
use std::env;

pub fn main() {
  load_sciter_lib();
  let mut frame = sciter::Window::new();
  frame.load_file("minimal.htm");
  frame.run_app();
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
    fs::create_dir_all(&libsciter_tmp_dir);
  }

  const libsciter_data: Dir = include_dir!("libsciter/bin.win/x64/");
  
  for file_data in libsciter_data.files() {
    let out_path = libsciter_tmp_dir.join(&file_data.path());
    if ! out_path.exists() {
      if let Err(e) = fs::write(&out_path, file_data.contents()) {
        println!("Error extracting {:?} to {:?}: {}", &file_data.path(), &out_path, e);
      }
    }
  }

  // Now update PATH to include libsciter_tmp_dir
  let original_path = env::var("PATH").unwrap_or("".to_string());
  let new_path = format!("{};{}", original_path, libsciter_tmp_dir.to_string_lossy());
  env::set_var("PATH", new_path);
}

#[cfg(target_os = "macos")]
fn load_sciter_lib_mac() {
  std::unimplemented!()
}

#[cfg(target_os = "linux")]
fn load_sciter_lib_linux() {
  let libsciter_tmp_dir = Path::new("/tmp/hcomms_libsciter/");
  if !libsciter_tmp_dir.exists() {
    fs::create_dir_all(&libsciter_tmp_dir);
  }
  
  const libsciter_data: Dir = include_dir!("libsciter/bin.lnx/x64/");
  
  for file_data in libsciter_data.files() {
    let out_path = libsciter_tmp_dir.join(&file_data.path());
    if ! out_path.exists() {
      if let Err(e) = fs::write(&out_path, file_data.contents()) {
        println!("Error extracting {:?} to {:?}: {}", &file_data.path(), &out_path, e);
      }
    }
  }

  // Now update PATH to include libsciter_tmp_dir
  let original_path = env::var("PATH").unwrap_or("".to_string());
  let new_path = format!("{}:{}", original_path, libsciter_tmp_dir.to_string_lossy());
  env::set_var("PATH", new_path);

}

