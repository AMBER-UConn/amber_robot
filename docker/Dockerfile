FROM rust:1.59

RUN apt-get update

#RUN apt-get update && apt-get install -y --no-install-recommends build-essential libglfw3-dev libglew-dev pkg-config libxrandr-dev libxi-dev

#RUN apt-get install x11vnc xvfb -y

#RUN apt-get install libeigen3-dev libboost-all-dev freeglut3-dev -y
RUN apt-get install libglfw3-dev libxinerama-dev libxcursor-dev libxi-dev -y


#libsm6 libxext6  -y

#RUN apt-get install xorg gdm3 -y

WORKDIR /amber-robot

CMD ["bash", "compilerun.sh"]