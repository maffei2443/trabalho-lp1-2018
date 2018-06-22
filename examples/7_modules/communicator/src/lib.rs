// lib.rs é o ARQUIVO NO QUAL O CRGO BUSCA OS MÓDULOS
// Logo, serve basicamente para exportar/chamar outros módulos.
// Tipo a main

pub mod client;


mod network;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
