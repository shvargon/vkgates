DOCKER_USERNAME ?= shvargon
APPLICATION_NAME ?= vkgates

_BUILD_ARGS_TAG ?= latest
_BUILD_ARGS_DOCKERFILE ?= Dockerfile
_BUILD_ARGS_TARGET ?= server
_BUILD_ARGS_BINARYPATH =? bin

_builder:
	docker build --target=${_BUILD_ARGS_TARGET} -t ${DOCKER_USERNAME}/${APPLICATION_NAME}:${_BUILD_ARGS_TAG} -f ${_BUILD_ARGS_DOCKERFILE} .
 
_builder_binary:
	docker build --target=${_BUILD_ARGS_TARGET} -t ${DOCKER_USERNAME}/${APPLICATION_NAME}:${_BUILD_ARGS_TAG}

build:
	$(MAKE) _builder

binary:
