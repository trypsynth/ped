package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"
)

func promptForName() (string, error) {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Name: ")
	line, err := reader.ReadString('\n')
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(line), nil
}

type petState struct {
	Name      string    `json:"name"`
	CreatedAt time.Time `json:"created_at"`
}

func loadPet() (petState, error) {
	statePath, err := petStatePath()
	if err != nil {
		return petState{}, err
	}
	data, err := os.ReadFile(statePath)
	if err != nil {
		return petState{}, err
	}
	var state petState
	if err := json.Unmarshal(data, &state); err != nil {
		return petState{}, err
	}
	return state, nil
}

func savePet(name string) error {
	state := petState{
		Name:      name,
		CreatedAt: time.Now().UTC(),
	}
	data, err := json.MarshalIndent(state, "", "  ")
	if err != nil {
		return err
	}
	statePath, err := petStatePath()
	if err != nil {
		return err
	}
	return os.WriteFile(statePath, append(data, '\n'), 0644)
}

func petStatePath() (string, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(homeDir, ".ped.json"), nil
}

func main() {
	state, err := loadPet()
	if err == nil && state.Name != "" {
		fmt.Println(state.Name)
		return
	}
	if err != nil && !os.IsNotExist(err) {
		log.Fatal(err)
	}
	name, err := promptForName()
	if err != nil {
		log.Fatal(err)
	}
	if err := savePet(name); err != nil {
		log.Fatal(err)
	}
	fmt.Println(name)
}
