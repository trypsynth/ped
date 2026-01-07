package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	name, err := promptForName()
	if err != nil {
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
	line = strings.TrimSpace(line)
	return line, nil
}
