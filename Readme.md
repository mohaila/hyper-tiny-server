# Tiny Rust Server using Hyper
## Steps to build the project
- Create a Rust musl based Docker image using Alpine Linux (rust 1.34 for Alpine 3.10 or rust 1.37 for Alpine Edge).<br/>
This step is useful is you use Windows or macOS, and want to build Linux binaries for Docker
```bash
# use sh rust-musl.sh
$ docker build -t rust:musl -f Dockerfile.rust-musl .c
$ docker image ls 
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
rust                musl                7216ade2d15b        12 hours ago        425MB
```
- build the project using the previous Docker image
```bash
# use sh musl-buil.sh
$ docker run -it -v $PWD:/build rust:musl
```
The Docker image will use build-project.sh to build the project.
- After the project build, you can create a Docker image for the tiny server
``` bash
$ docker build -t hyper-tiny-server:v1 .
Sending build context to Docker daemon  1.747MB
Step 1/4 : FROM scratch
 ---> 
Step 2/4 : ADD target/x86_64-alpine-linux-musl/release/hyper-tiny-server /hyper-tiny-server
 ---> 1ebe22c4d60c
Step 3/4 : EXPOSE 8080
 ---> Running in edd431b22a97
Removing intermediate container edd431b22a97
 ---> 0b9820a04b57
Step 4/4 : CMD ["/hyper-tiny-server"]
 ---> Running in 13d208b2257a
Removing intermediate container 13d208b2257a
 ---> 441be538dbe7
Successfully built 441be538dbe7
Successfully tagged hyper-tiny-server:v1
$ docker image ls
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
hyper-tiny-server   v1                  441be538dbe7        22 minutes ago      1.74MB
rust                musl                7216ade2d15b        12 hours ago        425MB
alpine              3.10                961769676411        4 weeks ago         5.58MB
```
- Run the tiny server using Docker