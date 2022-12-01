#!/bin/env python3
a = [18,50,9,4,25,37,39,40,29,6,41,28,3,11,31,8,1,38,33,30,42,15,26,36,43]
b = [32,44,19,47,12,48,14,2,13,10,35,45,34,7,5,17,46,21,24,49,16,22,20,27,23]
#a = [9,2,6,3,1]
#b = [5,8,4,7,10]

def play(a, b, recurse):
    played = set()
    while a and b:
        curr = (tuple(a), tuple(b))
        if curr in played:
            return True, a
        played.add(curr)
        (x, *a), (y, *b) = a, b

        a_won = None
        if recurse and len(a) >= x and len(b) >= y:
            a_won,_ = play(a[:x], b[:y], True)
        else:
            a_won = x > y
        if a_won:
            a.append(x)
            a.append(y)
        else:
            b.append(y)
            b.append(x)
    if a:
        return True, a
    else:
        return False, b

def win(_,w):
    score = 0
    for i, val in enumerate(reversed(w)):
        score += val * (1+i)
    print(score)

win(*play(a,b,False))
win(*play(a,b,True))

