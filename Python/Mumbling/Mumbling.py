def accum(st):
    return '-'.join(c.upper() + c.lower() * i for i, c in enumerate(st)) 
