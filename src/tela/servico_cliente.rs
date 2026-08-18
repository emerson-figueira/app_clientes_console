use crate::{models::cliente::Cliente, tela::{ler::{ler_dados, ler_dados_id}, operacoes_basicas::{esperar, limpar_tela}}};


pub fn incluir_cliente(clientes: &mut Vec<Cliente>) {

    limpar_tela();

    let mut cliente: Cliente = Cliente::default();

    cliente.id = clientes.len()+1; 

    digitar_dados_do_cliente(&mut cliente);

    clientes.push(cliente);

    limpar_tela();
    println!("Cliente cadastrado com sucesso!");
    esperar(1);
}

fn digitar_dados_do_cliente(cliente: &mut Cliente)  {
    
    println!("Digite o Nome");
    cliente.nome = ler_dados();
    println!("Digite o CPF:");
    cliente.cpf = ler_dados();
    println!("Digite o Endereço");
    cliente.endereco = ler_dados();
// Atribuindo um ID fixo para o cliente
}

pub fn listar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    if nao_tem_clientes(clientes)
    {
        return;
    }

    println!("{}", "-".to_string().repeat(40));
    for cliente in clientes.iter() {
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
    }

    println!("Digite ENTER para continuar...");
    ler_dados();
}

fn nao_tem_clientes(clientes: &[Cliente]) -> bool {

    if clientes.len() == 0 {
        println!("Não existem clientes cadastrados!");
        esperar(1);
        return true;
    }

    return false;
}

pub fn alterar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    if nao_tem_clientes(clientes)
    {
        return;
    }

    let id = captura_id();

    
    if let Some((indice, cliente)) = buscar_cliente_por_id(clientes, id)
    {
        println!("{}", "-".to_string().repeat(40));
        println!("Alterando o Cliente");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        digitar_dados_do_cliente(&mut clientes[indice]);
        limpar_tela();
        println!("Cliente alterado com sucesso!");
    }
    else
    {
        limpar_tela();
        println!("Cliente não encontrado!");

    }
    
    esperar(1);

}

pub fn excluir_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    if nao_tem_clientes(clientes)
    {
        return;
    }

    let id = captura_id();

    
    if let Some((indice, cliente)) = buscar_cliente_por_id(clientes, id)
    {
        println!("{}", "-".to_string().repeat(40));
        println!("Confirma a exclusão do cliente abaixo?");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        println!("S - Sim\nN - Não");

        let opcao: String = ler_dados();
        if opcao == "s"
        {
            clientes.remove(indice);
            limpar_tela();
            println!("Cliente excluído com sucesso!");
            esperar(1);
        }
    }
    else
    {
        limpar_tela();
        println!("Cliente não encontrado!");
        esperar(1);

    }  

}

fn buscar_cliente_por_id(clientes: &Vec<Cliente>, id: usize) -> Option<(usize, &Cliente)> {
    clientes.iter().enumerate().find(|(_, cliente)| cliente.id == id)
}

fn captura_id() -> usize {
    limpar_tela();
    println!("Digite o Id do Cliente");
    ler_dados_id()
}


fn mostrar_cliente(cliente: &Cliente)  {
    println!("\
        ID : {}\n\
        Nome : {}\n\
        CPF : {}\n\
        Endereço : {}
    ", cliente.id, cliente.nome, cliente.cpf, cliente.endereco);
}