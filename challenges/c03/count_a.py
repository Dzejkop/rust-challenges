def count_lowercase_a_in_file(file_name):
    with open(file_name, 'r') as file:
        text = file.read()
        a_count = text.count('a')
    return a_count

if __name__ == "__main__":
    file_name = 'large_text_file.txt'
    a_count = count_lowercase_a_in_file(file_name)
    print(f"The file {file_name} contains {a_count} lowercase 'a's.")
