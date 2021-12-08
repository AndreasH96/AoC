import numpy as np
def readFile(path):
    rows = open(path, "r").read().split('\n')
    rowParts = [row.split(' | ')[1]  for row in rows]
    segments = [rowPart.split(' ') for rowPart in rowParts]
    return segments
def solve(path):
    easyNumberLengths = [2,3,4,7]
    segments = list(np.array(readFile(path)).flat)
    lengths = np.char.str_len(segments)
    easyNumbers = list(filter(lambda x: x in easyNumberLengths,lengths))
    return len(easyNumbers)
    
print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))