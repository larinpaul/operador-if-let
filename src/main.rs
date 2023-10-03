//// 2023/10/03 // 22:43 //

//// #13 Operador if let

// El operador if let nos va a permitir combinar if y let de una manera menos
// detallada que match para manejar valores que conciden con un patrón
// ignorando el resto.

fn main() {

    let maximo_configurado = Some(7u8);
    // match maximo_configurado {
    //     Some(maximo) => println!("El máximo que se ha configurado es {}", maximo),
    //     _ => (),
    // }
    
    if let Some(maximo) = maximo_configurado {
        println!("El máximo que se ha configurado es {}", maximo);
    }


    // let tiempo  = Tiempo::Segundo;
    // let mut contador = 0;
    // // match tiempo {
    // //     Tiempo::Dia(mes) => println!("Es el mes de {:?}", mes),
    // //     _ => contador += 1, 
    // // }
    // // println!("El valor del contador es {}", contador);

    // if let Tiempo::Dia(mes) = tiempo {
    //     println!("Es un día del mes {:?}", mes);
    // } else {
    //     contador += 1;
    // }
    // println!("El valor del contador es {}", contador);


    let tiempo = Tiempo::Dia(Mes::Abril);
    let mut contador = 0;
    if let Tiempo::Dia(mes) = tiempo {
        println!("Es un día del mes {:?}", mes);
    } else {
        contador += 1;
    }
    println!("El valor del contador es {}", contador);

    
}

#[derive(Debug)]

enum Mes {
    Enero,
    Febrero,
    Marzo,
    Abril,
    Mayo,
    Junio,
    Julio,
    Agosto,
    Septiembre,
    Octubre,
    Noviembre,
    Diciembre,
}

enum Tiempo {
    Segundo,
    Minuto,
    Hora,
    Dia(Mes),
}


