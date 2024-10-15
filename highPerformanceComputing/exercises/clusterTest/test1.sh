#!/bin/bash
#PBS -l select=1:ncpus=1:mem=2gb
# set max execution time
#PBS -l walltime=0:05:00
# define the queue
#PBS -q short_cpuQ
# sleep 120
echo "hostname: " && hostname

module load mpich-3.2
mpicc -g -Wall -o mpi_hello ./mpi_hello.c
mpiexec -n 6 mpi_hello
# or mpirun.actual -n <core_number> ./mpi_hello.c