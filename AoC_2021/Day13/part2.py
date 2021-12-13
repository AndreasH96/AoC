import numpy as np
from numpy.lib.shape_base import column_stack 
import bitarray

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
        result = (left+right).T 
        
    else:
        top,bot = np.split(matrix,[instruction[1]])
        bot = np.flip(bot[1:],0)
        #if(top.shape[0] > bot.shape[0]):
            #top = top[:1]
        outliers = top[:top.shape[0]-bot.shape[0]]
        top = top[top.shape[0]-bot.shape[0]:]
        result = np.concatenate((outliers,top+bot))
        

    result[result > 1] = 1
    return result

def solve(path):
    matrix,instructions = readInput(path)
    
    for instructon in instructions:
        matrix = fold(matrix,instructon)
    for row in matrix:#np.reshape(matrix,(8,30)).T:
        stringList = ['.' if x == 0 else '#' for x in row]
        print("".join(stringList))

print("With test input: \n")
solve('testinput.txt')
print('\nWith real input: \n')
solve('input.txt')