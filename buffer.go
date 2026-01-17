package main

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
)

type lineBuffer struct {
	lines []string
	index int
}

func newLineBuffer(lines []string) *lineBuffer {
	return &lineBuffer{
		lines: lines,
		index: 0,
	}
}

func (b *lineBuffer) current() string {
	if len(b.lines) == 0 {
		return ""
	}
	return b.lines[b.index]
}

func (b *lineBuffer) move(delta int) bool {
	next := b.index + delta
	if next < 0 || next >= len(b.lines) {
		return false
	}
	b.index = next
	return true
}

func (b *lineBuffer) jump(line int) bool {
	if line <= 0 || line > len(b.lines) {
		return false
	}
	b.index = line - 1
	return true
}

func runBuffer(lines []string, in io.Reader, out io.Writer) {
	buffer := newLineBuffer(lines)
	if len(lines) == 0 {
		fmt.Fprintln(out, "?")
		return
	}
	fmt.Fprintln(out, buffer.current())
	reader := bufio.NewReader(in)
	for {
		line, err := reader.ReadString('\n')
		if err != nil && err != io.EOF {
			fmt.Fprintln(out, "?")
			return
		}
		eof := err == io.EOF
		cmd := strings.TrimSpace(line)
		switch {
		case cmd == "q":
			return
		case cmd == "=":
			fmt.Fprintln(out, len(buffer.lines))
		case cmd == ".=":
			fmt.Fprintln(out, buffer.index+1)
		case cmd == ".":
			fmt.Fprintln(out, buffer.current())
		case cmd == "":
			printMoveResult(out, buffer.move(1), buffer.current())
		case isRun(cmd, '+'):
			printMoveResult(out, buffer.move(len(cmd)), buffer.current())
		case isRun(cmd, '-'):
			printMoveResult(out, buffer.move(-len(cmd)), buffer.current())
		case len(cmd) > 1 && (cmd[0] == '+' || cmd[0] == '-') && isDigits(cmd[1:]):
			steps, _ := strconv.Atoi(cmd[1:])
			if steps == 0 {
				printInvalid(out)
				break
			}
			if cmd[0] == '-' {
				steps = -steps
			}
			printMoveResult(out, buffer.move(steps), buffer.current())
		case isDigits(cmd):
			target, _ := strconv.Atoi(cmd)
			printMoveResult(out, buffer.jump(target), buffer.current())
		default:
			printInvalid(out)
		}
		if eof {
			return
		}
	}
}

func printMoveResult(out io.Writer, ok bool, line string) {
	if !ok {
		printInvalid(out)
		return
	}
	fmt.Fprintln(out, line)
}

func printInvalid(out io.Writer) {
	fmt.Fprintln(out, "?")
}

func isRun(value string, target byte) bool {
	for i := 0; i < len(value); i++ {
		if value[i] != target {
			return false
		}
	}
	return len(value) > 0
}

func isDigits(value string) bool {
	for i := 0; i < len(value); i++ {
		if value[i] < '0' || value[i] > '9' {
			return false
		}
	}
	return len(value) > 0
}

func initialLines(name string) []string {
	return []string{
		fmt.Sprintf("name: %s", name),
		"status: content",
		"mood: curious",
		"last_seen: just now",
	}
}
