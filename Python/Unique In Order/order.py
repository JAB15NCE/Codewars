def unique_in_order(sequence):
    result = []
    
    for i, item in enumerate(sequence):
        if i == 0 or item != sequence[i -1]:
            result.append(item)    
    return result
