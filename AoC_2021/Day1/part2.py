import numpy as np
def readInputAsNumpy(path): 
    return np.array([int(i) for i in open(path, "r").read().split('\n')])
data = readInputAsNumpy("input.txt")
sliding_windows = np.sum(np.array([data[:-2],np.roll(data,-1)[:-2],np.roll(data,-2)[:-2]]),0)
print(len(list(filter(lambda x: x<0, sliding_windows[:-1] - np.roll(sliding_windows,-1)[:-1]))))

