from enum import Enum
from typing import List, Dict
import json

from pydantic import BaseModel


class Degree(str, Enum):
    BE = "BE"
    MS = "MS"


class JobInfo(BaseModel):
    title: str
    location: str


class Education(BaseModel):
    degree: Degree
    major: str


class Config(BaseModel):
    name: str
    age: int
    weight: float
    education: List[Education]
    experience: Dict[str, JobInfo]


def main():
    config = Config.parse_file("../config/example.json")
    print(config)

    config.age = 32
    config.weight = 82.10

    masters = Education(degree=Degree.MS, major="Mechanical Engineering")
    config.education.append(masters)

    current_job = JobInfo(title="Senior Software Engineer", location="Berlin, Germany")
    config.experience["HERE Technologies"] = current_job

    with open("../config/py-example.json", "w") as f:
        json.dump(config.dict(), f, indent=4)

    print(config)



if __name__ == "__main__":
    main()