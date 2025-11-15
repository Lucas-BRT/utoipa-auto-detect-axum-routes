# Utoipa Auto Detect Axum

[![Portuguese](https://img.shields.io/badge/language-Portuguese-blue.svg)](README-pt.md)

This repository serves as an example of how to automatically detect the Axum framework using the Utoipa library in Rust.

## Introduction

Utoipa is a powerful library for generating OpenAPI documentation for web APIs in Rust. This project demonstrates how to integrate Utoipa with the Axum framework, allowing for the automatic generation of documentation for endpoints defined in Axum, without the need for extensive manual configuration.

## Requirements

The project was tested using Rust 1.91.0 Stable and the following dependencies:

```toml
axum = "0.8.7"
tokio = { version = "1.48.0", features = ["full"] }
utoipa = "5.4.0"
utoipa-axum = "0.2.0"
utoipa-swagger-ui = { version = "9.0.2", features = ["axum"] }
```

## Disclaimer

This repository is just a simple example to demonstrate the automatic detection of Axum with Utoipa. It should not be used in production without first reviewing and adapting the code as necessary to meet the specific requirements of your project. In addition to not containing automated tests, integration with authentication, or other essential features for a robust web application, if you want to incorporate these functionalities, consult the official documentation of Utoipa and Axum and feel free to contribute with improvements.
