from copy import deepcopy
from collections import Counter
nodes = {}
smallNodeNames = []
def readInput(path):
    nodeBase = {"name":"","type":"","connections":set()}
    rows = open(path, "r").read().split('\n')
    allNodeNames=set()
    for row in rows:
        allNodeNames = allNodeNames.union(set(row.split("-")))
    
    for nodeName in allNodeNames:
        node = deepcopy(nodeBase)
        node["name"] = nodeName
        node["type"] = "small" if nodeName.lower() == nodeName else "big"
        if node["name"] == "end":
            node["type"] = "end"

        nodes[nodeName] = node

    for row in rows:
        firstName,secondName = row.split("-")
        node1 = nodes[firstName]
        node2 = nodes[secondName]
        if secondName != "start":
            node1["connections"].add(secondName)
        if firstName != "start":
            node2["connections"].add(firstName)
    return nodes

def exploreConnections(currentNode,currentPath,paths):
    if currentNode["type"] == "small":
        if currentNode["name"] in currentPath:
            smallNodeCounts = [Counter(currentPath)[n] for n in smallNodeNames]
            if  2 in  smallNodeCounts:
                return paths      

    currentPath.append(currentNode["name"])
    
    if(currentNode["name"] =="end"):
        return paths +1
    else:
        for connection in currentNode["connections"]:
            currPath = deepcopy(currentPath)
            paths = exploreConnections(nodes[connection],currPath,paths)
          
        return paths



def solve(path):
    nodes = readInput(path)
    global smallNodeNames
    smallNodeNames = list(filter(lambda x: nodes[x]["type"]=="small",nodes))
    startNode = nodes["start"]
    paths = exploreConnections(startNode,[],0)
    return paths

print("With test input1: {}\nWith test input2: {}\nWith test input3: {}\n".format(solve('testinput.txt'),solve('testinput2.txt'),solve('testinput3.txt')))
print("With real input: {}\n".format(solve('input.txt')))