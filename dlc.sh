#!/bin/bash
set -e

DOCKER_BUILDKIT=1 docker build \
	--target planner \
	--tag jsalverda/jarvis-wallbox-exporter:dlc-main-planner \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-planner \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
DOCKER_BUILDKIT=1 docker push jsalverda/jarvis-wallbox-exporter:dlc-main-planner

DOCKER_BUILDKIT=1 docker build \
	--target cacher \
	--tag jsalverda/jarvis-wallbox-exporter:dlc-main-cacher \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-planner \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-cacher \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
DOCKER_BUILDKIT=1 docker push jsalverda/jarvis-wallbox-exporter:dlc-main-cacher

DOCKER_BUILDKIT=1 docker build \
	--target builder \
	--tag jsalverda/jarvis-wallbox-exporter:dlc-main-builder \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-planner \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-cacher \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-builder \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
DOCKER_BUILDKIT=1 docker push jsalverda/jarvis-wallbox-exporter:dlc-main-builder

DOCKER_BUILDKIT=1 docker build \
	--target runtime \
	--tag jsalverda/jarvis-wallbox-exporter:dlc-main-runtime \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-planner \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-cacher \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-builder \
	--cache-from jsalverda/jarvis-wallbox-exporter:dlc-main-runtime \
	--build-arg BUILDKIT_INLINE_CACHE=1 .
DOCKER_BUILDKIT=1 docker push jsalverda/jarvis-wallbox-exporter:dlc-main-runtime