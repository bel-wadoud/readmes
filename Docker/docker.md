
# Basics commands and container management.
### runs a docker image
```
docker run <image-name>
```
#### properties
```
docker run
    --interactive ==> creates a shell inside the created container
    --rm ==> removes the image directly after running it.
    --tty ==> gives the user a real shell simulator.
    --name <container-name> ==> gives a name to the container.
    --it ==> interactive + tty
```

### lists all available images.
```
docker ps 
    -a ==> for all (inactive)
```

### starts a suspended container in ur device.
```
docker start <container-name>
```

### stops a cointainer.
```
docker stop <container-name>
```

### attaches to a running container's shell.
```
docker attach <container-name>
```

# docker volumes
### create a new volume
```
docker volume create <volume-name>
```

### runs a container with volumes.
```
docker run -it --mount source=<volume-name>,destination=/path/to/file <container-name>
```
**Note:** docker automatically creates the /path/to/file

### Example: saving postgress database into a volume.
```
docker volume create pgdata
docker run -d \ ==> for detach
    --name postgress-db
    -e POSTGRESS_PASSWORD modpass \
    -v pgadmin:/var/lib/postgresql/data \
    -p 5432:5432 \
    postgres
```
