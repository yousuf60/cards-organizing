import organizer 
 


def make():
    # name , id , preferred food , expired date
    return organizer.Cards([
        ("zelenisky", 184121, "chicken", "2060-3-13"), 
        ("Ali", 12548, "all meats and vegetables", "2070-7-12"),
    
    ])


if __name__ == "__main__":
    x = make()
    
    print(x.read(
        
    ))
    
