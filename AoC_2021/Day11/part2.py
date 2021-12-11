import numpy as np
from itertools import permutations

directions = list(permutations([0,1,-1],2))
directions.extend([(1,1),(-1,-1)])

def readFile(path):
    return np.matrix([[int(i) for i in  row] for row in open(path, "r").read().split('\n')])

def isValidCoordinate(coordinate,matrix):
    rowAmount, colAmount = matrix.shape
    if (coordinate[0] >= 0) and (coordinate[0] < rowAmount):
        if (coordinate[1] >= 0) and (coordinate[1] < colAmount):
            return True
    return False

def flash(flashDumbos,flashed,matrix):
    
    for dumbo in flashDumbos:
        if not tuple(dumbo) in flashed:
            flashed.add(tuple(dumbo))
            neighbors = np.array(list(filter(lambda pos: isValidCoordinate(pos,matrix), dumbo + directions)))
            Y = np.transpose(neighbors)[0]
            X = np.transpose(neighbors)[1]
            matrix[Y,X] += 1
            newFlashDumbos = np.argwhere(matrix > 9)
            flash(newFlashDumbos,flashed,matrix)
            
    return flashed,matrix
    

def takeStep(matrix):
    matrix += 1
    flashDumbos = np.argwhere(matrix > 9)
    flashed,matrix= flash(flashDumbos,set(),matrix)
    matrix[matrix >9] = 0
    return len(flashed),matrix
def solve(path):
    matrix = readFile(path)
    steps = 0
    allFlashed = False
    while not allFlashed:
        steps +=1
        _,matrix = takeStep(matrix)
        if(np.all(matrix==0)):
            return steps



print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))