package main

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
)

func readStuff(scanner *bufio.Scanner) {
	for scanner.Scan() {
		fmt.Println("Performed Scan")
		fmt.Println(scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "reading standard input:", err)
	}
}

func main() {
	cmd := exec.Command("cargo", "run")
	// in, err := cmd.StdinPipe()
	cmd.Stdin = os.Stdin
	out, err := cmd.StdoutPipe()

	err = cmd.Start()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to start err=%v", err)
		os.Exit(1)
	}

	scanner := bufio.NewScanner(out)
	fmt.Println("Scanner created")

	defer cmd.Wait()

	go readStuff(scanner)
}
