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

func main() {
	name, err := promptForName()
	if err != nil {
		log.Fatal(err)
	}
	if err := savePet(name); err != nil {
		log.Fatal(err)
	}
	fmt.Println(name)
}

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

func savePet(name string) error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return err
	}
	state := petState{
		Name:      name,
		CreatedAt: time.Now().UTC(),
	}
	data, err := json.MarshalIndent(state, "", "\t")
	if err != nil {
		return err
	}
	statePath := filepath.Join(homeDir, ".ped.json")
	return os.WriteFile(statePath, append(data, '\n'), 0644)
}
