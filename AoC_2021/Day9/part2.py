import numpy as np 
import sys
sys.path.insert(0,'../..')
from HelperFunctions.helperfunctions import readInputAsNumpy

directions = np.array([[1,0],[0,1],[-1,0],[0,-1]])
def readInput(path):
    return np.matrix([[int(i) for i in row] for row in open(path, "r").read().split('\n')])

def isValidCoordinate(coordinate,matrix):
    rowAmount, colAmount = matrix.shape
    if (coordinate[0] >= 0) and (coordinate[0] < rowAmount):
        if (coordinate[1] >= 0) and (coordinate[1] < colAmount):
            return True
    return False
def getNeighbors(currentPos,matrix):
    return  np.array(list(filter(lambda x:isValidCoordinate(x,matrix),[d + currentPos for d in directions])))

def isLocalMinimum(position,matrix):
    neighbors = getNeighbors(position,matrix)
    Y = np.transpose(neighbors)[0]
    X = np.transpose(neighbors)[1]
    neighbors = matrix[Y,X]
    currentPosVal = matrix[position[0],position[1]]
    if np.all((currentPosVal-neighbors) < 0) :
        return True
    return False
    
def getLowPoints(matrix): 
    lowPoints = []
    for row in range(matrix.shape[0]):
        for column in range(matrix.shape[1]):
            if isLocalMinimum(np.array([row,column]),matrix=matrix):
                lowPoints.append([row,column])
    return lowPoints

def getNeighborsInBasin(currentPos,matrix,currentBasin):
    neighbors = np.array(list(filter(lambda x: not any(np.equal(currentBasin,x).all(1)),getNeighbors(currentPos, matrix))))
    neighborsInBasin = np.array(list(filter(lambda x: matrix[x[0],x[1]] < 9 ,neighbors)))
    if len(neighborsInBasin):
        currentBasin = neighborsInBasin if len(currentBasin) == 0 else np.concatenate((currentBasin, neighborsInBasin),axis=0)
        for neighbor in neighborsInBasin:
            currentBasin = np.append(currentBasin,getNeighborsInBasin(neighbor, matrix,currentBasin),axis=0)
    return np.unique(currentBasin,axis=0)

def getBasinSize(lowPoints,matrix):
    basinLengths = []
    for lowPoint in lowPoints:
        currentBasin = getNeighborsInBasin(lowPoint, matrix,np.array([lowPoint]))
        basinLengths.append(len(currentBasin))
     
    return np.prod(sorted(basinLengths,reverse=True)[:3])


def solve(path):
    matrix = readInput(path)
    lowPoints = getLowPoints(matrix)
    return getBasinSize(lowPoints,matrix)
    

print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))