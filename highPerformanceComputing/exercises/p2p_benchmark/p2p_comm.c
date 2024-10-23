#include <stdio.h>
#include <string.h>
#include <mpi.h>
#include <stdlib.h>
#include <math.h> 

int main(void) {
    int comm_sz;  // number of processes
    int my_rank;  // my process rank

    MPI_Init(NULL, NULL);
    MPI_Comm_size(MPI_COMM_WORLD, &comm_sz);
    MPI_Comm_rank(MPI_COMM_WORLD, &my_rank);

    double start_time;
    double stop_time;

    for (int i = 0; i < 20; i++) {
        char c = 'a';
        // int power = std::pow((double)2,(double)i);
        int power = 2 << i;
        int size = power * sizeof(c);
        // printf("size %d\n", size);
        char *v = (char *)malloc(size);

        if (v==NULL) {
            printf("bestemmie vaie");
            exit(1);
        }

        if (my_rank == 0) {
            for (int j = 0; j < power; j++) {
                v[j] = 'c';
            }

            start_time = MPI_Wtime();
            MPI_Send(v, power, MPI_CHAR, 1, 0, MPI_COMM_WORLD);
            MPI_Recv(v, power, MPI_CHAR, 1, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
            stop_time = MPI_Wtime();
            double elapsed_time = stop_time-start_time;
            printf("Byte sent: %d -- Time: %f -- MB/s: %f\n", size, elapsed_time, (power*2/elapsed_time)/1e6);
        } else {
            MPI_Recv(v, power, MPI_CHAR, 0, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
            MPI_Send(v, power, MPI_CHAR, 0, 0, MPI_COMM_WORLD);
        }

        free(v);
    }

    MPI_Finalize();
    return 0;
}