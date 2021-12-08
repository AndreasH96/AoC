import numpy as np
import pandas as pd 

def readFile(path):
    rows = open(path, "r").read().split('\n')
    segments = [row.split(' | ')  for row in rows]
    return segments

def findSix(unknowns,knowns):
    five = set(knowns[5])
    one = set(knowns[1])
    for x in unknowns:
        if len(x) - len(five) == 1:
            if not one.issubset(x):
                return "".join(sorted(x))

def findThree(unknowns,knowns):
    one = set(knowns[1])
    nine = set(knowns[9])
    for x in unknowns:
        if len(nine) - len(x) == 1:
            if one.issubset(set(x)):
                return "".join(sorted(x))

def findFive(unknowns,knowns):
    nine = set(knowns[9])
    eight = set(knowns[8])
    bl = eight - nine
    for x in unknowns:
        if len(nine) - len(x) == 1:
            if not list(bl)[0] in x:
               
                return "".join(sorted(x))
                
def findNine(unknowns,knowns):

    fourPlusSeven = set(knowns[4] + knowns[7])
    for x in unknowns:
        if len(x) > len(fourPlusSeven):
            difference = set(sorted(x)) - fourPlusSeven
            if len(difference)  == 1:
                return "".join(sorted(x))

def findZero(unknowns,knowns):
    for x in unknowns:
        if len(x) == 6:
            if x not in knowns.values():
                return "".join(sorted(x))
        
    

 
def prepareWithEasyNumbers(row):
    easyNumberLengths = {2:1,3:7,4:4,7:8}
    knownEasyNumbers = {}
    for word in row.split(" "):
        if len(word) in easyNumberLengths.keys() and not (len(word) in knownEasyNumbers.keys()) :
            knownEasyNumbers[easyNumberLengths[len(word)]] = "".join(sorted(word))
            

    return knownEasyNumbers

def updateUnknowns(row,knowns):
    sortedRow = ["".join(sorted(x)) for x in row.split()]
    return set(filter(lambda x: x not in knowns.values(),sortedRow))

def solveRow(row):
   
    knowns = prepareWithEasyNumbers(row)
    if 8 not in knowns.keys():
        knowns[8] = "abcdefg"

    unknowns = updateUnknowns(row,knowns)
    knowns[9] = findNine(unknowns,knowns)
    unknowns = updateUnknowns(row,knowns)
    knowns[3] = findThree(unknowns,knowns)
    unknowns = updateUnknowns(row,knowns)
    knowns[5] = findFive(unknowns,knowns)
    unknowns = updateUnknowns(row,knowns)
    knowns[2] = list(filter(lambda x: len(x) == 5,unknowns))[0]
    unknowns = updateUnknowns(row,knowns)
    knowns[6] = findSix(unknowns,knowns)
    unknowns = updateUnknowns(row,knowns)
    knowns[0] = findZero(unknowns,knowns)
  
    inverse_knowns =  {v: k for k, v in knowns.items()}
    return inverse_knowns

    

def solve(path):
    
    segments = list(np.array(readFile(path)))
    summations = []
    for segment in segments:
        knownDict = solveRow(segment[0])
       
        output = sum([knownDict["".join(sorted(x))]  * (10**(3-i))for i,x in enumerate(segment[1].split())])
        summations.append(output)
     
    return sum(summations)

print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))