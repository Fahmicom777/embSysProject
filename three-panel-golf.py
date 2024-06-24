#!/usr/bin/env python
from samplebase import SampleBase
from rgbmatrix import graphics
import time
import json

class GrayscaleBlock(SampleBase):
    
    
    def __init__(self, *args, **kwargs):
        super(GrayscaleBlock, self).__init__(*args, **kwargs)
    
    def setPixelOnMap(self, x: int, y: int, data: list[list[int]], stage: bool, ballID = -1):
        colour = [0, 0, 0]
        print(data[y][x])
        if stage:
            match data[y][x]:
                case 0:
                    colour = [0, 70, 0]
                case 1:
                    colour = [0, 0, 70]
                case 2:
                    colour = [70, 0, 0]
                case 3:
                    colour = [0, 0, 0]
        
        #if not stage: # check if coords from ball has to adjusted for
        if y > 31 and x > 63: # upper panel, upper border
            y -= 32
            x -= 64
            
        elif y < 0 and x > 63: # upper panel, bottom border
            y += 32
            x -= 64
            
        if y > 31 and x <= 63: # bottom panel, upper border
            y -= 32
            x += 64
            
        elif y < 0 and x <= 63: # bottom panel, bottom border
            y += 32
            x += 64
        
        if ballID == 1:
            colour = [255, 255, 255]
        self.matrix.SetPixel(x + 64, y, colour[0], colour[1], colour[2])
    
    def updateText(self, offscreen_canvas, font, textColor, x: int, y: int, text: str):
        offscreen_canvas.Clear()
        graphics.DrawText(offscreen_canvas, font, x, 15, textColor, text)
        return self.matrix.SwapOnVSync(offscreen_canvas)
        
    def run(self):
        sub_blocks = 16
        width = self.matrix.width
        height = self.matrix.height
        x_step = max(1, width / sub_blocks)
        y_step = max(1, height / sub_blocks)
        x = 0
        y = 0
        timer = 0.2
        f = open('map_info.json')
        data = json.load(f)
        
        offscreen_canvas = self.matrix.CreateFrameCanvas()
        font = graphics.Font()
        font.LoadFont("5x8.bdf")
        textColor = graphics.Color(0, 0, 255)
        pos = 32
        my_text = "test: " + str(7)
        offscreen_canvas = self.updateText(offscreen_canvas, font, textColor, pos, 10, my_text)
                
        if True:
            for yArray in range(0, len(data)):
                for xArray in range(0, len(data[yArray])):
                    self.setPixelOnMap(xArray, yArray, data, True)
        if False:
            for yArray in range(0, height):
                for xArray in range(32, width):
                    self.matrix.SetPixel(xArray, yArray, 0, 20, 0)
        
        for bub in range (31, 64):
            self.matrix.SetPixel(bub, 30, 0, 0, 255)
            
        
        
        while True:
            x += 3
            y += 1
            if y > 63:
                y = 0
            elif y < 0:
                y = 63
            if x > 63:
                x = 0
            elif x < 0:
                x = 63 
            self.setPixelOnMap(x, y, data, False, 1)
            
            time.sleep(timer)
            self.setPixelOnMap(x, y, data, True)
            
        while False:
            for y in range(0, height):
                for x in range(0, width):
                    c = sub_blocks * int(y / y_step) + int(x / x_step)
                    if count % 4 == 0:
                        self.matrix.SetPixel(x, y, c, c, c)
                    elif count % 4 == 1:
                        self.matrix.SetPixel(x, y, c, 0, 0)
                    elif count % 4 == 2:
                        self.matrix.SetPixel(x, y, 0, c, 0)
                    elif count % 4 == 3:
                        self.matrix.SetPixel(x, y, 0, 0, c)
            print(x, y, c)
            count += 1
            time.sleep(2)


# Main function
if __name__ == "__main__":
    grayscale_block = GrayscaleBlock()
    if (not grayscale_block.process()):
        grayscale_block.print_help()
