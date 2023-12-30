# docker build --tag ${DOCKER_USERNAME}/${APPLICATION_NAME}
DOCKER_USERNAME ?= shvargon
APPLICATION_NAME ?= vkgates

# Docker image tag
_BUILD_ARGS_TAG ?= latest
_BUILD_ARGS_DOCKERFILE ?= Dockerfile
# Path to save compile binary
_BUILD_ARGS_BINARYPATH ?= target/docker
# destination features
_BUILD_ARGS_FEATURES ?= default

all: build binary

build:
	docker build --target=server -t ${DOCKER_USERNAME}/${APPLICATION_NAME}:${_BUILD_ARGS_TAG} \
		-f ${_BUILD_ARGS_DOCKERFILE} --build-arg="FEATURES=${_BUILD_ARGS_FEATURES}" .

binary:
	docker build --target=binaries --output=${_BUILD_ARGS_BINARYPATH} --build-arg="FEATURES=${_BUILD_ARGS_FEATURES}" .

clean:
	rm -rf target
	docker rmi ${DOCKER_USERNAME}/${APPLICATION_NAME}