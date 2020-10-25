
use sciter;
use include_dir::{include_dir, Dir};

use std::path::{
  Path, PathBuf
};
use std::fs;
use std::env;

pub fn main() {
  load_sciter_lib();
  
  let www_dir = load_sciter_www();
  let mut www_index = www_dir.clone();
  www_index.push("index.html");
  let www_index_s = www_index.as_path().to_string_lossy();

  let mut frame = sciter::Window::new();
  frame.load_file(&www_index_s);
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


fn load_sciter_www() -> PathBuf {
  #[cfg(target_os = "windows")]
  return load_sciter_www_win();
  #[cfg(target_os = "macos")]
  return load_sciter_www_mac();
  #[cfg(target_os = "linux")]
  return load_sciter_www_linux();
}

fn load_sciter_www_win() -> PathBuf {
  let www_tmp_dir = env::var("TEMP").unwrap_or(".".to_string()); // windows guarantees %TEMP% to exist, but if not we use CWD
  let www_tmp_dir = format!("{}\\hcomms_www", www_tmp_dir);
  let sciter_www_dir = PathBuf::from(&www_tmp_dir);
  if !sciter_www_dir.exists() {
    fs::create_dir_all(&sciter_www_dir);
  }
  
  const www_data: Dir = include_dir!("src/www/");
  
  for file_data in www_data.files() {
    let out_path = sciter_www_dir.join(&file_data.path());
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

  sciter_www_dir
}
fn load_sciter_www_mac() -> PathBuf {
  std::unimplemented!()
}
fn load_sciter_www_linux() -> PathBuf {
  let sciter_www_dir = PathBuf::from("/tmp/hcomms_www/");
  if !sciter_www_dir.exists() {
    fs::create_dir_all(&sciter_www_dir);
  }
  
  const www_data: Dir = include_dir!("src/www/");
  
  for file_data in www_data.files() {
    let out_path = sciter_www_dir.join(&file_data.path());
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

  sciter_www_dir
}




