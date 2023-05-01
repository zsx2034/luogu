import random as r

with open("t.txt", "w") as f:
    for i in range(10000000):
        f.write(str(r.randint(1, 10000)) + " ")