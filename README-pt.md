# Utoipa Auto Detect Axum

[![English](https://img.shields.io/badge/language-English-blue.svg)](README.md)

Este repositório serve como um exemplo de como detectar automaticamente o framework Axum usando a biblioteca Utoipa em Rust.

## Introdução

Utoipa é uma biblioteca poderosa para gerar documentação OpenAPI para APIs web em Rust. Este projeto demonstra como integrar Utoipa com o framework Axum, permitindo a geração automática de documentação para endpoints definidos em Axum, sem a necessidade de configuração manual extensa.

## Requisitos

O projeto foi testado usando Rust 1.91.0 Stable e as seguintes dependências:

```toml
axum = "0.8.7"
tokio = { version = "1.48.0", features = ["full"] }
utoipa = "5.4.0"
utoipa-axum = "0.2.0"
utoipa-swagger-ui = { version = "9.0.2", features = ["axum"] }
```

## Disclaimer

Este repositório é apenas um exemplo simples para demonstrar a detecção automática do Axum com Utoipa. Ele não deve ser usado em produção sem antes revisar e adaptar o código conforme necessário para atender aos requisitos específicos do seu projeto. Além de não conter testes automatizados, integração com authenticação, ou outras funcionalidades essenciais para uma aplicação web robusta, caso queira incorporar essas funcionalidades, consulte a documentação oficial do Utoipa e do Axum e sinta-se à vontade para contribuir com melhorias.
