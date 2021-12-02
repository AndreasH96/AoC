import sys
sys.path.insert(0,'../..')
from HelperFunctions.helperfunctions import readInputAsNumpy
import numpy as np

data = readInputAsNumpy("input.txt")
data_shifted = np.roll(data,-1)
print(len(list(filter(lambda x: x < 0, data[:-1] - np.roll(data,-1)[:-1]))))



