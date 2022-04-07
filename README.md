# The UConn AMBER (Adaptive Morphing and Balanced Exploratory Rover) Project

A group of students at the University of Connecticut were selected as finalists in the annual NASA Big Idea Challenge ([see our submission](https://bigidea.nianet.org/wp-content/uploads/2022-BIG-Idea-Challenge-Finalist-Team-Synopses.pdf) or the [video](https://youtu.be/4zF1PQumCn8))

# Project setup
1. Download Git - [Linux](https://git-scm.com/download/linux), [macOS](https://git-scm.com/download/mac), [Windows](https://git-scm.com/download/win)
2. Download and Setup Docker
   - [macOS](https://docs.docker.com/desktop/mac/install/) - Install Docker Desktop
   - [Windows](https://docs.docker.com/desktop/windows/install/) - Install Docker Desktop
   - [Linux](https://docs.docker.com/engine/install/)
 #### On the terminal or command prompt do the following:
3. Navigate to a directory where you want the project to be stored.
4. Clone the GitHub Repository using the command - 
```
git clone https://github.com/AMBER-UConn/amber_robot
```
5. The docker configuration files lie in the docker folder of the amber_robot project. Anytime you need to access these files to run a command, change your current directory to the docker directory.
6. Before creating and starting any docker services, you must start Docker Daemon. In Windows or macOS, you can manually do it by starting the Docker Desktop app. The state should transition to "Running" after a few seconds. In Linux you can do it [manually](https://docs.docker.com/config/daemon/systemd/) or [automatically](https://docs.docker.com/engine/install/linux-postinstall/).
7. Return to the terminal and use the command below to create and start services.
```
docker-compose --profile no-gui up
``` 

8. After a while of downloading and compiling your containers will be up and running. You should see messages sent across ROS Services:
   
```
ros_listener  | data: "yoooooooooooooooooooooooooo 1"
ros_listener  | ---
ros_listener  | data: "yoooooooooooooooooooooooooo 2"
ros_listener  | ---
ros_listener  | data: "yoooooooooooooooooooooooooo 3"
ros_listener  | ---
```

   ## Congratulations! Set up complete!
\
   Note: To stop a running container, it would be preferable if the process could shutdown smoothly instead of abruptly disconnecting users and corrupting files. Press ctrl+c and wait 10-12 seconds for the containers to stop.

   Note: To restart the containers use the command `docker-compose up`.
   
\
To open a bash terminal (Linux terminal) in the rust_ros container, start the containers using the command `docker-compose up` in one tab of the terminal, and in another tab, start a bash terminal using the command:
 ```
 docker exec -it rust_ros bash
 ```