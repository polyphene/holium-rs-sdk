#!/bin/bash

# A script to update the references for particular tests. The idea is
# that you do a run, which will generate files in the build directory
# containing the (normalized) actual output of the compiler. This
# script will then copy that output and replace the "expected output"
# files. You can then commit the changes.
#
# If you find yourself manually editing a foo.stderr file, you're
# doing it wrong.

MYDIR=$(dirname $0)

BUILD_DIR="../../../target/tests/holium-macro"

while [[ "$1" != "" ]]; do
    STDERR_NAME="${1/%.rs/.stderr}"
    STDOUT_NAME="${1/%.rs/.stdout}"
    shift
    if [ -f $BUILD_DIR/$STDOUT_NAME ] && \
           ! (diff $BUILD_DIR/$STDOUT_NAME $MYDIR/$STDOUT_NAME >& /dev/null); then
        echo updating $MYDIR/$STDOUT_NAME
        cp $BUILD_DIR/$STDOUT_NAME $MYDIR/$STDOUT_NAME
    fi
    if [ -f $BUILD_DIR/$STDERR_NAME ] && \
           ! (diff $BUILD_DIR/$STDERR_NAME $MYDIR/$STDERR_NAME >& /dev/null); then
        echo updating $MYDIR/$STDERR_NAME
        cp $BUILD_DIR/$STDERR_NAME $MYDIR/$STDERR_NAME
    fi
done


