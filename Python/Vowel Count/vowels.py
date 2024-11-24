def get_count(sentence):
    
    vowels = {"a", "e","i","o","u"}
    
    return sum(1 for char in sentence if char in vowels)
