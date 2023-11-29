#!/usr/bin/env bash
# This is an example/test engine that will play a set of moves

read uai # UAI
if [ "$uai" != "uai" ]; then
    echo "Error: Expected 'uai' but got '$uai'"
    exit 1
fi
echo "id name Example Engine"
echo "id author BlueCore"
echo "uaiok"

read position
pos_array=($position)
if [ "${pos_array[0]}" != "position" ]; then
    echo "Error: Expected 'position' but got '${pos_array[0]}'"
    exit 1
fi
read go
go_array=($go)
if [ "${go_array[0]}" != "go" ]; then
    echo "Error: Expected 'go' but got '${go_array[0]}'"
    exit 1
fi
echo "bestmove a1a2"
read position
pos_array=($position)
if [ "${pos_array[0]}" != "position" ]; then
    echo "Error: Expected 'position' but got '${pos_array[0]}'"
    exit 1
fi
read go
go_array=($go)
if [ "${go_array[0]}" != "go" ]; then
    echo "Error: Expected 'go' but got '${go_array[0]}'"
    exit 1
fi
echo "bestmove a1c1"
read position
pos_array=($position)
if [ "${pos_array[0]}" != "position" ]; then
    echo "Error: Expected 'position' but got '${pos_array[0]}'"
    exit 1
fi
read go
go_array=($go)
if [ "${go_array[0]}" != "go" ]; then
    echo "Error: Expected 'go' but got '${go_array[0]}'"
    exit 1
fi
echo "bestmove a2a3"
read position
pos_array=($position)
if [ "${pos_array[0]}" != "position" ]; then
    echo "Error: Expected 'position' but got '${pos_array[0]}'"
    exit 1
fi
read go
go_array=($go)
if [ "${go_array[0]}" != "go" ]; then
    echo "Error: Expected 'go' but got '${go_array[0]}'"
    exit 1
fi
echo "bestmove a3a4"
