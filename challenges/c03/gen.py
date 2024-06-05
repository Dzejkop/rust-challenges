import random
import string

def generate_random_word(min_length=3, max_length=8):
    length = random.randint(min_length, max_length)
    letters = string.ascii_lowercase
    return ''.join(random.choice(letters) for _ in range(length))

def generate_large_text_file(file_name, min_words=950, max_words=1500, min_word_length=3, max_word_length=8, line_break_probability=0.1):
    num_words = random.randint(min_words, max_words)
    with open(file_name, 'w') as file:
        for i in range(num_words):
            word = generate_random_word(min_word_length, max_word_length)
            file.write(word)
            if random.random() < line_break_probability:
                file.write('\n')
            else:
                file.write(' ')
    print(f"Generated {file_name} with {num_words} random words.")

if __name__ == "__main__":
    file_name = 'large_text_file.txt'
    min_words = 9000000           # Minimum number of words to generate
    max_words = 11000000          # Maximum number of words to generate
    min_word_length = 3       # Minimum length of each word
    max_word_length = 8       # Maximum length of each word
    line_break_probability = 0.1 # Probability of a line break after each word
    generate_large_text_file(file_name, min_words, max_words, min_word_length, max_word_length, line_break_probability)
