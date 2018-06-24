#include <stdio.h>
#include 

fn main() {
	// double complex z = 1.0 + 3.0 * I;
	// printf("z = %.2f + %.2f\n", rz(z), iz(z));
	// printf("modelo(z)^2 = %.2lf\n", msc(z));
	// printf("modelo(z) = %.2lf\n", sqrt(msc(z)) );
	// printf("conj(z) = %.2lf + %.2lf\n", rz(conj(z)) , iz(conj(z)));
	if(argc < 3) {
		printf("./program tam_matriz limit_iteracoes\n");
		return 1;
	}
	
	let LIM = atoi(argv[1]);
	let MAX_ITER = atoi(argv[2]);
	let OFFSET = LIM;
	if (LIM > 1000 || LIM < 0 || MAX_ITER > 100000 || MAX_ITER < 0) {
		printf("O tam_matriz digitado deve ser inteiro e entre 0 e 1000.\n");
		printf("O limit_iteracoes digitado deve ser inteiro e entre 0 e 5000.\n");
		return 2;
	}
	printf("%d\n", LIM);
	char mat[2*LIM][2*LIM];
	for(int i = 0-OFFSET; i < LIM; i++) {
		for (int j = 0-OFFSET; j < LIM; ++j)	{
			// double complex c = i/20.0 + (j/20.0 * I);
			double complex c = j/20.0 + (i/20.0 * I);
			double complex z = 0.0 + c;
			int k;
			for(k = 0; k < MAX_ITER && msc(z) < 4; k++) {
				z = z*z + c;
			}
			if(k == MAX_ITER) 
				mat[i+OFFSET][j+OFFSET] = 'x';
			else
				mat[i+OFFSET][j+OFFSET] = '.';
		}		
	}
	for(int i=0; i < 2*LIM; i++){
		printf("%2d - ", i);
		for(int j=0; j < 2*LIM; j++)
			printf("%c", mat[i][j]);
		printf("\n");
	}
}