

nimbus-core-gen:
	rm -r services/nimbus-core/proto-gen
	buf generate --template services/nimbus-core/generate.yaml

byte-processor-clean:
	rm -r services/byte-processor/src/proto-gen
byte-processor-gen:
	buf generate --template services/byte-processor/generate.yaml
