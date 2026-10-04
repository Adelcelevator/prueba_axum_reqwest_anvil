use std::{env, str::FromStr};
use log::error;

pub fn get_variable<T: FromStr>(variable:&str)->Option<T>{

    let busqueda = env::var(variable);
    let encontrado = match busqueda {
        Ok(res)=> res,
        Err(error)=>{
            error!("Existio un error al buscar la variable {}: {}",variable,error);
            return None;
        }
    };

    let convertido = encontrado.parse::<T>();

    match convertido {
        Ok(dato)=>Some(dato),
        Err(_)=>{
            error!("Existio un error al convertir la variable.");
            None
        }
    }

}