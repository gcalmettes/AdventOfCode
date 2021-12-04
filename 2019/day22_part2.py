m = 10007
m = 119315717514047

#https://en.wikibooks.org/wiki/Algorithm_Implementation/Mathematics/Extended_Euclidean_algorithm#Python
def modinv(a, b):
    def egcd(a, b):
        if a == 0: return (b, 0, 1)
        else:
            g, x, y = egcd(b % a, a)
            return (g, y - (b // a) * x, x)
    g, x, _ = egcd(a, b)
    if g == 1: return x % b


with open('day22_input.txt') as f:
    lines = f.readlines()
    lines.reverse()

k = 1
c = 0
for line in lines:
    l = line.split(" ")
    if len(l) == 2:
        i = int(l[-1])
        #y = (y+i)%m
        #print("(y+"+str(i)+")")
        c = (c + i) %m
    else:
        if l[1] == "with":
            i = int(l[-1])
            mi = modinv(i, m)
            #y = (mi * y) % m
            #print("("+str(mi)+" * y)")
            k = (k * mi) % m
            c = (c * mi) % m
        else:
            #y = (-y-1)%m
            #print("(-(y-1)")
            k = (-k) % m
            c = (-c-1) % m
#print("%d * %d + %d === %d"%(k, 2020, c, y))
#print((k*2020+c)%m)

kl = [k]
cl = [c]
y = 2020
x = 101741582076661
b = [b is '1' for b in list(bin(x)[2:])]
lb = len(b)-1
#print(b)

for _ in range(lb):
    c = (c + c*k) %m
    k = (k*k) %m
    kl.append(k)
    cl.append(c)

for i in range(lb, -1, -1):
    if b[lb-i]:
        y = (kl[i] * y + cl[i])%m

print(y)