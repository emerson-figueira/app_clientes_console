use std::{thread::sleep, time::Duration};

pub fn limpar_tela(){
    clearscreen::clear().expect("Falha ao limpar tela")
}

pub fn esperar(tempo: u64){
    sleep(Duration::from_secs(tempo));
}