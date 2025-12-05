from abc import ABC, abstractmethod
from dataclasses import dataclass

@dataclass(frozen=True, slots=True)
class Id:
    value: str

class Item(ABC):
    def __init__(self, id: Id, title: str) -> None:
        self._id, self._title = id, title

    @property
    def id(self) -> Id: 
        return self._id

    @property
    def title(self) -> str: 
        return self._title

    @abstractmethod
    def days_allowed(self) -> int: ...

class Book(Item):
    def days_allowed(self) -> int: 
        return 14

class Dvd(Item):
    def days_allowed(self) -> int: 
        return 7

class Magazine(Item):
    def days_allowed(self) -> int:
        return 4
