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

def isLocalMinimum(position,matrix):
    neighbors = np.array(list(filter(lambda x:isValidCoordinate(x,matrix),[d + position for d in directions])))
    Y = np.transpose(neighbors)[0]
    X = np.transpose(neighbors)[1]
    neighbors = matrix[Y,X]
    currentPosVal = matrix[position[0],position[1]]
    if np.all((currentPosVal-neighbors) < 0) :
        return True
    return False
    
def solve(path): 
    matrix = readInput(path)
    localMinimumValues = []
    for row in range(matrix.shape[0]):
        for column in range(matrix.shape[1]):
            if isLocalMinimum(np.array([row,column]),matrix=matrix):
                localMinimumValues.append(matrix[row,column])
    return np.sum(localMinimumValues) + len(localMinimumValues)
print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))