use std::cmp::Ordering;
pub fn generic_types_excercices() {
    // collection_average();
    generic_collection();
}

// Promedio de una colección genérica
/*
* Escribe una función que tome una colección genérica de números y devuelva su promedio.
* Asegúrate de manejar correctamente los tipos numéricos.
*/

fn collection_average() {
    let collection: Vec<f32> = vec![10.0, 7.0, 8.0, 9.0];
    let collection_avrg = collection_average_generic_type(collection);
    println!("{}", collection_avrg);
}

fn collection_average_generic_type<T>(col: Vec<T>) -> f32 where T: Copy + Into<f32> {
    let mut times: u8 = 0;
    let mut total: f32 = 0.0;
    for number in col {
        times += 1;
        total = total + number.into();
    }
    return total / times as f32
}


// Ordenar una colección genérica
/*
* Crea una función que tome una colección genérica de elementos comparables
* y devuelva la colección ordenada en orden ascendente o descendente.
*/

fn generic_collection() {
    let num: Vec<u8> = vec![10, 4, 6, 7, 8, 1, 2];
    println!("{:?}", sort_generic_collection(num));

    let float_num: Vec<f32> = vec![10.0, 4.0, 6.0, 7.0, 8.0, 1.0, 2.0];
    println!("{:?}", sort_generic_collection(float_num));
}

fn sort_generic_collection<T: Copy + PartialOrd>(col: Vec<T>) -> Vec<T> {
    let mut sort_col = col.clone();
    sort_col.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sort_col
}







// Combinar dos colecciones genéricas
/*
Escribe una función que tome dos colecciones genéricas del mismo tipo
* y las combine en una sola colección, manteniendo el orden original de los elementos.
*/
