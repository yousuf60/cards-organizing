import organizer 
 
converting_data = [
        ("zelenisky", 184121, "chicken"                 , "2060-3-13"), 
        ("Ali"      , 12548 , "all meats and vegetables", "2070-7-12"),
    ]

# name , id , preferred food , expired date
obj = organizer.Cards(converting_data)
print(obj)
print(obj.__dir__())
print(obj.read())

returned_read = [[],[], [], []]
# only converting rows to columns
for i0 in range(0,4):
    for i in [0,1]: 
        returned_read[i0].append(converting_data[i][i0])
        
def test_read():
    assert obj.read() == tuple(returned_read)
    

def test_get_with_id():
    print(obj.get(id=12548), "got")
    assert obj.get(id=12548) ==  tuple([[converting_data[1][i]] for i in range(0,4)])

def test_update():
    returned_read[2][1] = "birds"
    
    print(obj.update(12548,["favoritefood", "birds"]) )
    assert obj.read() == returned_read

#the return in the end because it is real data test
#this way is not for production of caurse , untile now
def test_delete():
    for i in returned_read:i.pop()
    assert obj.delete(id=12548)
    assert obj.read() == returned_read
    
    
    
