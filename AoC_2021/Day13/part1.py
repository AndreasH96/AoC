import numpy as np 

def readInput(path):
    matrixRows,instructions = open(path, "r").read().split('\n\n')
    instructions = [row.split(" ")[-1].split('=') for row in instructions.split('\n')]
    instructions = [(x[0],int(x[1])) for x in instructions]
    matrixRows = np.array([[int(number) for number in row.split(',') ]for row in matrixRows.split('\n')])
    maxX, maxY = np.max(matrixRows,0)
    matrix = np.zeros((maxY+1,maxX+1))
    matrix[matrixRows[:,1],matrixRows[:,0]] = 1 
    return matrix,instructions

def fold(matrix,instruction):
    if instruction[0] == 'x':
        left,right = np.split(matrix.T,[instruction[1]])
        right = np.flip(right[1:],0)
        result = left+right 
        return result.T
    else:
        top,bot = np.split(matrix,[7])
        bot = np.flip(bot[1:],0)
        result = top+bot
        return result

def solve(path):
    matrix,instructions = readInput(path)
    foldRes = fold(matrix,instructions[0])

    return len(foldRes[foldRes > 0 ])
    
print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))