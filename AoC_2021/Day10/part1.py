chunkCombos = {'(':')','[':']','{':'}','<':'>'}
chunkEnderScores = {')':3,']':57,'}':1197,'>':25137}

def readInput(path):
    return [row for row in open(path, "r").read().split('\n')]


def solve(path):
    rows = readInput(path)
    corruptRows = []
    incompleteRows = []
    expectedEndStack = []
    corruptScore = 0
    for row in rows:
        for sign in row:
            if sign in chunkCombos.keys():
                expectedEndStack.append(chunkCombos[sign])
            elif sign == expectedEndStack[-1]:
                expectedEndStack.pop()
            else:
                corruptScore += chunkEnderScores[sign]
                corruptRows.append(row)
                break
    return corruptScore

print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))