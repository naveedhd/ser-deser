import std/json
import std/tables


type
  Degree = enum
    BE, MS

  Education = object
    degree: Degree
    major: string

  JobInfo = object
    title: string
    location: string

  Config = object
    name: string
    age: int
    weight: float
    education: seq[Education]
    experience: Table[string, JobInfo]


let config_json = parseFile("../config/example.json")
var config = to(config_json, Config)
echo config

config.age = 32
config.weight = 82.10

let masters = Education(degree: Degree.MS, major: "Mechanical Engineering")
config.education.add(masters)

let current_job = JobInfo(title: "Senior Software Engineer", location: "Berlin, Germany")
config.experience["HERE Technologies"] = current_job

echo config

let new_config_json = %* config
echo new_config_json

writeFile("../config/nim-example.json", new_config_json.pretty(4))
