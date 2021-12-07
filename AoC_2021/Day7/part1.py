import numpy as np
import sys 
def readFile(path):
    return np.array([int(i) for i in open(path, "r").read().split(',')])


def solve(path):    
    optimalValue = 0
    optimalFuelSpending = sys.maxsize
    data = readFile(path)

    for x in range(max(data)):
        if sum(np.absolute(data-x)) < optimalFuelSpending:
            optimalFuelSpending = sum(np.absolute(data-x))
            optimalValue = x
    return optimalValue,optimalFuelSpending
    
print("With Test input")
print("Best move: {} With cost of: {}".format(*solve('testinput.txt')))
print("With Real input")
print("Best move: {} With cost of: {}".format(*solve('input.txt')))