# Threads

+ std::sync::mpsc::channel() **(multiple producers, single consumer)**
	* Cria um *canal* de comunicação;
	* retorna uma tupla (transmissor, receptor)
		+ receptor tem que ser possuído por 1 única thread
		+ várias threads podem usar o transmissor

