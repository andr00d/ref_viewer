#!/bin/bash

echo "checking dependencies..."

if ! command -v exiftool &> /dev/null
then
    echo "exiftool is not installed. you can download it at https://exiftool.org/install.html"
    echo "exiting...."
    exit 1
fi

if ! command -v cargo &> /dev/null
then
    echo "rust is not installed. you can download it at https://www.rust-lang.org/tools/install"
    echo "exiting...."
    exit 1
fi

echo "building program..."

if ! cargo build --release
then
    echo "build failed."
    echo "exiting...."
    exit 1
fi

echo "installing program..."

if ! install -CDt $HOME/.local/bin/ ./target/release/ref_viewer ; 
then
    echo "Installation failed"
    echo "exiting...."
    exit 1
fi

app_dir=$HOME/.local/share/applications

if [ ! -f $app_dir/ref_viewer.desktop ]; 
then
    rm $app_dir/ref_viewer.desktop
fi

echo "
[Desktop Entry]
Type=Application
Name=ref viewer
Exec=$HOME/.local/bin/ref_viewer
MimeType=inode/directory;image/gif;image/jpg;image/jpeg;image/png;image/webp
Categories=Graphics;
" > $app_dir/ref_viewer.desktop 

update-desktop-database $app_dir

echo "program installed."