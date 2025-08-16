import organizer 
test_id  = "12212"
converting_data = [
        ["zelenisky", "184121", "chicken"                 , "2060-3-13"], 
        ["Ali"      , "12325448" , "all meats and vegetables", "2070-7-12"],
        ["Ali"      , "1243548" , "all meats and vegetables", "2070-7-12"],
        ["Ali"      , "13232548" , "all meats and vegetables", "2070-7-12"],
        ["Ali"      , test_id , "all meats and vegetables", "2070-7-12"],
    ]

# name , id , preferred food , expired date
obj = organizer.Cards(converting_data)

returned_read = [[],[], [], []]
# only converting rows to columns

for i in converting_data:
    for n in range(0, 4):
        returned_read[n].append(i[n])
        
def test_read():
    assert obj.read() == returned_read
    

def test_get_with_id():
    print(obj.get(id=test_id), "got", [[converting_data[-1][i]] for i in range(0,4)])
    assert obj.get(id=test_id) ==  [[converting_data[-1][i]] for i in range(0,4)]


def test_update():
    global returned_read
    returned_read[2][-1] = "birds"
    print(obj.update(test_id,["favoritefood", "birds"]) )
    assert obj.read() == returned_read
    

#the return in the end because it is real data test
#this way is not for production of caurse , untile now
def test_delete():
    for i in returned_read:i.pop()
    assert obj.delete(id=test_id)
    assert obj.read() == returned_read
    


if __name__ == "__main__":
    print(obj)
    print(obj.__dir__())
    print(obj.read())
    print(obj.update(test_id,["favoritefood", "birds"]) )
