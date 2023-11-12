text = input("text: ")

for char in text:
    print(f"push {ord(char)}\nread\neq\npush CMP_FAIL\njmpc\n")
