from abc import ABC, abstractmethod
from dataclasses import dataclass

@dataclass(frozen=True, slots=True)
class Id: #static class that just stores a string value
    value: str

class Item(ABC): #parent class that all catalog items will inherit from
    def __init__(self, id: Id, title: str) -> None:
        self._id, self._title = id, title

    #classes that inherit from item require id(), title(), and days_allowed() to be defined
    @property
    def id(self) -> Id: 
        return self._id

    @property
    def title(self) -> str: 
        return self._title

    @abstractmethod #days_allowed is defined in the child classes since it defers by item type
    def days_allowed(self) -> int: ...

#Child classes that inherit from Item, they each must define their own days_allowed() method
class Book(Item):
    def days_allowed(self) -> int: 
        return 14

class Dvd(Item):
    def days_allowed(self) -> int: 
        return 7

class Magazine(Item):
    def days_allowed(self) -> int:
        return 4
