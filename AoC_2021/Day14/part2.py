from collections import Counter
from copy import deepcopy

def readFile(path):
    template,steps = open(path, "r").read().split('\n\n')
    steps =  {l.split(" -> ")[0]:l.split(" -> ")[1] for l in steps.split("\n")}
    return template,steps


def solve(path):
    template,steps = readFile(path)
    ngrams = dict(filter(lambda k: len(k[0]) > 1,{template[pos:pos+2]:0 for pos in range(len(template))}.items()))
    ngramList = [template[pos:pos+2] for pos in range(len(template))]
    for ngram in set(ngramList):
        ngrams[ngram] = Counter(ngramList)[ngram]

    print(ngrams)
    letters = {l:0 for l in set(template)}
    for l in template:
        letters[l] += 1

    for _ in range(40):
        currentNgrams = deepcopy(ngrams)
        for ngram in currentNgrams:
            if ngram in steps:
                newLetter = steps[ngram]
                newNgrams = [ngram[0] + newLetter,newLetter + ngram[1]]
                amount = currentNgrams[ngram]
                ngrams[ngram] -= amount
                
                if newLetter not in letters:
                    letters[newLetter] = 0
                letters[newLetter] += amount

                for newNgram in newNgrams:
                    if newNgram not in ngrams:
                        ngrams[newNgram] = 0
                    ngrams[newNgram] += amount
                    
                
    result = Counter(letters).most_common()
    mostCommon = result[0]
    leastCommon = result[-1]
    difference = mostCommon[-1] - leastCommon[-1]
    return "most common: {}, least common: {}, difference: {}".format(mostCommon,leastCommon,difference)


print("With test input: {}\nWith real input: {}".format(solve('testinput.txt'),solve('input.txt')))
