use crate::{models::cliente::Cliente, tela::{ler, operacoes_basicas::limpar_tela, servico_cliente::{alterar_clientes, excluir_clientes, incluir_cliente, listar_clientes}}};

pub fn mostrar_menu(clientes: &mut Vec<Cliente>) {

    loop {

        limpar_tela();

        println!("=== Menu Principal ===");
        println!("1. Cadastrar Cliente");
        println!("2. Alterar Cliente");
        println!("3. Excluir Clientes");
        println!("4. Listar Clientes");
        println!("0. Sair");

        let opcao: usize = ler::ler_dados_id();
        limpar_tela();
        match opcao {
            1 => {
                incluir_cliente(clientes);
            }
            2 => {
                // Lógica para alterar cliente
                alterar_clientes(clientes);
            }
            3 => {
                excluir_clientes(clientes);
            }
            4 => {
                // Lógica para listar clientes
                listar_clientes(clientes);
            }
            0 => {
                println!("Saindo do programa...");
                return;
            }
            _ => {
                println!("Opção inválida. Tente novamente.");
            }
          }
          
          //println!("Pressione Enter para continuar...");
          //ler::ler_dados(); // Pausa para o usuário pressionar Enter antes de continuar 

          //esperar(2);
    }

}   