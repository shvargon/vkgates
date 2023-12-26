# Docker build image
```bash
docker build --target=server -t shvargon/vkapi .
```

# Docker copy binary to bin directory
```bash
docker build --target=binaries --output=bin .
```