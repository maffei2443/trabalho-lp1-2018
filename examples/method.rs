#[derive(Debug)]  // Permite que {:?} seja habilitado para as structs que nao possuem implementacao propria "dele"
#[warn(dead_code)] // Nao dah warning para cohdigo que nunca eh executado
struct Name {
    field: i32,
}

impl Name {
    fn value(&self) -> i32 {
        self.field
    }
    fn erase(self) {

    }
}

impl Name {
    fn minus_value(&self) -> i32 {
        return self.value() * (-1);
    }
}

fn main(){
    let name = Name{field: 90};
    println!("Valor : {}", name.value() );
    // name.erase();
    println!("Debug : {:?}", name );
    println!("Debug bonito : {:#?}", name );
}