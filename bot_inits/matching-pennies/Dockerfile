FROM ubuntu:14.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y autoconf \
                                                                        automake \
                                                                        build-essential \
                                                                        curl \
                                                                        git \
                                                                        libkrb5-dev \
                                                                        libtool \
                                                                        pkg-config \
                                                                        unzip \
                                                                        vim
 
RUN cd /tmp && git clone git://github.com/jedisct1/libsodium.git && cd libsodium && git checkout e2a30a && ./autogen.sh && ./configure && make check && make install && ldconfig
RUN cd /tmp && git clone --depth 1 git://github.com/zeromq/libzmq.git && cd libzmq && ./autogen.sh && ./configure && make
# RUN cd /tmp/libzmq && make check
RUN cd /tmp/libzmq && make install && ldconfig
RUN rm /tmp/* -rf

RUN curl -s https://static.rust-lang.org/rustup.sh > /tmp/rustup.sh
RUN chmod +x /tmp/rustup.sh
RUN cd /tmp && ./rustup.sh --yes

ADD . /usr/local/mpb

RUN cd /usr/local/mpb && cargo build

RUN cp /usr/local/mpb/target/debug/matching-pennies-bot /usr/bin
RUN chmod +x /usr/bin/matching-pennies-bot

CMD matching-pennies-bot 

EXPOSE 5555 
