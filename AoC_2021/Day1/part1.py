import numpy as np
def readInputAsNumpy(path): 
    return np.array([int(i) for i in open(path, "r").read().split('\n')])
data = readInputAsNumpy("input.txt")
data_shifted = np.roll(data,-1)
print(len(list(filter(lambda x: x < 0, data[:-1] - np.roll(data,-1)[:-1]))))



