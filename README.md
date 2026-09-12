# rust-pontue-scrapper

Script automatizado em Rust para monitorar a plataforma Pontue. Ele verifica a existência de novas redações corrigidas a cada 60 minutos e envia uma notificação automática por e-mail assim que um novo ID de redação é detectado. Caso ocorram erros durante a comunicação com o pontue, um email com o erro será enviado. Outros erros serão notificados apenas via logs.

## Pré-requisitos

* **Rust e Cargo** instalados (`rustup`).
* Conta do Google com **Verificação em Duas Etapas** ativada (para gerar a Senha de Aplicativo).

## Instalação

Clone o repo e faça a build:

```bash
git clone https://github.com/Guilherme-Costa-Dev/rust-pontue-scrapper.git
cd rust-pontue-scrapper
cargo build --release
```

## Configuração

Na mesma pasta do executável, crie um arquivo chamado `config.json`. Este arquivo é obrigatório para o envio dos e-mails e login na plataforma:

```json
{
	"nome": "Seu Nome Completo",
	"email": "seu_email@gmail.com",
	"google_app_key": "xxxx xxxx xxxx xxxx",
    "login": "seu login do pontue",
    "senha": "sua senha do pontue"
}
```
