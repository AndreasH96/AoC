import numpy as np

def readFile(path):
    returnDict = {0:0,1:0,2:0,3:0,4:0,5:0,6:0,7:0,8:0}
    for i in open(path, "r").read().split(','):
        returnDict[int(i)]  += 1
    return returnDict

def passDay(data):
    newFishAmount = data[0]
    for k in list(data.keys())[:-1]:
        data[k] = data[k+1]
    data[8] = newFishAmount
    data[6] += newFishAmount
    return data
    

data = readFile('input.txt')   
for x in range(256):
    data = passDay(data)
print(sum(data.values()))
    