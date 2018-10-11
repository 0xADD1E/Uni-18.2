import sys
import math
with open(sys.argv[1]) as myFile:
    in_file = [line.replace(' ','').replace('\n','').split(',') for line in myFile.readlines()]

nodes = {}

for symbol in in_file[0]:
    nodes[symbol] = []

start,end = in_file[1]

for src,dst,weight in in_file[2:]:
    weight = int(weight)
    nodes[src].append((dst,weight))
    nodes[dst].append((src,weight))

# Begin algorithm
distances = {}
for k in nodes.keys():
    distances[k]=math.inf
distances[start] = 0

queue = list(nodes.keys())
path_tracks = {}

while queue:
    node = min([(distances[k],k) for k in queue],key=lambda x: x[0])[1]
    neighbours = nodes[node]
    queue.remove(node)
    for neighbour,weight in neighbours:
        new_distance = distances[node]+weight
        if new_distance < distances[neighbour]:
            distances[neighbour] = new_distance
            path_tracks[neighbour] = node

# And clean up by computing the actual path taken in the right order
actual_path = []
node = end
while node != start:
    actual_path.append(node)
    node = path_tracks[node]
actual_path.append(start)
actual_path = ', '.join(reversed(actual_path))
print(distances[end])
print(actual_path)