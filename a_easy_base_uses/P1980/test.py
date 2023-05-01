<<<<<<< HEAD
import random


def count_num_in_nums(nums, target):
    cnt = 0
    while nums > 0:
        if nums % 10 == target:
            cnt += 1
        nums //= 10
    return cnt


def count(upbound, target):
    cnt = 0
    for i in range(1, upbound + 1):
        cnt += count_num_in_nums(i, target)
    return cnt

def main():
    with open("data.txt", "w+") as f:
        for i in range(100):
            upbound = random.randint(1, 1e6)
            target = random.randint(0, 9) 
            res = count(upbound, target)
            f.write("{} {} {}\n".format(upbound, target, res))

# 387908
=======
import random


def count_num_in_nums(nums, target):
    cnt = 0
    while nums > 0:
        if nums % 10 == target:
            cnt += 1
        nums //= 10
    return cnt


def count(upbound, target):
    cnt = 0
    for i in range(1, upbound + 1):
        cnt += count_num_in_nums(i, target)
    return cnt

def main():
    with open("data.txt", "w+") as f:
        for i in range(100):
            upbound = random.randint(1, 1e6)
            target = random.randint(0, 9) 
            res = count(upbound, target)
            f.write("{} {} {}\n".format(upbound, target, res))

# 387908
>>>>>>> new_branch
print(count(796581, 0))