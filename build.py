#!/usr/bin/env python3

# This file runs a complete build
# (cross-compiled for windows, macos, and linux)
# and deploys to github releases if run on a developer's
# machine.

# Run this using python3 by executing:
#   python -m build
# or
#   python build.py


import os, sys, subprocess

if sys.version_info[0] < 3:
  raise Exception("Must be using Python 3")

# python3 -m pip install --user requests
import requests, zipfile, io
import shutil

def cmd(*args):
  subprocess.run([
    x for x in args
  ], check=True)

def ensure_libsciter_exists():
  lib_dir = os.path.abspath('libsciter')
  if not os.path.exists(lib_dir):
    zip_url = 'https://sciter.com/sdk/sciter-sdk.zip'
    print('downloading {}'.format(zip_url))
    zip_r = requests.get(zip_url)
    zip_mem = zipfile.ZipFile(io.BytesIO(zip_r.content))
    os.makedirs(lib_dir)
    print('extracting libsciter to {}'.format(lib_dir))
    zip_mem.extractall(lib_dir)



def main():
  # CD to script's directory
  os.chdir( os.path.dirname(os.path.abspath(__file__)) )

  ensure_libsciter_exists()

  if 'azure-angel' in os.uname()[1]:
    if 'cross' in sys.argv:
      cmd('cargo', 'build', '--release', '--target=x86_64-unknown-linux-gnu')

      cmd('cargo', 'build', '--release', '--target=x86_64-pc-windows-gnu')

      # if not 'LIBRARY_PATH' in os.environ:
      #   os.environ['LIBRARY_PATH'] = ''
      # os.environ['LIBRARY_PATH'] += ':/opt/osxcross/lib/'
      # os.environ['RUSTFLAGS'] = '-C link-args=-L/opt/osxcross/lib/'
      # cmd('cargo', 'build', '--release', '--target=x86_64-apple-darwin')
      print('^^ TODO fix macos cross-compile')

    else:
      cmd('cargo', 'build', '--release', '--target=x86_64-unknown-linux-gnu')

    if 'run' in sys.argv:
      cmd('cargo', 'run', '--release', '--target=x86_64-unknown-linux-gnu', '--')

    if 'deploy' in sys.argv:
      shutil.copy('target/x86_64-pc-windows-gnu/release/hcomms.exe', '/j/downloads/')
      shutil.copy('target/x86_64-unknown-linux-gnu/release/hcomms', '/j/downloads/')

      shutil.copy('target/x86_64-pc-windows-gnu/release/hcomms.exe', '/j/pub/')
      shutil.copy('target/x86_64-unknown-linux-gnu/release/hcomms', '/j/pub/')

  else:
    # Do boring build for host OS, binaries go
    # to target/release/hcomms[.exe]
    cmd('cargo', 'build', '--release')


if __name__ == '__main__':
  main()

