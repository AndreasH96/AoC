# # Day Twelve

# ## Prep

text = open("day12.txt").read()

from string import ascii_lowercase

class Node:
    
    def __init__(self, v):
        assert v in ascii_lowercase
        self.v = v
        self.predecessors = set()
        
    def __repr__(self):
        return f"Node({self.v})"
    
    def add_predecessor(self, other, d):
        if ord(other.v) - ord(self.v) <= 1:
            other.predecessors.add(self)

# Create the graph and save the start and end for part one

start = None
end = None

nodes = []
for l in text.splitlines():
    new_row = []
    for c in list(l):
        if c == 'S':
            n = Node('a')
            start = n
            new_row.append(n)
        elif c == 'E':
            n = Node('z')
            end = n
            new_row.append(n)
        else:
            new_row.append(Node(c))
    nodes.append(new_row)
            

for i in range(len(nodes)):
    for j in range(len(nodes[i])):
        nodes[i][j].coord = (i, j)
        if i > 0:
            nodes[i][j].add_predecessor(nodes[i-1][j], 'N')
        if i < len(nodes) - 1:
            nodes[i][j].add_predecessor(nodes[i+1][j], 'S')
        if j > 0:
            nodes[i][j].add_predecessor(nodes[i][j-1], 'W')
        if j < len(nodes[i]) - 1:
            nodes[i][j].add_predecessor(nodes[i][j+1], 'E')

# Dijkstra it up to get all min distances

def find_min_dist_index(Q):
    min_dist = 1e9
    min_index = -1
    
    for i in range(len(Q)):
        node = Q[i]
        if node.dist < min_dist:
            min_dist = node.dist
            min_index = i
    
    return min_index

Q = []
for row in nodes:
    for node in row:
        node.dist = 1e9
        node.prev = None
        Q.append(node)

end.dist = 0

while len(Q) > 0:

    u = Q.pop(find_min_dist_index(Q))
    for v in u.predecessors:
        if v in Q:
            alt = u.dist + 1 # distance is always 1
            if alt < v.dist:
                v.dist = alt
                v.prev = u


# ## Part One

print(start.dist)


# ## Part Two

print(min(n.dist for row in nodes for n in row if n.v == 'a'))