package db

import (
	"github.com/biswasakashdev/nimbus/services/nimbus-core/internal/config"

	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
)

func GetConnection(appConfig config.Config) *mongo.Client {
	client, err := mongo.Connect(options.Client().
		ApplyURI(appConfig.DBUrl))

	if err != nil {
		panic("")
	}
	return client
}
