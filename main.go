package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
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

func main() {
	state, err := loadPet()
	if err == nil && state.Name != "" {
		runBuffer(initialLines(state.Name), os.Stdin, os.Stdout)
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
	runBuffer(initialLines(name), os.Stdin, os.Stdout)
}
