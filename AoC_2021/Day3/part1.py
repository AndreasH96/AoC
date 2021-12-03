import numpy as np

def binaryArrayToDecimal(binArray):
    return np.sum([binArray[x]*2**(len(binArray)-1-x) for x in range(len(binArray))])

def solve(path):
    matrix = np.array(np.array([list(map(int,list(i))) for i in open(path, "r").read().split('\n')]))
    gamma = binaryArrayToDecimal([np.bincount(matrix[:,x]).argmax() for x in range(len(matrix.T))])
    epsilon = binaryArrayToDecimal([np.bincount(matrix[:,x]).argmin() for x in range(len(matrix.T))])
    return gamma * epsilon


print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))