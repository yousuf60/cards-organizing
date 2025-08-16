import organizer 
test_id  = "0"
test_id_pos = -1
converting_data = [
        ["zelenisky", "184121", "chicken"                 , "2060-3-13"], 
        ["Ali"      , "124352348" , "all meats and vegetables", "2070-7-12"],
        ["Ali"      , "124323548" , "all meats and vegetables", "2070-7-12"],
        ["Ali"      , "124348" , "all meats and vegetables", "2070-7-12"],
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
    print(obj.get(id=test_id), "got", [[converting_data[test_id_pos][i]] for i in range(0,4)])
    assert obj.get(id=test_id) ==  [[converting_data[test_id_pos][i]] for i in range(0,4)]


def test_update():
    global returned_read
    returned_read[2][test_id_pos] = "birds"
    print(obj.update(test_id,["favoritefood", "birds"]) )
    assert obj.read() == returned_read
    

def test_delete():
    global returned_read
    for i in returned_read:i.pop()
    assert obj.delete(id=test_id)
    assert obj.read() == returned_read

def test_insert():
    assert obj.insert([["Ali"      , "1232542348" , "all meats and vegetables", "2070-7-12"],

    ])
    

if __name__ == "__main__":
    print(obj)
    print(obj.__dir__())
    print(obj.read())
    print(obj.update(test_id,["favoritefood", "birds"]) )
    if not obj.get("112412342313412412423")[0]:print("empty")
