from PyQt5 import QtCore, QtGui, QtWidgets
from PIL import Image
import os
from colormath.color_objects import LabColor, sRGBColor
from colormath.color_conversions import convert_color

class Ui_QGenerate(object):
    def setupUi(self, QGenerate):
        QGenerate.setObjectName("QGenerate")
        QGenerate.resize(410, 232)
        QGenerate.setMinimumSize(QtCore.QSize(410, 232))
        QGenerate.setMaximumSize(QtCore.QSize(410, 232))
        QGenerate.setWindowOpacity(0.95)
        QGenerate.setAnimated(True)
        QGenerate.setTabShape(QtWidgets.QTabWidget.Rounded)
        QGenerate.setUnifiedTitleAndToolBarOnMac(False)
        self.centralwidget = QtWidgets.QWidget(QGenerate)
        self.centralwidget.setObjectName("centralwidget")
        self.Generate = QtWidgets.QPushButton(self.centralwidget)
        self.Generate.setGeometry(QtCore.QRect(20, 100, 181, 71))
        self.Generate.setMaximumSize(QtCore.QSize(550, 71))
        font = QtGui.QFont()
        font.setFamily("Teko")
        font.setPointSize(24)
        self.Generate.setFont(font)
        self.Generate.setStyleSheet("")
        self.Generate.setAutoDefault(False)
        self.Generate.setDefault(False)
        self.Generate.setFlat(False)
        self.Generate.setObjectName("Generate")
        self.width_in = QtWidgets.QTextEdit(self.centralwidget)
        self.width_in.setGeometry(QtCore.QRect(20, 60, 111, 31))
        self.width_in.setObjectName("width_in")
        self.w = QtWidgets.QLabel(self.centralwidget)
        self.w.setGeometry(QtCore.QRect(20, 20, 111, 41))
        font = QtGui.QFont()
        font.setFamily("Teko")
        font.setPointSize(24)
        self.w.setFont(font)
        self.w.setObjectName("w")
        self.h = QtWidgets.QLabel(self.centralwidget)
        self.h.setGeometry(QtCore.QRect(150, 20, 111, 41))
        font = QtGui.QFont()
        font.setFamily("Teko")
        font.setPointSize(24)
        self.h.setFont(font)
        self.h.setObjectName("h")
        self.height_in = QtWidgets.QTextEdit(self.centralwidget)
        self.height_in.setGeometry(QtCore.QRect(150, 60, 111, 31))
        self.height_in.setObjectName("height_in")
        self.ss = QtWidgets.QLabel(self.centralwidget)
        self.ss.setGeometry(QtCore.QRect(280, 20, 111, 41))
        font = QtGui.QFont()
        font.setFamily("Teko")
        font.setPointSize(24)
        self.ss.setFont(font)
        self.ss.setObjectName("ss")
        self.size_in = QtWidgets.QTextEdit(self.centralwidget)
        self.size_in.setGeometry(QtCore.QRect(280, 60, 111, 31))
        self.size_in.setObjectName("size_in")
        self.console = QtWidgets.QLabel(self.centralwidget)
        self.console.setGeometry(QtCore.QRect(20, 180, 371, 41))
        self.console.setObjectName("console")
        self.GenerateF = QtWidgets.QPushButton(self.centralwidget)
        self.GenerateF.setGeometry(QtCore.QRect(210, 100, 181, 71))
        self.GenerateF.setMaximumSize(QtCore.QSize(550, 71))
        font = QtGui.QFont()
        font.setFamily("Teko")
        font.setPointSize(24)
        self.GenerateF.setFont(font)
        self.GenerateF.setStyleSheet("")
        self.GenerateF.setAutoDefault(False)
        self.GenerateF.setDefault(False)
        self.GenerateF.setFlat(False)
        self.GenerateF.setObjectName("GenerateF")
        QGenerate.setCentralWidget(self.centralwidget)
        self.retranslateUi(QGenerate)
        QtCore.QMetaObject.connectSlotsByName(QGenerate)

        self.AddFunctions()


    def retranslateUi(self, QGenerate):
        _translate = QtCore.QCoreApplication.translate
        QGenerate.setWindowTitle(_translate("QGenerate", "TIFF"))
        self.Generate.setText(_translate("QGenerate", "Generate unique"))
        self.w.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">Width</p></body></html>"))
        self.h.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">Height</p></body></html>"))
        self.ss.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">Square size</p></body></html>"))
        self.console.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">To generate a picture, the colors.txt file<br/> must be located in the same directory as the script.</p></body></html>"))
        self.GenerateF.setText(_translate("QGenerate", "Generate filled"))


    square_size = 0
    width = 0
    height = 0


    def AddFunctions(self):
        self.Generate.clicked.connect(lambda: self.Generate_Unique(self.ReadRGB(), 0, "output_R.tiff"))
        self.Generate.clicked.connect(lambda: self.Generate_Unique(self.ReadRGB(), 1, "output_G.tiff"))
        self.Generate.clicked.connect(lambda: self.Generate_Unique(self.ReadRGB(), 2, "output_B.tiff"))
        self.GenerateF.clicked.connect(lambda: self.Generate_Filled(self.ReadRGB(), 0, "output_R.tiff"))
        self.GenerateF.clicked.connect(lambda: self.Generate_Filled(self.ReadRGB(), 1, "output_G.tiff"))
        self.GenerateF.clicked.connect(lambda: self.Generate_Filled(self.ReadRGB(), 2, "output_B.tiff"))
        self.height_in.textChanged.connect(lambda: self.ReadValues("height"))
        self.width_in.textChanged.connect(lambda: self.ReadValues("width"))
        self.size_in.textChanged.connect(lambda: self.ReadValues("size"))


    def ReadValues(self, what):
        try:
            if what == "width":
                self.width = int(self.width_in.toPlainText())
            elif what == "height":
                self.height = int(self.height_in.toPlainText())
            elif what == "size":
                self.square_size = int(self.size_in.toPlainText())
        except:
            return


    def Generate_Filled(self, rgb_array, kkz, filename):
        image_width = self.width
        image_height = self.height
        folder_path = "Filled"
        os.makedirs(folder_path, exist_ok=True)
        num_squares_x = image_width // self.square_size
        num_squares_y = image_height // self.square_size
        try:
            image = Image.new('I;16', (image_width, image_height), color=0)
            x = 0
            y = 0
            for pixel in range(num_squares_x * num_squares_y):
                color = rgb_array[pixel % len(rgb_array)]
                for i in range(self.square_size):
                    for j in range(self.square_size):
                        image.putpixel((x + i, y + j), int(color[kkz] * 65536))
                x += self.square_size
                if x >= image_width:
                    x = 0
                    y += self.square_size
                if y >= image_height:
                    break
            file_path = os.path.join(folder_path, filename)
            image.save(file_path, format='TIFF', compression='tiff_deflate')
            _translate = QtCore.QCoreApplication.translate
            self.console.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">Succeed!</p></body></html>"))
        except:
            _translate = QtCore.QCoreApplication.translate
            self.console.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">The width value must be divisible<br/> without remainder by the square size</p></body></html>"))
        

    def Generate_Unique(self, rgb_array, kkz, filename):
        image_width = self.width
        image_height = self.height
        num_squares_x = image_width // self.square_size
        num_squares_y = image_height // self.square_size
        folder_path = "Unique"
        os.makedirs(folder_path, exist_ok=True)
        image = Image.new('I;16', (image_width, image_height), color=0)
        try:
            for i, rgb in enumerate(rgb_array):
                red = int(rgb[kkz] * 65536)
                y = num_squares_y - 1 - (i // num_squares_x)
                x = i % num_squares_x
                square_x = x * self.square_size
                square_y = y * self.square_size
                for i in range(self.square_size):
                    for j in range(self.square_size):
                        image.putpixel((square_x + i, square_y + j), red)
            file_path = os.path.join(folder_path, filename)
            image.save(file_path, format='TIFF', compression='tiff_deflate')
            _translate = QtCore.QCoreApplication.translate
            self.console.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">Succeed!</p></body></html>"))
        except:
            _translate = QtCore.QCoreApplication.translate
            self.console.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">Something went wrong.<br/>Probably number of colors is under number of squares.</p></body></html>"))


    def ReadRGB(self):
        try:
            with open('colors.txt', 'r') as f: 
                lines = f.readlines() 
                begin_data_index = lines.index('BEGIN_DATA\n') 
                end_data_index = lines.index('END_DATA\n') 
                data_lines = lines[begin_data_index + 1:end_data_index] 
                rgb_array = [] 
                for line in data_lines: 
                    values = line.strip().split('\t') 
                    lab_l, lab_a, lab_b = float(values[5]), float(values[6]), float(values[7]) 
                    lab_color = LabColor(lab_l, lab_a, lab_b) 
                    rgb_color = convert_color(lab_color, sRGBColor) 
                    rgb_array.append(list(rgb_color.get_value_tuple())) 
                return rgb_array 
        except:
            _translate = QtCore.QCoreApplication.translate
            self.console.setText(_translate("QGenerate", "<html><head/><body><p align=\"center\">Don't forget about colors.txt<br/>If colors.txt exists: check </p></body></html>"))


if __name__ == "__main__":
    import sys
    app = QtWidgets.QApplication(sys.argv)
    QGenerate = QtWidgets.QMainWindow()
    ui = Ui_QGenerate()
    ui.setupUi(QGenerate)
    QGenerate.show()
    sys.exit(app.exec_())
