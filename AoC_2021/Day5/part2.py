import numpy as np
def handleLine(line):
    fromCoord = (int(line[0].split(",")[0]),int(line[0].split(",")[1]))
    toCoord = (int(line[1].split(",")[0]),int(line[1].split(",")[1]))
    return np.array([fromCoord,toCoord])

def getCoordsBetweenPoints(point1,point2):
    currentX,currentY = point1
    path =[]
    path.append([currentX,currentY])
    while(currentX != point2[0] or currentY != point2[1]):
        currentX = currentX + 1*np.sign(point2[0]-currentX) if currentX != point2[0] else currentX
        currentY = currentY + 1*np.sign(point2[1]-currentY) if currentY != point2[1] else currentY
        path.append([currentX,currentY])
    return path

def createNumpyMatrix(dataPoints):
    maxX = max(np.concatenate((dataPoints[:,0][:,0] , dataPoints[:,1][:,0])))
    maxY = max(np.concatenate((dataPoints[:,0][:,1] , dataPoints[:,1][:,1])))
    dataMap = np.empty([maxX+1,maxY+1])
    dataMap.fill(0)
    for row in dataPoints:
        for point in getCoordsBetweenPoints(row[0],row[1]):
            dataMap[point[1],point[0]] +=1
    return dataMap

def parseMapInput(path):
    lines = [line.split(" -> ") for line in open(path, "r").read().split('\n')]
    data = np.array([handleLine(line)  for line in lines])
    

    matrix = createNumpyMatrix(data)
    
    print(np.sum(matrix>1))

    
parseMapInput("input.txt")
