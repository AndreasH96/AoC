from collections import deque
chunkCombos = {'(':')','[':']','{':'}','<':'>'}
autocompleteScores = {')':1,']':2,'}':3,'>':4}

def readInput(path):
    return [row for row in open(path, "r").read().split('\n')]


def solve(path):
    rows = readInput(path)
    incompleteScores = []
    for row in rows:
        expectedEndStack = []
        incompleteScore = 0
        for sign in row:
            if sign in chunkCombos.keys():
                expectedEndStack.append(chunkCombos[sign])
            elif sign == expectedEndStack[-1]:
                expectedEndStack.pop()
            else:
                expectedEndStack = []
                break
        for missingChar in reversed(expectedEndStack):
            incompleteScore *=5
            incompleteScore += autocompleteScores[missingChar]
        if len(expectedEndStack) > 0:
            incompleteScores.append(incompleteScore)
            
    return sorted(incompleteScores)[int(len(incompleteScores)/2)]

print("With test input: {}\nWith real input: {} ".format(solve('testinput.txt'),solve('input.txt')))