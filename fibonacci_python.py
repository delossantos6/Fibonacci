
listvar = [0, 1]
n = 2

num = int(input("Please input a number: "))

if n >= num:
	listvar[num:] = [] #deletes starting index until the end (excluded)
else:
	while n < num:
		listvar.append(listvar[n-2] + listvar[n-1])
		n += 1

print(num, "number(s) =", listvar)