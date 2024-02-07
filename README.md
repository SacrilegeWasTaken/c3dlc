# 3DLC_CustomCheckerConverter

Rust application, that creates RGB image of each color from 3D LUT Creator custom checker. Generate unique: output is image with each color (RGB) from the source file. Generate filled: funny thing that creates amazing pictures filled with colors from checker. App takes lines between BEGIN_DATA/END_DATA lines and covert LAB_L, LAB_A and LAB_B columns to 32-bit RGB array. App takes the RGB values and creates PNG(for 16-bit mode) or OpenEXR(for 32-bit mode) images for each value using all colors from array. Also you can edit c3ldc/source/* files with built-in text editor.  

Enjoy saving a lotta time!

# Running
## Downloads

Download [executable](https://github.com/SacrilegeWasTaken/3DLC_CustomCheckerConverter/releases/tag/2.2.release) for Apple Silicon Mac or x86_64 Windows.  
If you're on linux -> build app with cargo. Short instruction located below.

## User guide

- Run the executable. It will instantly create c3dlc directory in the same folder.
- Put your text files (you can drop .cie directly) in the c3ldc/source folder.
- Open file in editor by clicking one of the file buttons.
- Select mode by clicking on the "Toggle mode" button.
- Enter width, height and box size.
- Click one of generate buttons and check your c3dlc/pics folder for result!

## To complile program for your system

- Download rust.
- Clone repository.
- Open terminal in the repository directory.
- Run the command `cargo build --release`.
- Check your target folder for application.

# c3dlc/source/* example
```txt
NUMBER_OF_SETS	96
LGOROWLENGTH	12
INFO	"sRGB"
INSTRUMENTATION	"3DLUTCreator"
BEGIN_DATA_FORMAT
SampleID	SAMPLE_NAME	RGB_R	RGB_G	RGB_B	LAB_L	LAB_A	LAB_B
END_DATA_FORMAT
BEGIN_DATA
1	A1	0.41	0.27	0.35	34.237	18.41	-5.74
2	A2	0.46	0.42	0.47	46.826	6.13	-5.95
3	A3	0.38	0.32	0.44	37.147	13.48	-15.86
4	A4	0.38	0.45	0.53	47.798	-1.17	-13.57
5	A5	0.41	0.47	0.50	49.707	-4.63	-5.9M
6	A6	0.19	0.24	0.26	24.569	-2.92	-5.20
7	A7	0.40	0.48	0.48	49.678	-7.29	-2.45
8	A8	0.22	0.22	0.21	23.135	0.21	0.77
9	B1	0.27	0.25	0.32	27.828	6.62	-10.66
10	B2	0.37	0.37	0.47	40.765	5.80	-14.06
END_DATA
```


# ChangeLog
## Known bugs
- None
## Waypoints
- Comments in source file.
- Make Nuke Addon.
- In-GUI picture preview.
## 2.2
- Renaming executable error fixed.
- color.txt is not accessible if it's not in the source folder at the startup moment bug fixed.
- Minor performance improvements.
## 2.11
- Major improvements in code readability.
- Fixed generated folder name on windows.
## 2.1
- Fixed box size equals zero crash.
- Now generating color boxes from bottom left corner.
- Now generating doesn't make UI freeze. Generating now in separate thread.
- Major improvements in code readability.
## 2.0
- Now it's pure Rust!
- Added 32-bit mode powered by OpenEXR.
- Now creating with merged channels (1 file instead of 3).
- Now have intergrated text editor to proccess multiple color files.
- Better GUI. Now looks the same on all systems.
## 1.3
- Fixed bug that cold colors were a bit different. Changed illuminant from d50 to d65.
- Better GUI (may be some errors with text on Win).
## 1.2
- Fixed bug that colors were a little bit different.
- Now taking LAB values from colors.txt and converting it to RGB.
## 1.1
- Major improvements in code readability.
## 1.0 
- First upload.

# Old python version
Old python version still in repository in the legacy folder. But it will never be updated by myself.
If you want to support python version text me: t.me/vietnam_veteran

