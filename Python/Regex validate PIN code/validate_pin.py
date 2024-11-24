def validate_pin(pin):
    #Check if the length is exactly 4 or 6
        if len(pin) not in (4, 6):
            return False
    #Checks each character in string, to be a number only.    
        for char in pin:
            if not char.isdigit():
                return False
    #Pin meets the requirments, return true.         
        else:
            return True
            
