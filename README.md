# 3DLC_CustomCheckerReader
Python script, that creates RGB image of each color from 3D LUT Creator custom checker. Unique mode: output is 3 images of each channel with each color (RGB). Filled mode: funny mode that creates amazing pictures filled with colors from checker. Script takes lines between BEGIN_DATA/END_DATA lines and interprets RGB_R, RGB_G and RGB_B columns as 16bit RGB array. Script takes the R/G/B values and creates grayscale images for each value using all colors from array. It creates a lotta squares with your colors. One thing you should know that this script in one mode flipping or mirroring image. Enjoy saving a lotta time!

# Running

You can run .py script on any platform of build executable for yours.

For e.x. on mac I ran:
```bash
pyinstaller --onefile 3DLC_CCR.py
```
On Win it probably the same

Don't forget about Qt and PIL :)
```bash
pip install pyqt5
pip install pillow
```

# colors.txt example
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
# P.S.
I know that code looks like shit. This is just my useful Python doodles with just copying and pasting ChatGPT algorithms with few corrections. :)

# ChangeLog
## Waypoints
Fix flip/mirror bug.\
Better GUI.\
Merge grayscales together.\
Make Nuke Addon.
## 1.1
Major improvements in code readability
## 1.0 
First upload
