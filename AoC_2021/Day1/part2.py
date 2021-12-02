import sys
sys.path.insert(0,'../..')
from HelperFunctions.helperfunctions import readInputAsNumpy
import numpy as np

data = readInputAsNumpy("input.txt")
sliding_windows = np.sum(np.array([data[:-2],np.roll(data,-1)[:-2],np.roll(data,-2)[:-2]]),0)
print(len(list(filter(lambda x: x<0, sliding_windows[:-1] - np.roll(sliding_windows,-1)[:-1]))))

