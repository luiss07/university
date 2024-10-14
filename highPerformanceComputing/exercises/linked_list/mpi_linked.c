#include <stdio.h>
#include <string.h>
#include <mpi.h>

const int MAX_STRING = 100;

int main(void) {
    int value;
    int comm_sz;  // number of processes
    int my_rank;  // my process rank

    MPI_Init(NULL, NULL);
    MPI_Comm_size(MPI_COMM_WORLD, &comm_sz);
    MPI_Comm_rank(MPI_COMM_WORLD, &my_rank);

    if (my_rank == 0) {
        value = my_rank;
        MPI_Send(&value, 1, MPI_INT, my_rank+1, 0, MPI_COMM_WORLD);
    } else {
        MPI_Recv(&value, 1, MPI_INT, my_rank-1, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
        printf("Im %d got data from %d\n", my_rank, value);
        if (my_rank < comm_sz-1) {
            value = my_rank;
            MPI_Send(&value, 1, MPI_INT, my_rank+1, 0, MPI_COMM_WORLD);
        } 
    }
    printf("Im %d sending data to %d!\n", my_rank, my_rank+1);

    MPI_Finalize();
    return 0;
}