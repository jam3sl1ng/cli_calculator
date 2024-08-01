import time

print("Welcome to the CLI Calculator!")

time.sleep(2)

num1 = input("Please enter the first number ")
op = input("Please enter the operator (+ - / *) ")
num2 = input("Please enter the second number ")

if op == "+":
    result = int(num1) + int(num2)
elif op == "-":
    result = int(num1) - int(num2)
elif op == "/":
    result = int(num1) / int(num2)
elif op == "*":
    result = int(num1) * int(num2)

print(f"The result is: {result}")