package main

import (
	"encoding/json"
	"fmt"
	"os"
)

type Education struct {
	Degree string `json:"degree"`
	Major  string `json:"major"`
}

type JobInfo struct {
	Title    string `json:"title"`
	Location string `json:"location"`
}

type Config struct {
	Name       string             `json:"name"`
	Age        int                `json:"age"`
	Weight     float32            `json:"weight"`
	Education  []Education        `json:"education"`
	Experience map[string]JobInfo `json:"experience"`
}

func main() {
	config_str, _ := os.ReadFile("../config/example.json")

	var config Config
	json.Unmarshal(config_str, &config)

	fmt.Println(config)

	config.Age = 32
	config.Weight = 82.1

	masters := Education{Degree: "MS", Major: "Mechanical Engineering"}
	config.Education = append(config.Education, masters)

	current_job := JobInfo{Title: "Senior Software Engineer", Location: "Berlin, Germany"}
	config.Experience["HERE Technologies"] = current_job

	fmt.Println(config)

	data, _ := json.MarshalIndent(config, "", "    ")

	_ = os.WriteFile("../config/go-example.json", data, 0644)
}
