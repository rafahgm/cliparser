package cliparser_test

import (
	"testing"

	"github.com/rafahgm/cliparser"
)

func TestNewApp(t *testing.T) {
	app := cliparser.NewApp()

	if app == nil {
		t.Fatal("NewApp() deveria retornar uma aplicação válida")
	}

	if app.Commands == nil {
		t.Error("Commands deveria ser inicializado")
	}

	if app.Flags == nil {
		t.Error("Flags deveria ser inicializado")
	}
}

func TestNewCommand(t *testing.T) {
	cmd := cliparser.NewCommand("test", "Comando de teste", "Descrição longa do comando")

	if cmd.Name != "test" {
		t.Errorf("Nome esperado 'test', obtido '%s'", cmd.Name)
	}

	if cmd.Short != "Comando de teste" {
		t.Errorf("Short esperado 'Comando de teste', obitido '%s'", cmd.Short)
	}
}
