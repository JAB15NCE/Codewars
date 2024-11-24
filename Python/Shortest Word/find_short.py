def find_short(s):
    #scans string and splits the words up to be checked.
    words = s.split()
    #Set the shortest word length to = l from the string
    l = min(len(word) for word in words) if words else 0
    
    return l # l: shortest word length
