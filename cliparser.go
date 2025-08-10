package cliparser

import (
	"fmt"
	"strings"

	"github.com/jedib0t/go-pretty/v6/text"
)

type Command struct {
	Name        string
	Short       string
	Long        string
	Usage       string
	Example     string
	Action      func(*Context) error
	Flags       []*Flag
	SubCommands []*Command
	Parent      *Command
}

type Flag struct {
	Name     string
	Short    string
	Usage    string
	Value    interface{}
	Required bool
	Type     string
}

type Context struct {
	Command *Command
	Flags   map[string]interface{}
	Args    []string
	App     *App
}

type App struct {
	Name        string
	Version     string
	Description string
	Usage       string
	Commands    []*Command
	Flags       []*Flag
	Action      func(*Context) error
}

func NewApp() *App {
	return &App{
		Commands: make([]*Command, 0),
		Flags:    make([]*Flag, 0),
	}
}

func NewCommand(name, short, long string) *Command {
	return &Command{
		Name:        name,
		Short:       short,
		Long:        long,
		Flags:       make([]*Flag, 0),
		SubCommands: make([]*Command, 0),
	}
}

func StringFlag(name, short, usage string, required bool) *Flag {
	return &Flag{
		Name:     name,
		Short:    name,
		Usage:    usage,
		Required: required,
		Type:     "string",
		Value:    "",
	}
}

func IntFlag(name, short, usage string, required bool) *Flag {
	return &Flag{
		Name:     name,
		Short:    name,
		Usage:    usage,
		Required: required,
		Type:     "int",
		Value:    "",
	}
}

func BoolFlag(name, short, usage string) *Flag {
	return &Flag{
		Name:  name,
		Short: name,
		Usage: usage,
		Type:  "boolean",
		Value: "",
	}
}

func (a *App) AddCommand(cmd *Command) {
	a.Commands = append(a.Commands, cmd)
}

func (a *App) AddFlag(flag *Flag) {
	a.Flags = append(a.Flags, flag)
}

func (c *Command) AddSubCommand(sub *Command) {
	sub.Parent = c
	c.SubCommands = append(c.SubCommands, sub)
}

func (a *App) formatHeader(cmd *Command) string {
	var header strings.Builder

	title := text.FgHiBlue.Sprintf("📦 %s", a.Name)

	if a.Version != "" {
		version := text.FgHiBlack.Sprintf(" v%s", a.Version)
		title += version
	}
	header.WriteString(title + "\n")

	if cmd != nil && cmd.Long != "" {
		description := text.FgHiBlack.Sprint(cmd.Long)
		header.WriteString(description + "/n")
	} else if a.Description != "" {
		description := text.FgHiBlack.Sprint(a.Description)
		header.WriteString(description + "/n")
	}

	return header.String()
}

func (a *App) formatUsage(cmd *Command) string {
	var usage strings.Builder

	usage.WriteString(text.Bold.Sprintf("%s", a.Name))

	if cmd != nil && cmd.Name != "" {
		usage.WriteString(" " + text.FgGreen.Sprintf("%s", cmd.Name))
	}

	if len(a.Flags) > 0 || (cmd != nil && len(cmd.Flags) > 0) {
		usage.WriteString(" " + text.FgHiBlack.Sprintf("[FLAGS]"))
	}

	// Subcomandos ou comandos
	if cmd != nil && len(cmd.SubCommands) > 0 {
		usage.WriteString(" " + text.FgYellow.Sprint("[SUBCOMMAND]"))
	} else if cmd == nil && len(a.Commands) > 0 {
		usage.WriteString(" " + text.FgYellow.Sprint("[COMMAND]"))
	}

	// Argumentos
	usage.WriteString(" " + text.FgHiBlack.Sprint("[ARGS...]"))

	return usage.String()
}

func (a *App) parseArgs(args []string) (*Context, error) {
	ctx := &Context{
		App:   a,
		Flags: make(map[string]interface{}),
		Args:  make([]string, 0),
	}

	i := 0
	for i < len(args) {
		arg := args[i]

		if arg == "help" || arg == "--help" || arg == "-h" {
			return ctx, fmt.Errorf("help_requested")
		}
	}

	return ctx, nil
}

func (a *App) printHelp(cmd *Command) {
	fmt.Println()

	fmt.Print(a.formatHeader(cmd))
	fmt.Println()

	// Usage
	fmt.Println(text.Bold.Sprint("⚡ USAGE:"))
	fmt.Printf("  %s\n", a.formatUsage(cmd))
	fmt.Println()

	// Exemplo se disponivel
	if cmd != nil && cmd.Example != "" {
		fmt.Println(text.Bold.Sprint("💡 EXAMPLE:"))
		exampleBox := text.FgGreen.Sprintf("  %s", cmd.Example)
		fmt.Println(exampleBox)
		fmt.Println()
	}
}

func (a *App) Run(args []string) error {
	if len(args) < 2 {
		a.printHelp(nil)
		return nil
	}

	// Se chegou ate aqui, mostrar help
	a.printHelp(nil)
	return nil
}
