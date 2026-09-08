# rust-pontue-scrapper

Script automatizado em Rust para monitorar a plataforma Pontue. Ele verifica a existência de novas redações corrigidas a cada 10 minutos e envia uma notificação automática por e-mail assim que um novo ID de redação é detectado.

## Pré-requisitos

* **Rust e Cargo** instalados (`rustup`).
* **Google Chrome** ou **Chromium** instalado no sistema (exigido pela biblioteca `headless_chrome`).
* Conta do Google com **Verificação em Duas Etapas** ativada (para gerar a Senha de Aplicativo).

## Configuração

Na raiz do projeto crie um arquivo chamado `config.json`. Este arquivo é obrigatório para o envio dos e-mails e deve seguir a estrutura abaixo:

```json
{
	"nome": "Seu Nome Completo",
	"email": "seu_email@gmail.com",
	"google_app_key": "xxxx xxxx xxxx xxxx"
}
```

## Instalação

Clone o repo e instale globalmente com cargo:

```bash
git clone https://github.com/Guilherme-Costa-Dev/rust-pontue-scrapper.git
cd rust-pontue-scrapper
cargo install --path .
```
