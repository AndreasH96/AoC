import numpy as np
def readInputAsNumpy(path): 
    return np.array([int(i) for i in open(path, "r").read().split('\n')])