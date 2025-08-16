from typing import Annotated
from enum import Enum
import organizer 
from fastapi import FastAPI, Body, Query, Path, Depends
from pydantic import BaseModel, Field
cards = organizer.Cards([])
print(cards.read())
app = FastAPI()


async def id_dep(id:int = Path(examples=["214"])):
    return str(id)

#the one string row for rows
row = Annotated[list[str], Field(max_length=4, min_length=4, examples=[
    ["name", "214", "whales", "2025-2-3"]
    ])
    ]
    
class Rows(BaseModel):
    dt: list[row] 

    
class Columns(str, Enum):
    name = "name"
    favoritefood = "favoriteFood"
    date = "date"
    

@app.get("/")
async def read():
    return {"result": cards.read()}

@app.put("/add")
async def insert(lst: Rows):
    return {"result": cards.insert(lst.dt)}

@app.put("/update/{id}")
async def update(id: Annotated[str, Depends(id_dep)],
                     in_: Columns = Query(alias="in"), 
                    to: str = Query(examples=["fish","tomatoes"]) ):
    return {"result": cards.update(id, [in_, to])}

@app.delete("/delete/{id}")
async def delete(id: Annotated[str, Depends(id_dep)]
):
    return {"result": cards.delete(id)}

