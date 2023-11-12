import random

l = list(range(256));
random.shuffle(l)
for val in l:
    print(f"push {val}")
