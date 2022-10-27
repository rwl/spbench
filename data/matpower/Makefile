IMAGE_TAG = ghcr.io/rwl/spbench/matpower:latest

all: build mtx.zip
	unzip mtx.zip


.PHONY: build
build:
	docker build -t $(IMAGE_TAG) .


mtx.zip:
	docker run -it --rm -d $(IMAGE_TAG)
	CONTAINER_ID=$$(docker ps -alq) && \
	docker cp $$CONTAINER_ID:/workspace/spbench/mtx.zip . && \
	docker stop $$CONTAINER_ID

clean:
	rm -f mtx.zip
	rm -f *.mtx
