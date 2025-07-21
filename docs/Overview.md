# ATN

This is an automated train network, designed to handle train systems. It can monitor traffic, activate and deactivate lines, stops, redirect trains all while being efficient and securing the safety of passengers.

## Scheduler

The Scheduler is the core of the ATN.
1. It manages traffic and makes sure that trains are always on time.
2. It makes sure that the distances between trains are always safe in case of an emergency.
3. It tries to minimize the average time it takes a train to go to each stop.

## System

The System has all information about each Train, Line and Stop, all of which are provided to the Scheduler - as needed.

## Controller

TODO

## Train

A train, which has all necessary information that can be fed into the Scheduler, or manually handled using the Controller.

## Line

A train line, which has all necessary information that can be fed into the Scheduler, or manually handled using the Controller.

## Stop

A train stop, which has all necessary information that can be fed into the Scheduler, or manually handled using the Controller.
