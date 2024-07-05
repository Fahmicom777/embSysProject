#!/usr/bin/env python
from samplebase import SampleBase
from rgbmatrix import graphics
import time
import json

class ThreePanelGolf(SampleBase):
    
    def __init__(self, *args, **kwargs):
        super(ThreePanelGolf, self).__init__(*args, **kwargs)
    
    def convertToCoords(self, x: int, y: int):
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
        return x, y
    
    def setPixelOnMap(self, x: int, y: int, data: list[list[int]], stage: bool, win: bool, ballID = -1):
        colour = [0, 0, 0]
        #print(data[y][x])
        if not win:
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
            x, y = self.convertToCoords(x, y)
            colourID = ballID % 7
            if ballID != -1:
                match (colourID):
                    case 0:
                        colour = [255, 255, 255]
                    case 1:
                        colour = [255, 255, 0]
                    case 2:
                        colour = [0, 255, 255]
                    case 3:
                        colour = [255, 0, 255]
                    case 4:
                        colour = [100, 50, 250]
                    case 5:
                        colour = [250, 50, 100]
                    case 6:
                        colour = [255, 10, 55]
                    case 7:
                        colour = [55, 10, 255]
                
            self.matrix.SetPixel(x + 64, y, colour[0], colour[1], colour[2])
        else:
            print("yup")
            offscreen_canvas = self.matrix.CreateFrameCanvas()
            font = graphics.Font()
            font.LoadFont("./5x8.bdf")
            textColor = graphics.Color(255, 255, 255)
            pos = 150
            winText = ["WIN !", "BALL " + str(ballID)]
            self.updateText(offscreen_canvas, font, textColor, [[32, 10], [32, 26]], winText)
    
    def updateText(self, offscreen_canvas, font, textColor, coords: list[list[int]], text: list[str]):
        offscreen_canvas.Clear()
        for index in range(0, len(text)):
            graphics.DrawText(offscreen_canvas, font, coords[index][0], coords[index][1], textColor, text[index])
        return self.matrix.SwapOnVSync(offscreen_canvas)
        
    def run(self):
        firstRun = True
        sub_blocks = 16
        width = self.matrix.width
        height = self.matrix.height
        x_step = max(1, width / sub_blocks)
        y_step = max(1, height / sub_blocks)
        x = 0
        y = 0
        bProbs = {}
        timer = 0.7
        newRound = False
        f = open('./map/map_info.json')
        fMap = json.load(f)
        
        offscreen_canvas = self.matrix.CreateFrameCanvas()
        font = graphics.Font()
        font.LoadFont("./5x8.bdf")
        textColor = graphics.Color(0, 0, 255)
        pos = 32
        my_text = ["D8:3A:", "DD:8D:", "6A:94"]
        
        bigHole = []
        fPlayer = {}
        connected = []
        debug = 0
        test = 0
        
        # Place hole till someone joins
        while (True):
            while (True):
                debug = 0                
                while (True):
                    try:
                        test += 1
                        f = open('./golf_info.json')
                        fPlayer = json.load(f)
                        connected = fPlayer["players"]
                        test = 0
                        break
                    except Exception as e:
                        continue
                #try:
                if (connected == [] or newRound or firstRun):
                    self.updateText(offscreen_canvas, font, textColor, [[32, 10], [32, 18], [32, 26]], my_text)
                    newRound = False
                    firstRun = False 
                    print("world")
                    print(connected == [] or newRound)
                    print(connected == [])
                    print(newRound)
                    print("world")
                    debug = 1
                    xHole = int(fPlayer["map"]["end_pos_x"])
                    yHole = int(fPlayer["map"]["end_pos_y"])
                    xHole, yHole = self.convertToCoords(xHole, yHole)
                    if (xHole > 60 or yHole > 60):
                        continue
                    
                    coHoles = [
                        {"x": xHole, "y": yHole},
                        {"x": xHole+1, "y": yHole},
                        {"x": xHole-1, "y": yHole},
                        {"x": xHole, "y": yHole+1},
                        {"x": xHole, "y": yHole-1},
                        ]
                    debug = 2
                    # If hole is in new position, set the old position back to gras
                    if (bigHole != coHoles) and bigHole != []:
                        for noHole in bigHole:
                            fMap[noHole["y"]][noHole["x"]] = 0
                    bigHole = coHoles
                    debug = 3
                       
                    debug = 4
                    for coHole in coHoles:
                        fMap[coHole["y"]][coHole["x"]] = 3
                    debug = 5
                    # Draw map
                    for yArray in range(0, len(fMap)):
                        debug = 6
                        for xArray in range(0, len(fMap[yArray])):
                            debug = 7
                            self.setPixelOnMap(xArray, yArray, fMap, True, False)
                    print("beep")
                    time.sleep(2)
                else:
                    break
        
            #if False:
             #   for yArray in range(0, height):
              #      for xArray in range(32, width):
               #         self.matrix.SetPixel(xArray, yArray, 0, 20, 0)
            
            for cLine in range (31, 64):
                self.matrix.SetPixel(cLine, 30, 0, 0, 255)
            newRound = False
            yo = False
            while True:
                print("newRound: " + str(newRound))
                if newRound:
                    print("look man, i dunno")
                    sleep(3)
                    break
                debug = 10
                try:
                    test += 1
                    f = open('./golf_info.json')
                    fPlayer = json.load(f)
                    test = 0
                except Exception as e:
                    print(test)
                    continue
                debug = 11
                if len(fPlayer["players"]) > 0:
                    if yo:
                        self.setPixelOnMap([], [], fMap, True, True, 7)
                        yo = False
                        time.sleep(5)
                    try:                        
                        debug = 12
                        players = fPlayer["players"]
                        debug = 13
                        """
                        I need the information from the game that a player has disconnected.
                        Now the code has to check every time who is new and who is not there
                        to cover the edge cases. Not very efficient code
                        """
                        
                        newPlayers = []
                        for player in players:
                            """
                            1. Ball ist in einer anderen Position
                            2. Neuer Spieler hat sich verbunden
                            """
                            debug = 22
                            # Check ob sich bei diesen player überhaupt was geändert hat
                            if player not in bProbs.values():
                                debug = 33
                                # check if ID is in pProbs. If not, then its a new player
                                newPlayer = True
                                for key in bProbs.keys():
                                    debug = 44
                                    if player["id"] == key:
                                        debug = 55
                                        # New Values for this Ball, since id can be found, but other
                                        # values are not the same
                                        bProbs[key] = player
                                        newPlayer = False
                                        break;
                                if newPlayer:
                                    debug = 66
                                    # this player could not be found in bProbs. Its a new player
                                    newPlayers.append(player)
                        if newPlayers != []:
                            for newPlayer in newPlayers:
                                bProbs[newPlayer["id"]] = newPlayer
                        
                        
                        # Check if a User has disconnected
                        lostPlayers = []
                        for key in bProbs.keys():
                            debug = 33
                            isConnected = False
                            for player in players:
                                debug = 44
                                # check if ID can be found. if not, then its the disconnected player
                                if key == player["id"]:
                                    debug = 55
                                    isConnected = True
                                    break
                            if not isConnected:
                                debug = 66
                                lostPlayers.append(bProbs[key])
                        if lostPlayers != []:
                            debug = 77
                            for lostPlayer in lostPlayers:
                                debug = 88
                                del bProbs[bProbs.index(lostPlayer)]
                                debug = 99
                                print("Disconnected: ID - " + bProbs.index(lostPlayer))
                                time.sleep(1)
                        debug = 14
                        
                        debug = 15
                        print(bProbs)
                        time.sleep(0.5)
                        winner = -1
                        for player in bProbs.values():
                            x = int(player["ball_pos_x"])
                            y = int(player["ball_pos_y"])
                            #x, y = self.convertToCoords(x, y)
                            pID = int(player["id"])
                            ballHeight = int(player["ball_height"])
                            self.setPixelOnMap(x, y, fMap, False, False, pID)
                            print("player " + str(player["id"]))
                            if {'x': x, 'y': y} in bigHole:
                                winner = pID
                                newRound = True
                                break
                        if newRound:
                            self.setPixelOnMap([], [], fMap, True, True, 7)
                            time.sleep(3)
                            break
                        else:
                            time.sleep(timer)
                            self.setPixelOnMap(x, y, fMap, True, False)
                            time.sleep(timer)
                    except Exception as e:
                        print(fPlayer["players"])
                        print("---")
                        print(bProbs)
                        print("ERROR: ")
                        print(e)
                        print(debug)
                else:
                    print("!AYO!")
                    bProbs = {}
                    break
            for cLine in range (31, 64):
                self.matrix.SetPixel(cLine, 30, 0, 0, 0)

# Main function
if __name__ == "__main__":
    grayscale_block = ThreePanelGolf()
    if (not grayscale_block.process()):
        grayscale_block.print_help()
