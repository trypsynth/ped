package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"time"
)

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
