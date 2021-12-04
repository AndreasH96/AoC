import sys
sys.path.insert(0,'../..')
from HelperFunctions.helperfunctions import readInputAsNumpy
import numpy as np
def solve(path):
    data = readInputAsNumpy(path)
    return len(list(filter(lambda x: x < 0, data[:-1] - np.roll(data,-1)[:-1])))
    
print(solve("input.txt"))