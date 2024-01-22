# 3DLC_CustomCheckerReader



# Running



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
## Waypoints
- Fix flip/mirror bug.
- Comments in source file.
- Make Nuke Addon.
- In-GUI picture preview.
- Comments in colors.txt
## 2.0
- Now it's pure Rust!
- Added 32-bit mode powered by OpenEXR
- Now creating with merged channels (1 file instead of 3)
- Now have intergrated text editor to proccess multiple color files.
## 1.3
- Fixed bug that cold colors were a bit different. Changed illuminant from d50 to d65.
- Better GUI (may be some errors with text on Win).
## 1.2
- Fixed bug that colors were a little bit different.
- Now taking LAB values from colors.txt and converting it to RGB.
## 1.1
- Major improvements in code readability
## 1.0 
- First upload

# Old python version
Old python version still in repository in the legacy folder. But it will never be updated by myself.
If you want to support python version text me: t.me/vietnam_veteran

