from collections import Counter


def readFile(path):
    template,steps = open(path, "r").read().split('\n\n')
    steps =  {l.split(" -> ")[0]:l.split(" -> ")[1] for l in steps.split("\n")}
    return template,steps



def solve(path):
    template,steps = readFile(path)
    for x in range(10):
        offset = 0
        for pos in range(len(template)):
            current_ngram = template[pos+offset:pos+offset+2]
            if current_ngram in steps.keys():
                template = template[:pos+offset+1] + steps[current_ngram] + template[pos+offset+1:]
                offset +=1

    count = Counter(template).most_common()
    return count[0][1] - count[-1][1]


print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))


