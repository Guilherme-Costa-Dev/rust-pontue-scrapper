# rust-pontue-scrapper

Script automatizado em Rust para monitorar a plataforma Pontue. Ele verifica a existência de novas redações corrigidas a cada 60 minutos e envia uma notificação automática por e-mail assim que um novo ID de redação é detectado.

## Pré-requisitos

* **Rust e Cargo** instalados (`rustup`).
* Conta do Google com **Verificação em Duas Etapas** ativada (para gerar a Senha de Aplicativo).

## Instalação

Clone o repo na raiz do sistema `~/` e instale globalmente com cargo:

```bash
git clone https://github.com/Guilherme-Costa-Dev/rust-pontue-scrapper.git
cd rust-pontue-scrapper
cargo install --path .
```

## Configuração

Na raiz do projeto crie um arquivo chamado `config.json`. Este arquivo é obrigatório para o envio dos e-mails e login na plataforma:

```json
{
	"nome": "Seu Nome Completo",
	"email": "seu_email@gmail.com",
	"google_app_key": "xxxx xxxx xxxx xxxx",
    "login": "seu login do pontue",
    "senha": "sua senha do pontue"
}
```
