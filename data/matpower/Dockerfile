FROM ubuntu:22.04

ENV DEBIAN_FRONTEND = noninteractive

RUN apt-get update && apt-get install -yq --no-install-recommends \
	build-essential git ca-certificates zip curl wget unzip nano \
	octave

ENV OCTAVE_EVAL octave-cli --no-gui --eval

RUN git clone https://github.com/MATPOWER/matpower /workspace/matpower
WORKDIR /workspace/matpower
RUN git checkout 7.1
RUN $OCTAVE_EVAL "install_matpower(1,1,1)"
#RUN $OCTAVE_EVAL "test_mptest(0,1)"
#RUN $OCTAVE_EVAL "test_mips(0,1)"
#RUN $OCTAVE_EVAL "test_matpower(0,1)"

RUN mkdir -p /workspace/spbench
WORKDIR /workspace/spbench

COPY mmwrite.m /workspace/matpower/lib/mmwrite.m
COPY mpc2mtx.m /workspace/matpower/lib/mpc2mtx.m

RUN $OCTAVE_EVAL "mpc2mtx('case_ACTIVSg2000')"
RUN $OCTAVE_EVAL "mpc2mtx('case_ACTIVSg10k')"
RUN $OCTAVE_EVAL "mpc2mtx('case_ACTIVSg25k')"
RUN $OCTAVE_EVAL "mpc2mtx('case_ACTIVSg70k')"

#RUN cd mtx && zip ../mtx.zip *.gz
RUN zip mtx.zip *.mtx

RUN ls -alh