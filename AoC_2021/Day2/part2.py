import numpy as np
directions = {'forward':np.array([0,0]),'up':np.array([0,-1]),'down':np.array([0,1])}
def solve(path):
    currentDirection,currentPos = np.array([1,0]), np.array([0,0])
    for row in open(path, "r").read().split('\n'):
        dir,length = row.split(' ')
        currentDirection += directions[dir] * int(length)
        currentPos += currentDirection * int(length) if dir =='forward' else 0
    return np.prod(currentPos)

print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))