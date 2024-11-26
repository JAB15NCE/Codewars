def alphabet_position(text):
    
    return ' '.join(str(ord(char.lower()) - 96) for char in text if char.isalpha())
