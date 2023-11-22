docker build -t lyonsyonii/rust-quest-runner .

case $1 in 
    run)
        docker run -p 3030:3030 --cpus="2" lyonsyonii/rust-quest-runner;;
    push)
        docker push lyonsyonii/rust-quest-runner:latest;;
    *)
        echo "Usage: ./run_docker.sh [run|push]";;
esac