docker build -t ghcr.io/lyonsyonii/rust-quest-runner .

case $1 in 
    run)
        docker run -p 3030:3030 --cpus="2" ghcr.io/lyonsyonii/rust-quest-runner;;
    push)
        docker push ghcr.io/lyonsyonii/rust-quest-runner:latest;;
    *)
        echo "Usage: ./run_docker.sh [run|push]";;
esac