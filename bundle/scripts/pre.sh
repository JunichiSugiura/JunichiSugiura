#!/bin/sh 
 
set -e 

file_dir=$(realpath "$0" | dirname "$0")

"$file_dir/macos-defaults.sh"
