import numpy as np
from numpy.core.fromnumeric import argmax

def binaryArrayToDecimal(binArray):
    return np.sum([binArray[x]*2**(len(binArray)-1-x) for x in range(len(binArray))])

def filterBinaryArrays(arrays,bitFunction,index):
    filterVector = bitFunction(arrays.T)
    if(len(arrays) > 1):
        return filterBinaryArrays(np.array(list(filter(lambda x: x[index] == filterVector[index],arrays))),bitFunction,index+1)
    return arrays[0]

def getMostCommonBits(matrix):
    returnArray = []
    for col in matrix:
        argMax = np.bincount(col).argmax()
        argMin = np.bincount(col).argmin()
        returnArray.append(argMax if argMax != argMin else 1)
    return np.array(returnArray)

def getLeastCommonBits(matrix):
    returnArray = []
    for col in matrix:
        argMax = np.bincount(col).argmax()
        argMin = np.bincount(col).argmin()
        returnArray.append(argMin if argMax != argMin else 0)
    return np.array(returnArray)

def solve(path):
    matrix = np.array(np.array([list(map(int,list(i))) for i in open(path, "r").read().split('\n')]))
    
    oxygenRating = binaryArrayToDecimal(filterBinaryArrays(matrix,getMostCommonBits,0))
    scrubberRating = binaryArrayToDecimal(filterBinaryArrays(matrix,getLeastCommonBits,0))
    return oxygenRating * scrubberRating
    
    

print("With test input: {}\nWith real input:{} ".format(solve('testinput.txt'),solve('input.txt')))