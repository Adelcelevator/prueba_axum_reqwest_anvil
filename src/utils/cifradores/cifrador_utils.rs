use aes_gcm::{Aes256Gcm, Nonce, KeyInit, aead::Aead};
use base64::Engine;
use rand::{Rng, RngExt,};
use sha3::{Digest, Sha3_256};
use log::error;

use crate::utils::env::env_utils::get_variable;

pub fn _cifrar(texto_cifrar: String, nombre_clave:String) -> Result<String,i32> {
    let cipher = match Aes256Gcm::new_from_slice(_get_clave(nombre_clave)?.as_ref()){
        Ok(res)=>res,
        Err(error)=>{
            error!("Error al generar el cifrador: {}",error);
            return Err(0);
        }
    };
    let mut proto_nonce = [0u8;12];
    rand::rng().fill(&mut proto_nonce);

    let nonce = Nonce::from(proto_nonce);
    let ciphertext = match cipher.encrypt(&nonce, texto_cifrar.as_bytes()){
        Ok(res)=>res,
        Err(error)=>{
            error!("Existio un error al cifrar el texto: {}",error);
            return Err(0);
        }
    };

    let mut texto_final = Vec::with_capacity(proto_nonce.len()+ciphertext.len());

    texto_final.extend_from_slice(&proto_nonce);
    texto_final.extend_from_slice(&ciphertext);

    Ok(base64::engine::general_purpose::STANDARD.encode(texto_final))
}

pub fn _descifrar(texto_descifrar: &String, nombre_clave:String) -> Result<String,i32> {
    let cipher = match Aes256Gcm::new_from_slice(_get_clave(nombre_clave)?.as_ref()){
        Ok(res)=>res,
        Err(error)=>{
            error!("Existio  un error al generar el decifrador: {}",error);
            return Err(0);
        }
    };
    let decodificado = match base64::engine::general_purpose::STANDARD
        .decode(texto_descifrar.as_bytes()){
            Ok(des)=>des,
            Err(error)=>{
                error!("Existio  un error al decodificar de base 64: {}",error);
                return Err(0);
            }
        };
    
    if decodificado.len() < 12 {
        error!("El texto decodificado no es el texto esperado: {}",decodificado.len());
        return Err(0);
    }
    let (nonce_bytes, ciphertext_bytes) = decodificado.split_at(12);
    let nonce_array: [u8; 12] = match nonce_bytes.try_into(){
        Ok(res)=>res,
        Err(error)=>{
            error!("No se pudo determinar el nonce: {}",error);
        return Err(0);
        }
    };
    let nonce = Nonce::from(nonce_array); // 96-bits; unique per message
    let plaintext = match cipher
        .decrypt(&nonce, ciphertext_bytes){
            Ok(descifrado)=>descifrado,
            Err(error)=>{
                error!("Existio  un error al descifrar: {}",error);
                return Err(0);
            }
        };
    
    match std::string::String::from_utf8(plaintext){
        Ok(texto)=>Ok(texto),
        Err(error)=>{
            error!("Existio un error al formtear el texto: {}",error);
            Err(0)
        }
    }
}

fn _get_clave(nombre_clave:String)-> Result<sha3::digest::Output<Sha3_256>,i32> {

    let Some(clave_cifrado) = get_variable::<String>(&nombre_clave) else {
        error!("No se pudo detrminar la clave de cifrado.");
        return Err(0);
    };
    let mut hasher = Sha3_256::new();
    hasher.update(clave_cifrado.as_bytes());
    Ok(hasher.finalize())
}

pub fn _random_text(tam:usize)->String{

    let mut key = vec![0u8;tam];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut key);

    base64::engine::general_purpose::STANDARD.encode(key)
}

pub fn _hash_sha3256(texto:&str)->String{
    let mut hasher = Sha3_256::new();
    hasher.update(texto.as_bytes());
    let hasehado = hasher.finalize();
    base64::engine::general_purpose::STANDARD.encode(hasehado)
}

pub fn _generar_pin(longitud: usize) -> String {
    let mut rng = rand::rng();

    (0..longitud)
        .map(|_| {
            let digito: u8 = rng.random_range(0..10);
            char::from(b'0' + digito)
        })
        .collect()
}