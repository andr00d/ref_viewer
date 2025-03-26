# ref viewer
Ref viewer is a simple image viewer using rust, [egui](https://github.com/emilk/egui) and [exiftool](https://exiftool.org/). it has the extra functionality to add tags, links, and artists to images and search folders based on these tags to easily find specific images. 

The program works for both linux and windows, but for now you will still get an extra terminal when opening the program in windows. 

### layout
<img alt="ref viewer screenshot" src="media/ref_viewer.png"> 

### building
For installing on linux, there is the expectation that you have both rust and exiftool already installed. If that is the case you can simply run the included install.sh script to automatically build and install the program

For creating a windows executable, you can use the ref_viewer.iss file with inno setup to create an installer. Make sure the exiftool.exe file and exiftool_files folder are in the same folder as the .iss file. 

### usage
you can right click any jpg, png, gif, webp, or folder, and open it with ref viewer. It will recursively display all images in the folder.

### dependencies
For the linux version, ref viewer requires [exiftool](https://exiftool.org/) to be installed to run. 
For the windows version, ref viewer requires the exif executable and exiftools_files folder to be in the same folder as the ref viewer executable. 