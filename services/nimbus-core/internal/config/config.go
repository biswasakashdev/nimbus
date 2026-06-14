package config

import (
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	AppName string
	DBUrl   string
}

func Load() Config {
	godotenv.Load()

	return Config{
		AppName: os.Getenv("APP_NAME"),
		DBUrl:   os.Getenv("DB_URL"),
	}
}
