use std::cell::OnceCell;

struct ConexionBaseDeDatos {
    session_id: String,
    hash: String,
}

impl ConexionBaseDeDatos {
    fn ejecutar_query(&self, query: &str) -> String {
        return format!(
            "Query Ejecutado con el session_id {}, hash {}, query: {}",
            self.session_id.as_str(),
            self.hash.as_str(),
            query
        );
    }
}

fn ejecutando_un_query(cell: &OnceCell<ConexionBaseDeDatos>, query: &str) {
    // En esta funcion vamos a intentar reinicializar nuestra referencia
    // de OnceCell
    let conexion: &ConexionBaseDeDatos = cell.get_or_init(|| ConexionBaseDeDatos {
        session_id: "cambiara desde otra funcion?".to_string(),
        hash: "cambiara desde otra funcion?".to_string(),
    });
    // Aca "ejecutamos el query" el cual simplemente imprimira el session_id
    // hash los cuales no debieron haber cambiado, tambien imprime el query
    println!("{}", conexion.ejecutar_query(query));
}

fn main() {
    // Aca creamos una nueva celda de memoria, pero unicamente
    // se inicializa una vez, luego mantendra el mismo valor
    // sin cambiarse ni hacer copy o reescribir, lo cual es optimio
    // a nivel de memoria.
    let cell = OnceCell::new();

    // Simulando que abrimos una conexion, es importante destacar
    // que el metodo get_or_init, crea una nueva instancia si no se ha hecho antes
    // pero si ya la instancia habia sido creada, devolvera siempre el mismo
    // objeto pero como una referencia &
    // tambien es importante mencionar que no regresa un &mut por lo que luego
    // no se pueden cambiar los valores, los escribe solo una vez
    let conexion: &ConexionBaseDeDatos = cell.get_or_init(|| ConexionBaseDeDatos {
        session_id: "123456789".to_string(),
        hash: "abcde1234".to_string(),
    });

    println!(
        "Session id: {} - Hash: {}",
        conexion.session_id, conexion.hash
    );

    // Aca volvemos a pedir la referencia con get_or_init
    // Como OnceCell ya habia iniciliazado una vez la referencia
    // A ConexionBaseDeDatos, esta no deberia de cambiar sus valores
    // Aun si se pasan nuevos parametros para session_id y hash
    let conexion2: &ConexionBaseDeDatos = cell.get_or_init(|| ConexionBaseDeDatos {
        session_id: "cambiara?".to_string(),
        hash: "cambiara?".to_string(),
    });

    // Aca vamos a verificar si los valores cambiaron
    println!(
        "Nuevo Print Desde main, ha cambiado? Session id: {} - Hash: {}",
        conexion2.session_id, conexion2.hash
    );

    ejecutando_un_query(&cell, "SELECT * FROM mitabla");
    ejecutando_un_query(&cell, "SELECT * FROM usuarios");
}
