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
		cmd := strings.TrimSpace(line)
		if cmd == "q" {
			return
		}
		if cmd == "=" {
			fmt.Fprintln(out, len(buffer.lines))
			if err == io.EOF {
				return
			}
			continue
		}
		if cmd == ".=" {
			fmt.Fprintln(out, buffer.index+1)
			if err == io.EOF {
				return
			}
			continue
		}
		if cmd == "." {
			fmt.Fprintln(out, buffer.current())
			if err == io.EOF {
				return
			}
			continue
		}
		if cmd == "" {
			if !buffer.move(1) {
				fmt.Fprintln(out, "?")
			} else {
				fmt.Fprintln(out, buffer.current())
			}
			if err == io.EOF {
				return
			}
			continue
		}
		if isRun(cmd, '+') {
			if !buffer.move(len(cmd)) {
				fmt.Fprintln(out, "?")
			} else {
				fmt.Fprintln(out, buffer.current())
			}
			if err == io.EOF {
				return
			}
			continue
		}
		if isRun(cmd, '-') {
			if !buffer.move(-len(cmd)) {
				fmt.Fprintln(out, "?")
			} else {
				fmt.Fprintln(out, buffer.current())
			}
			if err == io.EOF {
				return
			}
			continue
		}
		if len(cmd) > 1 && (cmd[0] == '+' || cmd[0] == '-') && isDigits(cmd[1:]) {
			steps, _ := strconv.Atoi(cmd[1:])
			if cmd[0] == '-' {
				steps = -steps
			}
			if !buffer.move(steps) {
				fmt.Fprintln(out, "?")
			} else {
				fmt.Fprintln(out, buffer.current())
			}
			if err == io.EOF {
				return
			}
			continue
		}
		if isDigits(cmd) {
			target, _ := strconv.Atoi(cmd)
			if !buffer.jump(target) {
				fmt.Fprintln(out, "?")
			} else {
				fmt.Fprintln(out, buffer.current())
			}
			if err == io.EOF {
				return
			}
			continue
		}
		fmt.Fprintln(out, "?")
		if err == io.EOF {
			return
		}
	}
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
