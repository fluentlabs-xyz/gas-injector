
.PHONY: build run test clean
build:
	go build -o build/main main.go
run:
	LD_LIBRARY_PATH="./libs${LD_LIBRARY_PATH:+":$LD_LIBRARY_PATH"}" go run -v main.go
test:
	LD_LIBRARY_PATH="$(PWD)/libs{LD_LIBRARY_PATH:+":$LD_LIBRARY_PATH"}" go test ./...
clean:
	rm -rf build
