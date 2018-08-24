/* Program to compute Pi using Monte Carlo methods */

#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <string.h>
#define SEED 4242424242

int is_in_circle() {
    double x,y,z;

    x = (double)rand() / RAND_MAX;
    y = (double)rand() / RAND_MAX;
    z = hypot(x,y);
    
    if (z<=1){
        return 1;
    } else {
        return 0;
    }
}

int main(){
    int samples = 100000000;
    int i,count; 

    srand(SEED);
    count=0;

    for (i = 0; i < samples; i++) {
        count += is_in_circle();
    }
    double pi = (double)count / samples * 4;
    printf("Pi ~ %g \n", pi);
}

