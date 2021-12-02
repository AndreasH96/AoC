import numpy as np
directions = {'forward':np.array([1,0]),'up':np.array([0,-1]),'down':np.array([0,1])}
def solve(path): 
    return np.prod(np.sum(np.array([directions[i.split(' ')[0]]*int(i.split(' ')[1]) for i in  open(path, "r").read().split('\n')]),0))

print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))