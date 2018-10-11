import sys
with open(sys.argv[1]) as myFile:
    nodes = myFile.readline().split(',')
    edges = [(a,b) for a,b in x.split(',') for x in myFile.readline()[1:-1].replace('(','').replace(')','').replace(' ','').split(';')]
    graph = {}
    for node in nodes:
        graph[node] = []
    for a,b in edges:
        graph[a].push(b)
        graph[b].push(a)
if connected(graph):
    odd_deg_count = len(x for x in graph.keys() if len(graph[x])%2)
    if odd_deg_count == 0:
        print('circuit')
        # find the euler circuit
    else if odd_deg_count ==i 2:
        print('path')
        # find euler path
    else:
        print('none')
else:
    print('none')
