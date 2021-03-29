from math import sqrt
def area_of_triangle(a, b):
    return 0.5 * a * b

def b_from_a(a):
    return 1 - sqrt(1-a*a)

def new_a(previous_a, previous_b):
    return 0.5 * sqrt(previous_a*previous_a + previous_b*previous_b)

def pi_from_area(area):
    return area * 4

area = 0.5
step = 1
a = sqrt(2)*0.5
while True:
    b = b_from_a(a)
    area += area_of_triangle(a, b) * 2**step
    step += 1
    a = new_a(a, b)
    print("{:.50f}".format(pi_from_area(area)), step)
